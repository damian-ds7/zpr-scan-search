#[cfg(test)]
pub mod tests;
mod fastembed;

use crate::{error::Result};
pub trait TextEncoder{
    fn encode(&self, text: Vec<&str>) -> Result<Vec<Vec<f32>>>;
}