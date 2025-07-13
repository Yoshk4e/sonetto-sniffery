#![allow(non_snake_case)]

mod interceptor;
mod decoder;
mod sonettoproto;
mod error;
mod cmd_ids;
mod visualizer;

use interceptor::{Context, Interceptor};
use decoder::ClientPacket;
use cmd_ids::{CsNetCmdID, ScNetCmdID};

use std::{ffi::c_void, thread, time::Duration};
use std::convert::TryFrom;
use base64::Engine;
use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{GetProcAddress, GetModuleHandleA};
use visualizer::VisualLog;
use base64::engine::general_purpose;
use serde_json::json;

pub unsafe fn read_managed_string(ptr: *const u8) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    

    let len_ptr = ptr.add(0x10) as *const i32;
    let len = len_ptr.read();

    if len <= 0 || len > 2048 {
        return None;
    }

    let data_ptr = ptr.add(0x14) as *const u16;
    let slice    = std::slice::from_raw_parts(data_ptr, len as usize);

    String::from_utf16(slice).ok()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    _hmod: *mut c_void,
    reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;

    if reason == DLL_PROCESS_ATTACH {
        let handle = thread::spawn(|| {
            unsafe { windows::Win32::System::Console::AllocConsole(); }
            thread::sleep(Duration::from_secs(1));
            let vis_tx = visualizer::init_visualizer_server();
            println!("[sniffer] visualizer server started on ws://127.0.0.1:8888/ws");
            
            let game_assembly_base = loop {
                let h = unsafe { GetModuleHandleA(PCSTR(b"GameAssembly.dll\0".as_ptr())) };
                if let Some(handle) = h.ok() {
                    if handle.0 != 0 {
                        break handle.0 as usize;
                    }
                }
                thread::sleep(Duration::from_millis(100));
            };

            let mut intr = Interceptor::new(game_assembly_base);
            
            let rva = 0x5e8020;
            intr.attach(rva, |ctx: &mut Context| {
                let regs       = ctx.registers();
                let result     = regs.rdx as i16;
                let cmd_id     = regs.r8 as i32;
                let _data_ptr  = regs.r9 as *const u8;

                let stack = unsafe { std::slice::from_raw_parts(regs.rsp as *const usize, 10) };
                let tag   = (stack[4] & 0xFF) as u8;
                let sock  = stack[6] as i32;

                let cmd_name = ScNetCmdID::try_from(cmd_id)
                    .map(|id| format!("{:?}", id))
                    .unwrap_or_else(|_| "Unknown".into());

                println!(
                    "[SC:{:#06X} = {}] result_code={} down_tag={} sock={}",
                    cmd_id, cmd_name, result, tag, sock
                );
                
                let scdummy: String = String::from("well not now");

                visualizer::send_visual_log(VisualLog{
                    source: 0,
                    packetID: cmd_id.clone() as u16,
                    protoName: cmd_name.clone(),
                    object: json!({
                        "tag": tag.clone(),
                        "data": scdummy
                    }),
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    packet: "ayy".to_string()
                })
            });
            println!("[sniffer] hooked LuaMsgHandler::Handle at RVA 0x{:X}", rva);
            
            intr.attach(0x5EE110, |ctx| {
                let this = ctx.registers().rcx as usize;
                let cmd  = ctx.registers().rdx as i16;
                println!("[ProtobufMsgPacket::set_Cmd] this=0x{:X}, cmd={:#06X}", this, cmd);
            });

            intr.attach(0x5EE0D0, |ctx| {
                let this = ctx.registers().rcx as usize;
                println!("[ProtobufMsgPacket::get_Cmd] this=0x{:X}", this);
            });
            
            intr.attach(0x1EA75C0, move |ctx: &mut Context| {
                let str_ptr = ctx.registers().rcx as *const u8;
                if let Some(msg) = unsafe { read_managed_string(str_ptr) } {
                    println!("[vlog] {}", msg);
                    /*visualizer::send_visual_log(VisualLog::Vlog { message: msg });*/
                } else {
                    println!("[vlog] <failed to read string>");
                }
            });
            
            let ws2_base = loop {
                let h = unsafe { GetModuleHandleA(PCSTR(b"ws2_32.dll\0".as_ptr())) };
                let handle = h.unwrap();
                if handle.0 != 0 {
                    break handle.0 as usize;
                }
                thread::sleep(Duration::from_millis(100));
            };

            let mut intr_ws2 = Interceptor::new(ws2_base);

            let send_addr = unsafe {
                GetProcAddress(HMODULE(ws2_base as _), PCSTR(b"send\0".as_ptr()))
            }
                .expect("GetProcAddress failed") as usize;

            let send_offset = send_addr - ws2_base;

            let vis_tx = vis_tx.clone();
            intr_ws2.attach(send_offset, move |ctx| {
                let regs = ctx.registers();
                let sock = regs.rcx as i32;
                let buf  = regs.rdx as *const u8 as *mut u8;
                let len  = regs.r8 as i32;

                if !buf.is_null() && len > 0 {
                    let slice = unsafe { std::slice::from_raw_parts(buf, len as usize) };

                    if let Ok(pkt) = ClientPacket::decode(slice) {
                        let cmd_enum = CsNetCmdID::try_from(pkt.cmd_id as i32).ok();
                        let cmd_name = cmd_enum
                            .as_ref()
                            .map(|id| format!("{:?}", id))
                            .unwrap_or_else(|| "Unknown".into());

                        println!(
                            "[CS:{:#06X} = {}] len={} → seq={} tag={} data={:02X?}",
                            pkt.cmd_id,
                            cmd_name,
                            slice.len(),
                            pkt.sequence,
                            pkt.up_tag,
                            &pkt.data[..pkt.data.len().min(16)]
                        );

                        let decoded_json = if let Some(cmd_enum) = cmd_enum {
                            if let Some(decoded) = decoder::decode_cs_packet(&format!("{:?}", cmd_enum), &pkt.data) {
                                println!("{:#?}", decoded);
                                decoded
                            } else {
                                println!("→ [!] Could not decode payload for {}", cmd_enum as i32);
                                Box::new(serde_json::Value::Null)
                            }
                        } else {
                            Box::new(serde_json::Value::Null)
                        };

                        visualizer::send_visual_log(VisualLog {
                            source: 1, // client
                            packetID: pkt.cmd_id as u16,
                            protoName: cmd_name,
                            object: json!({
                                "seq": pkt.sequence,
                                "tag": pkt.up_tag,
                                "data": format!("{:?}", decoded_json)
                            }),
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as u64,
                            packet: general_purpose::STANDARD.encode(&slice)
                        });
                    } else {
                        /*
                        println!(
                            "[send:{}] undecodable → {:02X?}",
                            sock,
                            &slice[..slice.len().min(32)]
                        );
                        visualizer::send_visual_log(VisualLog {
                            source: 1,
                            packetID: 0,
                            protoName: "Undecodable".to_string(),
                            object: serde_json::Value::from(json!({ "error": "Failed to decode packet" })),
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as u64,
                            packet: general_purpose::STANDARD.encode(&slice),
                        });
                        */
                    }
                }
            });

            println!("[sniffer] hooked send at offset 0x{:X}", send_offset);
            
            loop {
                thread::sleep(Duration::from_secs(10));
            }
        });
        1
    } else {
        1
    }
}