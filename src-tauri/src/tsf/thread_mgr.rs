use anyhow::Result;
use windows::{
    core::Interface,
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER},
        UI::{Input::KeyboardAndMouse::HKL, TextServices::{CLSID_TF_InputProcessorProfiles, CLSID_TF_ThreadMgr, ITfFnSearchCandidateProvider, ITfFunctionProvider, ITfInputProcessorProfileMgr, ITfThreadMgr2, GUID_TFCAT_TIP_KEYBOARD, TF_INPUTPROCESSORPROFILE, TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE, TF_PROFILETYPE_INPUTPROCESSOR, TF_TMAE_NOACTIVATEKEYBOARDLAYOUT}, WindowsAndMessaging::{SystemParametersInfoW, SPI_SETTHREADLOCALINPUTSETTINGS, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS}},
    },
};

pub struct ThreadMgr {
    thread_mgr: ITfThreadMgr2,
}

impl ThreadMgr {
    pub fn new() -> Result<Self> {
        let thread_mgr = unsafe { CoCreateInstance(&CLSID_TF_ThreadMgr, None, CLSCTX_INPROC_SERVER)? };
        Ok(ThreadMgr { thread_mgr })
    }

    pub fn activate_ex(&self, flags: u32) -> Result<u32> {
        let mut client_id = 0;
        unsafe { self.thread_mgr.ActivateEx(&mut client_id as *mut _ as *const _ as *mut _, flags)? };
        Ok(client_id)
    }

    pub fn get_function_provider(&self, clsid: &windows_core::GUID) -> Result<ITfFunctionProvider> {
        Ok(unsafe { self.thread_mgr.GetFunctionProvider(clsid)? })
    }
}
