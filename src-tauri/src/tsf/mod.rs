use anyhow::Result;
use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_SETTHREADLOCALINPUTSETTINGS, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS};

pub mod input_processor_profile_mgr;
pub mod function_provider;
pub mod search_candidate_provider;
pub mod thread_mgr;

pub fn set_thread_local_input_settings(thread_local_input_settings: bool) -> Result<()> {
    let mut result = thread_local_input_settings;
    unsafe { SystemParametersInfoW(SPI_SETTHREADLOCALINPUTSETTINGS, 0, Some(&mut result as *mut _ as *const _ as *mut _), SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0))? };

    Ok(())
}
