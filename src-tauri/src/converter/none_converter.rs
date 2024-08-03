use super::converter::Converter;

pub struct NoneConverter;

impl Converter for NoneConverter {
    fn convert(&self, text: &str) -> anyhow::Result<String> {
        Ok(text.to_string())
    }
    
    fn name(&self) -> String {
        "none".to_string()
    }
}
