use crate::config::Config;

/// A section of an HTML webpage
pub trait Section {
    /// Label used to reference the section in the .toml configuration
    type Label;

    /// Create section from a .toml configuration
    fn new(config: &Config) -> Self;
}
