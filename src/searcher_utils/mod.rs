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
) -> (Range<usize>, Range<usize>, Range<usize>) {
    let before = if mid > 0 {
        words[0].start..words[mid - 1].end
    } else {
        let s = words[mid].start;
        s..s
    };

    let matched = words[mid].clone();

    let after = if mid + 1 < words.len() {
        words[mid + 1].start..words[words.len() - 1].end
    } else {
        let e = words[mid].end;
        e..e
    };

    (before, matched, after)
}
