use anyhow::Result;
use tracing::{debug, trace};

use super::{calculator::CalculatorConverter, hiragana::HiraganaConverter, katakana::KatakanaConverter, none_converter::NoneConverter, roman_to_kanji::RomanToKanjiConverter};

pub trait Converter {
    fn convert(&self, text: &str) -> Result<String>;
    fn name(&self) -> String;
}

pub fn get_custom_converter(prefix: char) -> Option<Box<dyn Converter>> {
    debug!("Getting custom converter for prefix: {}", prefix);
    let converter = match prefix {
        'r' => Some(Box::new(RomanToKanjiConverter) as Box<dyn Converter>),
        'h' => Some(Box::new(HiraganaConverter) as Box<dyn Converter>),
        'c' => Some(Box::new(CalculatorConverter) as Box<dyn Converter>),
        'n' => Some(Box::new(NoneConverter) as Box<dyn Converter>),
        'k' => Some(Box::new(KatakanaConverter) as Box<dyn Converter>),
        _ => None,
    };
    match &converter {
        Some(c) => debug!("Custom converter found: {}", c.name()),
        None => trace!("No custom converter found for prefix: {}", prefix),
    }
    converter
}
