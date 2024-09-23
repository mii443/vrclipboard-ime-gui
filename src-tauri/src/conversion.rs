use crate::{config::Config, converter::converter::{get_custom_converter, Converter}, STATE};
use anyhow::Result;

pub struct ConversionBlock {
    pub text: String,
    pub converter: Box<dyn Converter>,
}

pub struct Conversion;

impl Conversion {
    pub fn new() -> Self {
        Self {}
    }

    pub fn convert_text(&self, text: &str) -> Result<String> {
        println!("Processing text: {}", text);
        let blocks = self.split_text(text)?;
        self.convert_blocks(blocks)
    }

    pub fn convert_blocks(&self, blocks: Vec<ConversionBlock>) -> Result<String> {
        let mut result = String::new();
        for block in blocks {
            let converted = self.convert_block(&block)?;
            println!("  {}: {} -> {}", block.converter.name(), block.text, converted);
            result.push_str(&converted);
        }
        println!("    {}", result);
        Ok(result)
    }

    pub fn convert_block(&self, block: &ConversionBlock) -> Result<String> {
        if block.text == "" {
            return Ok(String::default());
        }
        block.converter.convert(&block.text)
    }

    pub fn split_text(&self, text: &str) -> Result<Vec<ConversionBlock>> {
        let mut text = text.to_string();
        let mut blocks = Vec::new();
        let mut current_converter = 'r';

        let config = self.get_config();

        if text.starts_with(&config.command) {
            text = text.split_off(1);
            if text.len() != 0 {
                current_converter = text.chars().next().unwrap_or('n');
                text = text.split_off(1);
            }
        }

        for (i, command_splitted) in text.split(&config.command).enumerate() {
            let mut command_splitted = command_splitted.to_string();
            if i != 0 {
                if command_splitted.len() != 0 {
                    current_converter = command_splitted.chars().next().unwrap_or('n');
                    command_splitted = command_splitted.split_off(1);
                }
            }

            for splitted in command_splitted.split(&config.split) {
                blocks.push(ConversionBlock {
                    text: splitted.to_string(),
                    converter: get_custom_converter(current_converter).unwrap_or(get_custom_converter('n').unwrap())
                });
            }
        }

        Ok(blocks)
    }

    pub fn get_config(&self) -> Config {
        STATE.lock().unwrap().clone()
    }
}
