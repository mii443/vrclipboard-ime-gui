use calc::Context;

use super::converter::Converter;

pub struct CalculatorConverter;

impl Converter for CalculatorConverter {
    fn convert(&self, text: &str) -> anyhow::Result<String> {
        let mut ctx = Context::<f64>::default();
        let result = match ctx.evaluate(text) {
            Ok(result) => format!("{} = {}", text, result.to_string()),
            Err(e) => e.to_string(),
        };

        Ok(result)
    }

    fn name(&self) -> String {
        "calculator".to_string()
    }
}
