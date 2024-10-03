use crate::sender::packet_sender;
use crate::select_device::select_device;
use pcap::{Active, Capture, Device};
mod select_device;
mod settings;
mod sender;
mod packet_builder;

use pnet::packet::MutablePacket;
use std::error::Error;
use crate::settings::SettingsPattern;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut cap, device): (Capture<Active>, Device) = select_device()?;
    println!("デバイスの選択に成功しました: {}", device.name);

    packet_sender(&mut cap, SettingsPattern::Attack)?;

    Ok(())
}