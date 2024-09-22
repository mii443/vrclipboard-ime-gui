use anyhow::Result;
use windows::{
    core::Interface,
    Win32::UI::TextServices::{ITfFnSearchCandidateProvider, ITfFunctionProvider},
};

use super::search_candidate_provider::SearchCandidateProvider;

pub struct FunctionProvider {
    function_provider: ITfFunctionProvider,
}

impl FunctionProvider {
    pub fn new(function_provider: ITfFunctionProvider) -> Self {
        Self { function_provider }
    }

    pub fn get_search_candidate_provider(&self) -> Result<SearchCandidateProvider> {
        let zeroed_guid = windows_core::GUID::zeroed();
        let search_candidate_provider = unsafe { self.function_provider.GetFunction(&zeroed_guid, &ITfFnSearchCandidateProvider::IID)? };
        Ok(SearchCandidateProvider::new(search_candidate_provider.cast()?))
    }
}
