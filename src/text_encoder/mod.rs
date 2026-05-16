pub mod fastembed;
#[cfg(test)]
pub mod tests;

use crate::error::Result;
use std::sync::Arc;
pub trait TextEncoder {
    fn encode(&self, text: &Vec<&str>) -> Result<Vec<Vec<f32>>>;
}
