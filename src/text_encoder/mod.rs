pub mod fastembed;
#[cfg(test)]
pub mod tests;

use crate::error::Result;


pub trait TextEncoder {
    fn encode(&self, text: &[&str]) -> Result<Vec<Vec<f32>>>;
}
