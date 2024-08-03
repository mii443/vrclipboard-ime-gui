use anyhow::Result;
use windows::Win32::System::Com::{CoInitialize, CoUninitialize};

pub struct Com;

impl Drop for Com {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

impl Com {
    pub fn new() -> Result<Self> {
        unsafe { CoInitialize(None)? };
        Ok(Com)
    }
}
