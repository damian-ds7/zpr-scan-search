#[derive(Default, Debug)]
pub struct ScanSearchConfig {
    pub fs_scan: FsScanConfig,
}
/// Configuration for scanning the filesystem and collecting supported files.
#[derive(Debug)]
pub struct FsScanConfig {
    /// Follow symbolic links when walking the filesystem tree.
    pub follow_links: bool,

    /// Include hidden files and directories in the results.
    pub include_hidden: bool,
}

impl Default for FsScanConfig {
    fn default() -> Self {
        Self {
            follow_links: true,
            include_hidden: false,
        }
    }
}
