use anyhow::Result;
use crate::tsf::{search_candidate_provider::SearchCandidateProvider, set_thread_local_input_settings};

pub struct TsfConversion {
    pub conversion_history: Vec<String>,
    pub clipboard_history: Vec<String>,
    pub now_reconvertion: bool,
    pub target_text: String,
    pub search_candidate_provider: SearchCandidateProvider,
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
        }
    }

    pub fn convert(&self, text: &str) -> Result<String> {
        Ok(self.search_candidate_provider.get_candidates(text, 10)?[0].clone())
    }
}
