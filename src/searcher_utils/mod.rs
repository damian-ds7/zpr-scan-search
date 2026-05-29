use std::ops::Range;

/// Collects byte ranges of fragments (words or lines) from `text` within the window
/// `[fetch_from, fetch_to]`, using the provided `splitter` iterator.
///
/// The splitter can be any iterator yielding `&str` slices pointing into `text`,
/// such as `split_whitespace()` or `lines()`. Pointer arithmetic is used to derive
/// byte offsets, so the slices must originate from `text`.
///
/// `item_pos` and `byte_pos` act as a cursor, the splitter should be created from
/// `text[*byte_pos..]` so it resumes from the previous call's position.
///
/// Returns `None` if `fetch_from` is beyond the end of the text.
pub fn collect_context_fragments<'a>(
    text: &'a str,
    mut splitter: impl Iterator<Item = &'a str>,
    fetch_from: usize,
    fetch_to: usize,
    item_pos: &mut usize,
    byte_pos: &mut usize,
) -> Option<Vec<Range<usize>>> {
    // The base address of `text` in memory, used to convert raw pointers back
    // into byte offsets relative to the start of the string.
    let base = text.as_ptr() as usize;

    // Skip ahead to `fetch_from` by advancing past items already behind the cursor.
    // `item_pos` tracks how far we've consumed, so only the delta needs to be skipped.
    let skip = fetch_from - *item_pos;
    let first_item = splitter.nth(skip)?;

    // Derive the byte offset of the first fetched item by subtracting the base address.
    let first_item_byte_pos = first_item.as_ptr() as usize - base;

    // Advance both cursors to reflect the item we just landed on.
    *item_pos = fetch_from;
    *byte_pos = first_item_byte_pos;

    let mut items = Vec::with_capacity(fetch_to - fetch_from + 1);
    items.push(first_item_byte_pos..first_item_byte_pos + first_item.len());

    // Collect the remaining items in [fetch_from+1, fetch_to], computing each
    // one's byte range the same way as the first.
    for item in splitter.take(fetch_to - fetch_from) {
        let start = item.as_ptr() as usize - base;
        items.push(start..start + item.len());
    }

    Some(items)
}

