#[cfg(test)]
pub mod tests;
pub mod fastembed;

use std::sync::Arc;
use crate::{error::Result};
pub trait TextEncoder{
    fn encode(&self, text: &Vec<&str>) -> Result<Vec<Vec<f32>>>;
}