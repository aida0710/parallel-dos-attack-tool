use super::settings::SendPacketSettings;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::tcp::MutableTcpPacket;
use pnet::packet::MutablePacket;
use rand::random;
use std::error::Error;

pub fn build_packet(settings: &SendPacketSettings) -> Result<Vec<u8>, Box<dyn Error>> {
    let total_length = 14 + 20 + 20 + settings.payload.len();
    let mut ethernet_buffer = vec![0u8; total_length];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    // Ethernet層の設定
    ethernet_packet.set_destination(settings.ethernet_dst_mac);
    ethernet_packet.set_source(settings.ethernet_src_mac);
    ethernet_packet.set_ethertype(pnet::packet::ethernet::EtherTypes::Ipv4);

    // IPv4層の設定
    let mut ipv4_packet = MutableIpv4Packet::new(ethernet_packet.payload_mut()).unwrap();
    ipv4_packet.set_version(settings.ipv4_version);
    ipv4_packet.set_header_length(5);
    ipv4_packet.set_dscp(settings.ipv4_dscp);
    ipv4_packet.set_ecn(settings.ipv4_ecn);
    ipv4_packet.set_total_length((20 + 20 + settings.payload.len()) as u16);
    ipv4_packet.set_identification(settings.ipv4_identification);
    ipv4_packet.set_flags(settings.ipv4_flags);
    ipv4_packet.set_ttl(64);
    ipv4_packet.set_next_level_protocol(settings.ipv4_next_level_protocol);
    ipv4_packet.set_source(settings.ipv4_src_ip);
    ipv4_packet.set_destination(settings.ipv4_dst_ip);

    // TCP/UDP層の設定
    let mut tcp_packet = MutableTcpPacket::new(ipv4_packet.payload_mut()).unwrap();
    tcp_packet.set_source(settings.src_port);
    tcp_packet.set_destination(settings.dst_port);
    tcp_packet.set_sequence(random::<u32>());
    tcp_packet.set_acknowledgement(0);
    tcp_packet.set_flags(settings.tcp_flags);
    tcp_packet.set_window(64240);
    tcp_packet.set_data_offset(5);
    tcp_packet.set_payload(&settings.payload);

    Ok(ethernet_buffer)
}