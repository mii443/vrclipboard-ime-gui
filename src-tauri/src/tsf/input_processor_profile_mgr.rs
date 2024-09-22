use anyhow::Result;
use windows::Win32::{
    System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
    UI::{Input::KeyboardAndMouse::HKL, TextServices::{CLSID_TF_InputProcessorProfiles, ITfInputProcessorProfileMgr, GUID_TFCAT_TIP_KEYBOARD, TF_INPUTPROCESSORPROFILE, TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE, TF_PROFILETYPE_INPUTPROCESSOR}},
};

pub struct InputProcessorProfileMgr {
    input_processor_profile_mgr: ITfInputProcessorProfileMgr,
}

impl InputProcessorProfileMgr {
    pub fn new() -> Result<Self> {
        let input_processor_profile_mgr = unsafe { CoCreateInstance(&CLSID_TF_InputProcessorProfiles, None, CLSCTX_INPROC_SERVER)? };
        Ok(InputProcessorProfileMgr { input_processor_profile_mgr })
    }

    pub fn get_active_profile(&self) -> Result<TF_INPUTPROCESSORPROFILE> {
        let keyboard_guid = GUID_TFCAT_TIP_KEYBOARD;
        let mut profile = TF_INPUTPROCESSORPROFILE::default();

        unsafe { self.input_processor_profile_mgr.GetActiveProfile(&keyboard_guid, &mut profile)? };

        Ok(profile)
    }
    
    pub fn activate_profile(&self, profile: &TF_INPUTPROCESSORPROFILE) -> Result<()> {
        unsafe { self.input_processor_profile_mgr.ActivateProfile(TF_PROFILETYPE_INPUTPROCESSOR, profile.langid, &profile.clsid, &profile.guidProfile, HKL::default(), TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE)? };
        Ok(())
    }
}
