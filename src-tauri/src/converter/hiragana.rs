use windows::Win32::UI::Input::Ime::{FELANG_CMODE_HIRAGANAOUT, FELANG_CMODE_NOINVISIBLECHAR, FELANG_CMODE_PRECONV, FELANG_REQ_REV};

use crate::felanguage::FElanguage;

use super::converter::Converter;

#[derive(Clone)]
pub struct HiraganaConverter;

impl Converter for HiraganaConverter {
    fn convert(&self, text: &str) -> anyhow::Result<String> {
        let felanguage = FElanguage::new()?;
        felanguage.j_morph_result(text, FELANG_REQ_REV, FELANG_CMODE_HIRAGANAOUT | FELANG_CMODE_PRECONV | FELANG_CMODE_NOINVISIBLECHAR)
    }

    fn name(&self) -> String {
        "hiragana".to_string()
    }
}
