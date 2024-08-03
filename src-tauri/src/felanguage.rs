use std::ptr;

use anyhow::Result;
use windows::{
    core::{w, PCWSTR},
    Win32::{
        System::Com::{CLSIDFromProgID, CoCreateInstance, CLSCTX_ALL},
        UI::Input::Ime::IFELanguage,
    },
};

pub struct FElanguage {
    ife: IFELanguage,
}

impl Drop for FElanguage {
    fn drop(&mut self) {
        unsafe { self.ife.Close().ok() };
    }
}

impl FElanguage {
    pub fn new() -> Result<Self> {
        let clsid = unsafe { CLSIDFromProgID(w!("MSIME.Japan"))? };
        let ife: IFELanguage = unsafe { CoCreateInstance(&clsid, None, CLSCTX_ALL)? };
        unsafe { ife.Open()? };
        Ok(FElanguage { ife })
    }

    pub fn j_morph_result(&self, input: &str, request: u32, mode: u32) -> Result<String> {
        let input_utf16: Vec<u16> = input.encode_utf16().chain(Some(0)).collect();
        let input_len = input_utf16.len();
        let input_pcwstr = PCWSTR::from_raw(input_utf16.as_ptr());

        let mut result_ptr = ptr::null_mut();
        unsafe {
            self.ife.GetJMorphResult(
                request,
                mode,
                input_len as _,
                input_pcwstr,
                ptr::null_mut(),
                &mut result_ptr,
            )?;
        }

        let result_struct = unsafe { ptr::read_unaligned(result_ptr) };
        let output_bstr_ptr = result_struct.pwchOutput;
        let output_bstr = unsafe { output_bstr_ptr.to_string()? };
        let output_string: String = output_bstr.chars().take(result_struct.cchOutput as usize).collect();

        Ok(output_string)
    }
}
