use windows::Win32::UI::Input::Ime::{FELANG_CMODE_HIRAGANAOUT, FELANG_CMODE_NOINVISIBLECHAR, FELANG_CMODE_PRECONV, FELANG_CMODE_ROMAN, FELANG_REQ_CONV};

use crate::felanguage::FElanguage;

use super::converter::Converter;

pub struct RomanToKanjiConverter;

impl Converter for RomanToKanjiConverter {
    fn convert(&self, text: &str) -> anyhow::Result<String> {
        let felanguage = FElanguage::new()?;
        felanguage.j_morph_result(text, FELANG_REQ_CONV, FELANG_CMODE_HIRAGANAOUT
            | FELANG_CMODE_ROMAN
            | FELANG_CMODE_NOINVISIBLECHAR
            | FELANG_CMODE_PRECONV)
    }

    fn name(&self) -> String {
        "roman_to_kanji".to_string()
    }
}