/// Builds before/matched/after byte ranges from a slice of fragment ranges.
///
/// `mid` is the index of the first matched fragment within `words`, and `query_len`
/// is the number of consecutive fragments the match spans (1 for a single word,
/// N for a phrase).
///
/// - `before` spans from the first fragment up to (but not including) the match.
///   Empty range at the start of `matched` if there are no fragments before it.
/// - `matched` spans from `words[mid]` to `words[mid + query_len - 1]`.
/// - `after` spans from the first fragment after the match to the last fragment.
///   Empty range at the end of `matched` if there are no fragments after it.
///
/// The returned ranges are byte offsets into the original text.
pub fn build_context_ranges(
    words: &[Range<usize>],
    mid: usize,
    query_len: usize,
) -> (Range<usize>, Range<usize>, Range<usize>) {
    let before = if mid > 0 {
        words[0].start..words[mid - 1].end
    } else {
        let s = words[mid].start;
        s..s
    };

    let matched = words[mid].start..words[mid + query_len - 1].end;

    let after = if mid + query_len < words.len() {
        words[mid + query_len].start..words[words.len() - 1].end
    } else {
        let e = words[mid + query_len - 1].end;
        e..e
    };

    (before, matched, after)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words_from_ranges<'a>(text: &'a str, ranges: &[Range<usize>]) -> Vec<&'a str> {
        ranges.iter().map(|r| &text[r.clone()]).collect()
    }

    fn str_from_range(text: &str, range: Range<usize>) -> &str {
        &text[range]
    }

    #[test]
    fn test_collect_basic() {
        let text = "one two three four five";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            1,
            3,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        assert_eq!(
            words_from_ranges(text, &words),
            vec!["two", "three", "four"]
        );
    }

    #[test]
    fn test_collect_from_start() {
        let text = "one two three four five";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            2,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        assert_eq!(words_from_ranges(text, &words), vec!["one", "two", "three"]);
    }

    #[test]
    fn test_collect_to_end() {
        let text = "one two three four five";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            3,
            10,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        assert_eq!(words_from_ranges(text, &words), vec!["four", "five"]);
    }

    #[test]
    fn test_collect_returns_none_when_out_of_range() {
        let text = "one two three";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let result = collect_context_fragments(
            text,
            text.split_whitespace(),
            10,
            12,
            &mut word_pos,
            &mut byte_pos,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_collect_saves_cursor() {
        let text = "one two three four five six seven";
        let mut word_pos = 0;
        let mut byte_pos = 0;

        collect_context_fragments(
            text,
            text[byte_pos..].split_whitespace(),
            1,
            2,
            &mut word_pos,
            &mut byte_pos,
        );
        assert_eq!(word_pos, 1);
        assert_eq!(&text[byte_pos..].split_whitespace().next().unwrap(), &"two");

        let words = collect_context_fragments(
            text,
            text[byte_pos..].split_whitespace(),
            4,
            5,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        assert_eq!(words_from_ranges(text, &words), vec!["five", "six"]);
    }

    #[test]
    fn test_collect_with_lines() {
        let text = "line one\nline two\nline three\nline four";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words =
            collect_context_fragments(text, text.lines(), 1, 2, &mut word_pos, &mut byte_pos)
                .unwrap();
        assert_eq!(
            words_from_ranges(text, &words),
            vec!["line two", "line three"]
        );
    }

    #[test]
    fn test_build_middle() {
        let text = "one two three four five";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            4,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 2, 1);
        assert_eq!(str_from_range(text, before), "one two");
        assert_eq!(str_from_range(text, matched), "three");
        assert_eq!(str_from_range(text, after), "four five");
    }

    #[test]
    fn test_build_no_before() {
        let text = "one two three";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            2,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 0, 1);
        assert_eq!(str_from_range(text, before), "");
        assert_eq!(str_from_range(text, matched), "one");
        assert_eq!(str_from_range(text, after), "two three");
    }

    #[test]
    fn test_build_no_after() {
        let text = "one two three";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            2,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 2, 1);
        assert_eq!(str_from_range(text, before), "one two");
        assert_eq!(str_from_range(text, matched), "three");
        assert_eq!(str_from_range(text, after), "");
    }

    #[test]
    fn test_build_single_word() {
        let text = "hello";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            0,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 0, 1);
        assert_eq!(str_from_range(text, before), "");
        assert_eq!(str_from_range(text, matched), "hello");
        assert_eq!(str_from_range(text, after), "");
    }

    #[test]
    fn test_build_phrase_middle() {
        let text = "one two three four five six seven";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            6,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 2, 3); // "three four five" is match
        assert_eq!(str_from_range(text, before), "one two");
        assert_eq!(str_from_range(text, matched), "three four five");
        assert_eq!(str_from_range(text, after), "six seven");
    }

    #[test]
    fn test_build_phrase_no_before() {
        let text = "one two three four five";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            4,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 0, 2); // "one two" is match
        assert_eq!(str_from_range(text, before), "");
        assert_eq!(str_from_range(text, matched), "one two");
        assert_eq!(str_from_range(text, after), "three four five");
    }

    #[test]
    fn test_build_phrase_no_after() {
        let text = "one two three four five";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            4,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 3, 2); // "four five" is match
        assert_eq!(str_from_range(text, before), "one two three");
        assert_eq!(str_from_range(text, matched), "four five");
        assert_eq!(str_from_range(text, after), "");
    }

    #[test]
    fn test_build_phrase_spans_entire_text() {
        let text = "one two three";
        let mut word_pos = 0;
        let mut byte_pos = 0;
        let words = collect_context_fragments(
            text,
            text.split_whitespace(),
            0,
            2,
            &mut word_pos,
            &mut byte_pos,
        )
        .unwrap();
        let (before, matched, after) = build_context_ranges(&words, 0, 3); // entire text is match
        assert_eq!(str_from_range(text, before), "");
        assert_eq!(str_from_range(text, matched), "one two three");
        assert_eq!(str_from_range(text, after), "");
    }
}
