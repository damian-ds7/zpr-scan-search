use std::ops::Range;

pub fn collect_context_fragments<'a>(
    text: &'a str,
    splitter: impl Iterator<Item = &'a str>,
    fetch_from: usize,
    fetch_to: usize,
    word_pos: &mut usize,
    byte_pos: &mut usize,
) -> Option<Vec<Range<usize>>> {
    let base = text.as_ptr() as usize;
    let mut words = Vec::new();
    let mut word_idx = *word_pos;

    for word in splitter {
        let start = word.as_ptr() as usize - base;

        if word_idx == fetch_from {
            *word_pos = word_idx;
            *byte_pos = start;
        }

        if word_idx >= fetch_from {
            words.push(start..start + word.len());
        }

        word_idx += 1;

        if word_idx > fetch_to {
            break;
        }
    }

    if words.is_empty() { None } else { Some(words) }
}

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
