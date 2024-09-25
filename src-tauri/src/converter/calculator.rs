use calc::Context;
use tracing::{debug, info, trace};

use super::converter::Converter;

pub struct CalculatorConverter;

impl Converter for CalculatorConverter {
    fn convert(&self, text: &str) -> anyhow::Result<String> {
        debug!("Evaluating expression: {}", text);
        let mut ctx = Context::<f64>::default();
        let result = match ctx.evaluate(text) {
            Ok(result) => {
                let formatted = format!("{} = {}", text, result.to_string());
                info!("Evaluation successful: {}", formatted);
                formatted
            },
            Err(e) => {
                debug!("Evaluation failed: {}", e);
                e.to_string()
            },
        };

        Ok(result)
    }

    fn name(&self) -> String {
        trace!("Getting converter name");
        "calculator".to_string()
    }
}
