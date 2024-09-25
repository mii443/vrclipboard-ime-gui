use anyhow::Result;
use crate::{converter::{converter::Converter, hiragana::HiraganaConverter, roman_to_kanji::RomanToKanjiConverter}, tsf::{search_candidate_provider::SearchCandidateProvider, set_thread_local_input_settings}};

pub struct TsfConversion {
    pub conversion_history: Vec<String>,
    pub clipboard_history: Vec<String>,
    pub now_reconvertion: bool,
    pub target_text: String,
    pub search_candidate_provider: SearchCandidateProvider,
    pub reconversion_candidates: Option<Vec<String>>,
    pub reconversion_index: Option<i32>,
    pub reconversion_prefix: Option<String>,
}

impl TsfConversion {
    pub fn new() -> Self {
        set_thread_local_input_settings(true).unwrap();

        Self {
            conversion_history: Vec::new(),
            clipboard_history: Vec::new(),
            now_reconvertion: false,
            target_text: String::new(),
            search_candidate_provider: SearchCandidateProvider::create().unwrap(),
            reconversion_candidates: None,
            reconversion_index: None,
            reconversion_prefix: None,
        }
    }

    fn reset_conversion_state(&mut self) {
        self.now_reconvertion = false;
        self.reconversion_prefix = None;
        self.reconversion_index = None;
        self.reconversion_candidates = None;
    }

    fn convert_roman_to_kanji(&mut self, text: &str) -> Result<String> {
        let o_minus_1 = self.conversion_history.get(if self.conversion_history.len() > 0 { self.conversion_history.len() - 1 } else { 0 }).unwrap_or(&("".to_string())).clone();
        let mut first_diff_position = o_minus_1.chars().zip(text.chars()).position(|(a, b)| a != b);

        if o_minus_1 != text && first_diff_position.is_none() {
            first_diff_position = Some(o_minus_1.chars().count());
        }
        let diff = text.chars().skip(first_diff_position.unwrap_or(0)).collect::<String>();

        let roman_to_kanji_converter = RomanToKanjiConverter;
        let converted = roman_to_kanji_converter.convert(&diff)?;
        self.conversion_history.push(o_minus_1.chars().zip(text.chars()).take_while(|(a, b)| a == b).map(|(a, _)| a).collect::<String>() + &converted);
        self.clipboard_history.push(text.to_string());
        return Ok(self.conversion_history.last().unwrap().clone());
    }

    fn convert_tsf(&mut self, text: &str) -> Result<String> {
        self.now_reconvertion = true;
        let mut diff_hiragana = String::new();
        let mut diff = String::new();
        if self.reconversion_prefix.is_none() {
            let o_minus_2 = self.conversion_history.get(if self.conversion_history.len() > 1 { self.conversion_history.len() - 2 } else { 0 }).unwrap_or(&("".to_string())).clone();
            let i_minus_1 = self.clipboard_history.get(if self.clipboard_history.len() > 0 { self.clipboard_history.len() - 1 } else { 0 }).unwrap_or(&("".to_string())).clone();
            println!("o,i: {}, {}", o_minus_2, i_minus_1);
            let mut first_diff_position = i_minus_1.chars().zip(o_minus_2.chars()).position(|(a, b)| a != b);
            println!("diff_pos: {:?}", first_diff_position);
            if o_minus_2 != i_minus_1 && first_diff_position.is_none() {
                first_diff_position = Some(o_minus_2.chars().count());
            }
            diff = i_minus_1.chars().skip(first_diff_position.unwrap_or(0)).collect::<String>();
            println!("diff: {}", diff);
            diff_hiragana = HiraganaConverter.convert(&diff)?;
            let prefix = i_minus_1.chars().zip(o_minus_2.chars()).take_while(|(a, b)| a == b).map(|(a, _)| a).collect::<String>();
            self.reconversion_prefix = Some(prefix.clone());
        }
        println!("diff_hiragana: {}", diff_hiragana);

        let candidates = self.reconversion_candidates.get_or_insert_with(|| {
            let mut candidates = self.search_candidate_provider.get_candidates(&diff_hiragana, 10).unwrap_or_default();
            if candidates.is_empty() {
                candidates.push(diff_hiragana.clone());
                let roman_to_kanji_converter = RomanToKanjiConverter;
                let roman_to_kanji = roman_to_kanji_converter.convert(&diff_hiragana).unwrap();
                candidates.push(roman_to_kanji);
            }
            candidates.insert(0, diff.to_string());
            candidates
        });

        let index = self.reconversion_index.get_or_insert(-1);

        if *index + 1 < candidates.len() as i32 {
            *index += 1;
        } else {
            *index = 0;
        }

        if self.reconversion_candidates.is_some() {
            println!("Candidates: {:?}", self.reconversion_candidates.as_ref().unwrap());
        }

        self.conversion_history.push(self.reconversion_prefix.clone().unwrap() + &self.reconversion_candidates.as_ref().unwrap()[self.reconversion_index.unwrap() as usize].clone());
        self.clipboard_history.push(text.to_string());

        while self.conversion_history.len() > 3 {
            self.conversion_history.remove(0);
        }
        while self.clipboard_history.len() > 3 {
            self.clipboard_history.remove(0);
        }

        return Ok(self.conversion_history.last().unwrap().clone());
    }

    pub fn convert(&mut self, text: &str) -> Result<String> {
        println!();
        println!("History: {:?}, {:?}", self.conversion_history, self.clipboard_history);
        println!("{} == {}", text, self.conversion_history.last().unwrap_or(&("".to_string())).clone());
        let same_as_last_conversion = text.to_string() == self.conversion_history.last().unwrap_or(&("".to_string())).clone();

        self.target_text = text.to_string();

        if !same_as_last_conversion && self.now_reconvertion {
            self.reset_conversion_state();
        }

        if !self.now_reconvertion && !same_as_last_conversion {
            println!("Convert using roman_to_kanji");
            return self.convert_roman_to_kanji(text);
        }

        if same_as_last_conversion || self.now_reconvertion {
            println!("Convert using TSF");
            return self.convert_tsf(text);
        }

        Err(anyhow::anyhow!("Failed to convert"))
    }
}
