use super::converter::Converter;
use tracing::{debug, trace};

pub struct NoneConverter;

impl Converter for NoneConverter {
    fn convert(&self, text: &str) -> anyhow::Result<String> {
        debug!("Converting with NoneConverter: {}", text);
        Ok(text.to_string())
    }

    fn name(&self) -> String {
        trace!("Getting converter name");
        "none".to_string()
    }
}
