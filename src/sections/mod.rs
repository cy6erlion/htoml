use crate::config::Config;

/// A section of an HTML webpage
pub trait Section {
    /// Label used to reference the section in the .toml configuration
    type L;

    /// Indicates which style should be used
    type S;

    /// Create section from a .toml configuration
    fn new(config: &Config) -> Self;

    /// Export section to HTML string
    fn export() -> String;
}
