use std::net::UdpSocket;

use chrono::Local;
use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{ClipboardHandler, CallbackResult};
use regex::Regex;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use tauri::{AppHandle, Manager};
use crate::{config::{Config, OnCopyMode}, conversion::Conversion, tsf_conversion::TsfConversion, Log, STATE};
use anyhow::Result;

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

        Ok(Self { app_handle, conversion, tsf_conversion, clipboard_ctx, last_text: String::new() })
    }

    pub fn get_config(&self) -> Config {
        STATE.lock().unwrap().clone()
    }
}

impl ConversionHandler {
    fn tsf_conversion(&mut self, contents: &str, config: &Config) -> CallbackResult {
        if self.tsf_conversion.is_none() {
            self.tsf_conversion = Some(TsfConversion::new());

            println!("TSF conversion created.");
        }

        let tsf_conversion = self.tsf_conversion.as_mut().unwrap();

        CallbackResult::Next
    }
}

impl ClipboardHandler for ConversionHandler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let config = self.get_config();
        if let Ok(mut contents) = self.clipboard_ctx.get_contents() {
            if config.use_tsf_reconvert {
                return self.tsf_conversion(&contents, &config);
            }

            if contents != self.last_text {
                if contents.starts_with(&config.prefix) || config.ignore_prefix {

                    if config.skip_url && Regex::new(r"(http://|https://){1}[\w\.\-/:\#\?=\&;%\~\+]+").unwrap().is_match(&contents) {
                        return CallbackResult::Next;
                    }

                    let parsed_contents = if config.ignore_prefix { contents } else { contents.split_off(1) };
                    let converted = match self.conversion.convert_text(&parsed_contents) {
                        Ok(converted) => converted,
                        Err(err) => {
                            println!("Error: {:?}", err);
                            format!("Error: {:?}", err)
                        }
                    };

                    self.last_text = converted.clone();

                    match config.on_copy_mode {
                        OnCopyMode::ReturnToClipboard => {
                            let mut count = 0;
                            while self.clipboard_ctx.set_contents(converted.clone()).is_err() {
                                if count > 4 {
                                    break;
                                }
                                count += 1;
                            }
                        },
                        OnCopyMode::ReturnToChatbox => {
                            let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
                            let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                                addr: "/chatbox/input".to_string(),
                                args: vec![
                                    OscType::String(converted.clone()),
                                    OscType::Bool(false),
                                    OscType::Bool(true)
                                ]
                            })).unwrap();

                            sock.send_to(&msg_buf, "127.0.0.1:9000").unwrap();
                        },
                        OnCopyMode::SendDirectly => {
                            let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
                            let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                                addr: "/chatbox/input".to_string(),
                                args: vec![
                                    OscType::String(converted.clone()),
                                    OscType::Bool(true),
                                    OscType::Bool(true)
                                ]
                            })).unwrap();

                            sock.send_to(&msg_buf, "127.0.0.1:9000").unwrap();
                        },
                    }

                    let datetime = Local::now(); 
                    if self.app_handle
                        .emit_all("addLog", Log {
                            time: datetime.format("%Y %m/%d %H:%M:%S").to_string(),
                            original: parsed_contents,
                            converted
                        }).is_err() {
                            println!("App handle add log failed.");
                        }
                } else {
                    self.last_text = contents;
                }
            }
        }
        CallbackResult::Next
    }
}
