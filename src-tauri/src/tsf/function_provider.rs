use anyhow::Result;
use windows::{
    core::Interface,
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
        UI::{Input::KeyboardAndMouse::HKL, TextServices::{CLSID_TF_InputProcessorProfiles, CLSID_TF_ThreadMgr, ITfFnSearchCandidateProvider, ITfFunctionProvider, ITfInputProcessorProfileMgr, ITfThreadMgr2, GUID_TFCAT_TIP_KEYBOARD, TF_INPUTPROCESSORPROFILE, TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE, TF_PROFILETYPE_INPUTPROCESSOR, TF_TMAE_NOACTIVATEKEYBOARDLAYOUT}, WindowsAndMessaging::{SystemParametersInfoW, SPI_SETTHREADLOCALINPUTSETTINGS, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS}},
    },
};

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
