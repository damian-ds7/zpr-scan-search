mod filetype;
mod scanner;
#[cfg(test)]
mod tests;

pub use filetype::FileType;
pub use scanner::get_fts_from_paths;
