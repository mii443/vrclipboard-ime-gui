use std::net::UdpSocket;

use crate::{
    config::{Config, OnCopyMode},
    conversion::Conversion,
    tsf_conversion::TsfConversion,
    Log, STATE,
};
use anyhow::Result;
use chrono::Local;
use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler};
use regex::Regex;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use tauri::{AppHandle, Emitter, Manager};
use tracing::{error, info, warn};
use windows::Win32::System::DataExchange::GetClipboardOwner;

pub struct ConversionHandler {
    app_handle: AppHandle,
    conversion: Conversion,
    tsf_conversion: Option<TsfConversion>,
    clipboard_ctx: ClipboardContext,
    last_text: String,
}

impl ConversionHandler {
    pub fn new(app_handle: AppHandle) -> Result<Self> {
        let conversion = Conversion::new();
        let tsf_conversion = None;
        let clipboard_ctx = ClipboardProvider::new().unwrap();

        info!("ConversionHandler created");
        Ok(Self {
            app_handle,
            conversion,
            tsf_conversion,
            clipboard_ctx,
            last_text: String::new(),
        })
    }

    pub fn get_config(&self) -> Config {
        STATE.lock().unwrap().clone()
    }
}

impl ConversionHandler {
    fn clipboard_has_owner(&mut self) -> bool {
        unsafe { GetClipboardOwner() }.is_ok()
    }

    fn tsf_conversion(&mut self, contents: &str, config: &Config) -> Result<()> {
        if contents.chars().count() > 140 {
            info!("Content exceeds 140 characters, skipping TSF conversion");
            return Ok(());
        }
        if config.skip_url
            && Regex::new(r"(http://|https://){1}[\w\.\-/:\#\?=\&;%\~\+]+")
                .unwrap()
                .is_match(&contents)
        {
            info!("URL detected, skipping TSF conversion");
            return Ok(());
        }

        if self.tsf_conversion.is_none() {
            self.tsf_conversion = Some(TsfConversion::new());
            info!("TSF conversion created");
        }

        let tsf_conversion = self.tsf_conversion.as_mut().unwrap();

        let converted = tsf_conversion.convert(contents)?;

        info!("TSF conversion: {} -> {}", contents, converted);

        self.last_text = contents.to_string().clone();

        self.return_conversion(contents.to_string(), converted, config);

        Ok(())
    }

    fn return_conversion(&mut self, parsed_contents: String, converted: String, config: &Config) {
        match config.on_copy_mode {
            OnCopyMode::ReturnToClipboard => {
                let mut count = 0;
                while self.clipboard_ctx.set_contents(converted.clone()).is_err() {
                    if count > 4 {
                        warn!("Failed to set clipboard contents after 5 attempts");
                        break;
                    }
                    count += 1;
                }
                info!("Conversion returned to clipboard");
            }
            OnCopyMode::ReturnToChatbox => {
                let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
                let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                    addr: "/chatbox/input".to_string(),
                    args: vec![
                        OscType::String(converted.clone()),
                        OscType::Bool(false),
                        OscType::Bool(true),
                    ],
                }))
                .unwrap();

                if let Err(e) = sock.send_to(&msg_buf, "127.0.0.1:9000") {
                    error!("Failed to send UDP packet: {}", e);
                } else {
                    info!("Conversion returned to chatbox");
                }
            }
            OnCopyMode::SendDirectly => {
                let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
                let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                    addr: "/chatbox/input".to_string(),
                    args: vec![
                        OscType::String(converted.clone()),
                        OscType::Bool(true),
                        OscType::Bool(true),
                    ],
                }))
                .unwrap();

                if let Err(e) = sock.send_to(&msg_buf, "127.0.0.1:9000") {
                    error!("Failed to send UDP packet: {}", e);
                } else {
                    info!("Conversion sent directly");
                }
            }
        }

        let datetime = Local::now();
        if self
            .app_handle
            .emit(
                "addLog",
                Log {
                    time: datetime.format("%Y %m/%d %H:%M:%S").to_string(),
                    original: parsed_contents,
                    converted,
                },
            )
            .is_err()
        {
            error!("App handle add log failed");
        }
    }
}

impl ClipboardHandler for ConversionHandler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let config = self.get_config();
        if config.skip_on_out_of_vrc && self.clipboard_has_owner() {
            info!("Clipboard has owner (maybe from outside of VRChat), skipping conversion");
            return CallbackResult::Next;
        }

        if let Ok(mut contents) = self.clipboard_ctx.get_contents() {
            if config.use_tsf_reconvert {
                if let Err(e) = self.tsf_conversion(&contents, &config) {
                    error!("TSF conversion failed: {}", e);
                }
                return CallbackResult::Next;
            }

            if contents != self.last_text {
                if contents.starts_with(&config.prefix) || config.ignore_prefix {
                    if config.skip_url
                        && Regex::new(r"(http://|https://){1}[\w\.\-/:\#\?=\&;%\~\+]+")
                            .unwrap()
                            .is_match(&contents)
                    {
                        info!("URL detected, skipping conversion");
                        return CallbackResult::Next;
                    }

                    let parsed_contents = if config.ignore_prefix {
                        contents
                    } else {
                        contents.split_off(1)
                    };
                    let converted = match self.conversion.convert_text(&parsed_contents) {
                        Ok(converted) => converted,
                        Err(err) => {
                            error!("Conversion error: {:?}", err);
                            format!("Error: {:?}", err)
                        }
                    };

                    self.last_text = converted.clone();

                    self.return_conversion(parsed_contents, converted, &config);
                } else {
                    self.last_text = contents;
                }
            }
        }
        CallbackResult::Next
    }
}
