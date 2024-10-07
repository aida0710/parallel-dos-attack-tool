use crate::select_device::select_device;
use crate::sender::packet_sender;
use pcap::{Active, Capture, Device};
mod select_device;
mod settings;
mod sender;
mod packet_builder;

use crate::settings::SendPacketSettingsBuilder;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpFlags;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut cap, device): (Capture<Active>, Device) = select_device()?;
    println!("デバイスの選択に成功しました: {}", device.name);

    let settings = SendPacketSettingsBuilder::new()
        .ipv4_src_ip("36.13.145.72".parse().unwrap())
        .ipv4_dst_ip("160.251.215.3".parse().unwrap())
        .ipv4_header_length(5)
        .src_port(12000)
        .dst_port(80)
        .ipv4_next_level_protocol(IpNextHeaderProtocols::Tcp)
        .tcp_flags(TcpFlags::SYN)
        .payload(vec![0])
        .packet_count(100000000)
        .interval(Duration::from_secs(1))
        .build();

    packet_sender(&mut cap, settings)?;

    Ok(())
}