#[cfg(test)]
pub mod tests;
mod rust_bert;

use crate::{error::Result};
pub trait TextEncoder{
    fn encode(&self, text: &str) -> Result<Vec<Vec<f32>>>;
}