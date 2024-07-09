type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

use clap::ValueEnum;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum OutputFormatter {
    Json,
    Yaml,
}

#[allow(dead_code)]
pub trait GenerateOutputFormat {
    fn try_format_single<S>(&self, data: S) -> Result<String>
    where
        S: Serialize;

    fn try_format_multiple<S>(&self, data: &[S]) -> Result<String>
    where
        S: Serialize;
}

impl GenerateOutputFormat for OutputFormatter {
    fn try_format_single<S>(&self, data: S) -> Result<String>
    where
        S: Serialize,
    {
        match self {
            OutputFormatter::Json => Ok(serde_json::to_string_pretty(&data)?),
            OutputFormatter::Yaml => Ok(serde_yaml::to_string(&data)?),
        }
    }

    fn try_format_multiple<S>(&self, data: &[S]) -> Result<String>
    where
        S: Serialize,
    {
        match self {
            OutputFormatter::Json => Ok(serde_json::to_string_pretty(&data)?),
            OutputFormatter::Yaml => Ok(serde_yaml::to_string(&data)?),
        }
    }
}
