use super::TextEncoder;
use crate::error::Result;

struct MockEncoder {}

impl TextEncoder for MockEncoder {
    fn encode(&self, text: &[&str]) -> Result<Vec<Vec<f32>>> {
        let mut embeddings = Vec::new();
        for (i, _) in text.iter().enumerate() {
            let val = i as f32;
            embeddings.push(vec![val + 0.1, val + 0.2, val + 0.3]);
        }
        Ok(embeddings)
    }
}

#[test]
fn test_mock_encoder_trait() {
    let encoder = MockEncoder {};
    let text = vec!["This is a test sentence.", "This is another test sentence."];
    let result = encoder.encode(&text);
    let embeddings = result.unwrap();
    assert_eq!(embeddings.len(), 2);

    assert_eq!(embeddings[0], vec![0.1, 0.2, 0.3]);
    assert_eq!(embeddings[1], vec![1.1, 1.2, 1.3]);
}
