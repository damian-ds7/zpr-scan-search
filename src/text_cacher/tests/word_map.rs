use crate::text_cacher::WordMap;

#[test]
fn test_word_map_from_str() {
    let test_cases = [
        (
            "Ala ma kota",
            vec![("Ala", vec![0]), ("ma", vec![1]), ("kota", vec![2])],
        ),
        ("kota kota", vec![("kota", vec![0, 1])]),
        ("", vec![]),
    ];

    for (text, expected) in test_cases {
        let map = WordMap::from(text);
        assert_eq!(map.len(), expected.len());
        for (word, indices) in expected {
            assert_eq!(map.get(word).unwrap(), &indices);
        }
    }
}
