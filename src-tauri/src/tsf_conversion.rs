use anyhow::Result;
use crate::{converter::{converter::Converter, hiragana::HiraganaConverter, roman_to_kanji::RomanToKanjiConverter}, tsf::{search_candidate_provider::SearchCandidateProvider, set_thread_local_input_settings}};

pub struct TsfConversion {
    pub conversion_history: Vec<String>,
    pub clipboard_history: Vec<String>,
    pub now_reconvertion: bool,
    pub target_text: String,
    pub search_candidate_provider: SearchCandidateProvider,
    pub reconversion_candidates: Option<Vec<String>>,
    pub reconversion_index: Option<usize>,
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

    pub fn convert(&mut self, text: &str) -> Result<String> {
        println!();
        println!("History: {:?}, {:?}", self.conversion_history, self.clipboard_history);
        println!("{} == {}", text, self.conversion_history.last().unwrap_or(&("".to_string())).clone());
        let same_as_last_conversion = text.to_string() == self.conversion_history.last().unwrap_or(&("".to_string())).clone();

        if !same_as_last_conversion && self.now_reconvertion {
            self.now_reconvertion = false;
            self.reconversion_prefix = None;
            self.reconversion_index = None;
            self.reconversion_candidates = None;
        }

        if !self.now_reconvertion && !same_as_last_conversion {
            println!("Convert using roman_to_kanji");
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

        if same_as_last_conversion || self.now_reconvertion {
            println!("Convert using TSF");
            self.now_reconvertion = true;
            let mut diff_hiragana = String::new();
            if self.reconversion_prefix.is_none() {
                let o_minus_2 = self.conversion_history.get(if self.conversion_history.len() > 1 { self.conversion_history.len() - 2 } else { 0 }).unwrap_or(&("".to_string())).clone();
                let i_minus_1 = self.clipboard_history.get(if self.clipboard_history.len() > 0 { self.clipboard_history.len() - 1 } else { 0 }).unwrap_or(&("".to_string())).clone();
                println!("o,i: {}, {}", o_minus_2, i_minus_1);
                let mut first_diff_position = i_minus_1.chars().zip(o_minus_2.chars()).position(|(a, b)| a != b);
                println!("diff_pos: {:?}", first_diff_position);
                if o_minus_2 != i_minus_1 && first_diff_position.is_none() {
                    first_diff_position = Some(o_minus_2.chars().count());
                }
                let diff = i_minus_1.chars().skip(first_diff_position.unwrap_or(0)).collect::<String>();
                println!("diff: {}", diff);
                diff_hiragana = HiraganaConverter.convert(&diff)?;
                let prefix = i_minus_1.chars().zip(o_minus_2.chars()).take_while(|(a, b)| a == b).map(|(a, _)| a).collect::<String>();
                self.reconversion_prefix = Some(prefix.clone());
            }
            println!("diff_hiragana: {}", diff_hiragana);

            if self.reconversion_index.is_none() {
                self.reconversion_candidates = Some(self.search_candidate_provider.get_candidates(&diff_hiragana, 10)?);
                self.reconversion_index = Some(0);
                if self.reconversion_prefix.clone().unwrap() + &self.reconversion_candidates.as_ref().unwrap()[self.reconversion_index.unwrap()].clone() == text {
                    self.reconversion_index = Some(self.reconversion_index.unwrap() + 1);
                }
            } else if self.reconversion_index.unwrap() + 1 < self.reconversion_candidates.as_ref().unwrap().len() {
                self.reconversion_index = Some(self.reconversion_index.unwrap() + 1);
            } else {
                self.reconversion_index = Some(0);
            }

            if self.reconversion_candidates.is_some() {
                println!("Candidates: {:?}", self.reconversion_candidates.as_ref().unwrap());
            }

            self.conversion_history.push(self.reconversion_prefix.clone().unwrap() + &self.reconversion_candidates.as_ref().unwrap()[self.reconversion_index.unwrap()].clone());
            self.clipboard_history.push(text.to_string());

            while self.conversion_history.len() > 3 {
                self.conversion_history.remove(0);
            }
            while self.clipboard_history.len() > 3 {
                self.clipboard_history.remove(0);
            }

            return Ok(self.conversion_history.last().unwrap().clone());
        }

        Err(anyhow::anyhow!("Failed to convert"))
    }
}
