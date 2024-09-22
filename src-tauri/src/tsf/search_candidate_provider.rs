use anyhow::Result;
use windows::Win32::UI::TextServices::{ITfFnSearchCandidateProvider, TF_TMAE_NOACTIVATEKEYBOARDLAYOUT};

use super::{function_provider::FunctionProvider, input_processor_profile_mgr::InputProcessorProfileMgr, thread_mgr::ThreadMgr};

pub struct SearchCandidateProvider {
    search_candidate_provider: ITfFnSearchCandidateProvider,
}

impl SearchCandidateProvider {
    pub fn new(search_candidate_provider: ITfFnSearchCandidateProvider) -> Self {
        Self { search_candidate_provider }
    }

    pub fn create() -> Result<Self> {
        let profile_mgr = InputProcessorProfileMgr::new()?;
        let profile = profile_mgr.get_active_profile()?;

        let thread_mgr = ThreadMgr::new()?;
        let _client_id = thread_mgr.activate_ex(TF_TMAE_NOACTIVATEKEYBOARDLAYOUT)?;

        let function_provider = thread_mgr.get_function_provider(&profile.clsid)?;

        let search_candidate_provider = FunctionProvider::new(function_provider).get_search_candidate_provider()?;  

        Ok(search_candidate_provider)
    }

    pub fn get_candidates(&self, input: &str, max: usize) -> Result<Vec<String>> {
        let input_utf16: Vec<u16> = input.encode_utf16().chain(Some(0)).collect();
        let input_bstr = windows_core::BSTR::from_wide(&input_utf16)?;

        let input_utf16: Vec<u16> = "".encode_utf16().chain(Some(0)).collect();
        let input_bstr_empty = windows_core::BSTR::from_wide(&input_utf16)?;
        
        let candidates = unsafe { self.search_candidate_provider.GetSearchCandidates(&input_bstr, &input_bstr_empty)? };
        let candidates_enum = unsafe { candidates.EnumCandidates()? };

        let mut candidates = vec![None; max];
        let mut candidates_count = 0;
        unsafe { candidates_enum.Next(&mut candidates, &mut candidates_count)? };

        candidates.resize(candidates_count as usize, None);

        let candidates: Vec<String> = candidates.iter().map(|candidate| unsafe { candidate.as_ref().unwrap().GetString().unwrap().to_string() }).collect();
        Ok(candidates)
    }
}
