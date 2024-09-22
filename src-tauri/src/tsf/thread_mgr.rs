use anyhow::Result;
use windows::Win32::{
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::TextServices::{CLSID_TF_ThreadMgr, ITfFunctionProvider, ITfThreadMgr2},
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
