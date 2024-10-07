use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::util::MacAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct SendPacketSettings {
    pub ethernet_src_mac: MacAddr,
    pub ethernet_dst_mac: MacAddr,
    pub ipv4_src_ip: Ipv4Addr,
    pub ipv4_dst_ip: Ipv4Addr,
    pub ipv4_version: u8,
    pub ipv4_header_length: u8,
    pub ipv4_dscp: u8,
    pub ipv4_ecn: u8,
    pub ipv4_identification: u16,
    pub ipv4_next_level_protocol: IpNextHeaderProtocol,
    pub ipv4_flags: u8,
    pub tcp_flags: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub timeout: Duration,
    pub payload: Vec<u8>,
    pub packet_count: usize,
    pub interval: Duration,
}

#[derive(Default)]
pub struct SendPacketSettingsBuilder {
    ethernet_src_mac: Option<MacAddr>,
    ethernet_dst_mac: Option<MacAddr>,
    ipv4_src_ip: Option<Ipv4Addr>,
    ipv4_dst_ip: Option<Ipv4Addr>,
    ipv4_version: Option<u8>,
    ipv4_header_length: Option<u8>,
    ipv4_dscp: Option<u8>,
    ipv4_ecn: Option<u8>,
    ipv4_identification: Option<u16>,
    ipv4_next_level_protocol: Option<IpNextHeaderProtocol>,
    ipv4_flags: Option<u8>,
    tcp_flags: Option<u8>,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    timeout: Option<Duration>,
    payload: Option<Vec<u8>>,
    packet_count: Option<usize>,
    interval: Option<Duration>,
}

impl SendPacketSettingsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ethernet_src_mac(mut self, value: MacAddr) -> Self {
        self.ethernet_src_mac = Some(value);
        self
    }

    pub fn ethernet_dst_mac(mut self, value: MacAddr) -> Self {
        self.ethernet_dst_mac = Some(value);
        self
    }

    pub fn ipv4_src_ip(mut self, value: Ipv4Addr) -> Self {
        self.ipv4_src_ip = Some(value);
        self
    }

    pub fn ipv4_dst_ip(mut self, value: Ipv4Addr) -> Self {
        self.ipv4_dst_ip = Some(value);
        self
    }

    // ipv4のバージョン デフォルトは4
    pub fn ipv4_version(mut self, value: u8) -> Self {
        self.ipv4_version = Some(value);
        self
    }

    // ipv4のヘッダ長 デフォルトは5
    pub fn ipv4_header_length(mut self, value: u8) -> Self {
        self.ipv4_header_length = Some(value);
        self
    }

    // ipv4のDSCP (Differentiated Services Code Point) デフォルトは0
    pub fn ipv4_dscp(mut self, value: u8) -> Self {
        self.ipv4_dscp = Some(value);
        self
    }

    // ipv4のECN (Explicit Congestion Notification) デフォルトは0
    pub fn ipv4_ecn(mut self, value: u8) -> Self {
        self.ipv4_ecn = Some(value);
        self
    }

    // ipv4の識別子 デフォルトは0
    pub fn ipv4_identification(mut self, value: u16) -> Self {
        self.ipv4_identification = Some(value);
        self
    }

    // ipv4の次のレベルのプロトコル デフォルトはTCP
    pub fn ipv4_next_level_protocol(mut self, value: IpNextHeaderProtocol) -> Self {
        self.ipv4_next_level_protocol = Some(value);
        self
    }

    // ipv4のフラグ デフォルトは0
    // 0x03: Reserved
    // 0x02: Don't Fragment
    // 0x01: More Fragments
    // 0x00: Don't Fragment, More Fragments
    pub fn ipv4_flags(mut self, value: u8) -> Self {
        self.ipv4_flags = Some(value);
        self
    }

    // TCPのフラグ デフォルトはSYN
    pub fn tcp_flags(mut self, value: u8) -> Self {
        self.tcp_flags = Some(value);
        self
    }

    // 送信元ポート デフォルトは10000
    pub fn src_port(mut self, value: u16) -> Self {
        self.src_port = Some(value);
        self
    }

    // 送信先ポート デフォルトは20000
    pub fn dst_port(mut self, value: u16) -> Self {
        self.dst_port = Some(value);
        self
    }

    // タイムアウト デフォルトは10秒
    pub fn timeout(mut self, value: Duration) -> Self {
        self.timeout = Some(value);
        self
    }

    // ペイロード 1000バイトの0埋めデータ
    pub fn payload(mut self, value: Vec<u8>) -> Self {
        self.payload = Some(value);
        self
    }

    // 送信パケット数 デフォルトは1
    pub fn packet_count(mut self, value: usize) -> Self {
        self.packet_count = Some(value);
        self
    }

    // 送信間隔 デフォルトは0秒
    // Duration::from_secs(1) // 1秒
    // Duration::from_millis(1000) // 1000ms = 1秒
    // Duration::from_micros(1000000) // 1000000us = 1秒
    // Duration::from_nanos(1000000000) // 1000000000ns = 1秒
    pub fn interval(mut self, value: Duration) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn build(self) -> SendPacketSettings {
        SendPacketSettings {
            ethernet_src_mac: self.ethernet_src_mac.unwrap_or(MacAddr::zero()),
            ethernet_dst_mac: self.ethernet_dst_mac.unwrap_or(MacAddr::zero()),
            ipv4_src_ip: self.ipv4_src_ip.unwrap_or_else(|| "0.0.0.0".parse().unwrap()),
            ipv4_dst_ip: self.ipv4_dst_ip.unwrap_or_else(|| "0.0.0.0".parse().unwrap()),
            ipv4_version: self.ipv4_version.unwrap_or(4),
            ipv4_header_length: self.ipv4_header_length.unwrap_or(5),
            ipv4_dscp: self.ipv4_dscp.unwrap_or(0),
            ipv4_ecn: self.ipv4_ecn.unwrap_or(0),
            ipv4_identification: self.ipv4_identification.unwrap_or(0),
            ipv4_next_level_protocol: self.ipv4_next_level_protocol.unwrap_or(IpNextHeaderProtocols::Tcp),
            ipv4_flags: self.ipv4_flags.unwrap_or(0),
            tcp_flags: self.tcp_flags.unwrap_or(0b001),
            src_port: self.src_port.unwrap_or(10000),
            dst_port: self.dst_port.unwrap_or(20000),
            timeout: self.timeout.unwrap_or(Duration::from_secs(10)),
            payload: self.payload.unwrap_or(vec![0; 1000]),
            packet_count: self.packet_count.unwrap_or(1),
            interval: self.interval.unwrap_or(Duration::from_secs(1)),
        }
    }
}