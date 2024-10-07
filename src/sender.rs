use super::packet_builder::build_packet;
use super::settings::SendPacketSettings;
use crossbeam_channel::{bounded, Receiver, SendError, Sender};
use pcap::{Active, Capture};
use rayon::prelude::*;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const BATCH_SIZE: usize = 1; // 一度に送信するパケット数
const PROGRESS_INTERVAL: usize = 100000; // 進捗表示間隔 10000 パケットごとに表示

#[derive(Debug)]
pub enum PacketSenderError {
    PacketBuildError,
    ChannelSendError,
    ChannelReceiveError,
    PacketSendError,
    ThreadJoinError,
}

impl fmt::Display for PacketSenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "パケット送信エラー")
    }
}

impl Error for PacketSenderError {}

pub fn packet_sender(cap: &mut Capture<Active>, settings: SendPacketSettings) -> Result<(), PacketSenderError> {
    send_packets(cap, Arc::new(settings))
}

fn send_packets(
    cap: &mut Capture<Active>,
    settings: Arc<SendPacketSettings>,
) -> Result<(), PacketSenderError> {
    let ethernet_buffer = Arc::new(build_packet(&settings)
        .map_err(|_| {
            println!("パケット構築エラー");
            PacketSenderError::PacketBuildError
        })?);

    let (tx, rx): (Sender<Vec<Vec<u8>>>, Receiver<Vec<Vec<u8>>>) = bounded(100);
    let producer_settings = Arc::clone(&settings);

    let producer_thread = thread::spawn(move || -> Result<(), SendError<Vec<Vec<u8>>>> {
        (0..producer_settings.packet_count)
            .collect::<Vec<_>>()
            .par_chunks(BATCH_SIZE)
            .try_for_each(|_| {
                let batch: Vec<Vec<u8>> = (0..BATCH_SIZE)
                    .map(|_| ethernet_buffer.to_vec())
                    .collect();
                tx.send(batch)
            })?;
        Ok(())
    });

    println!("パケット送信を開始します...");
    println!("送信設定: {{\n\
    \tethernet_src_mac: {:?},\n\
    \tethernet_dst_mac: {:?},\n\
    \tipv4_src_ip: {:?},\n\
    \tipv4_dst_ip: {:?},\n\
    \tipv4_version: {:?},\n\
    \tipv4_header_length: {:?},\n\
    \tipv4_dscp: {:?},\n\
    \tipv4_ecn: {:?},\n\
    \tipv4_identification: {:?},\n\
    \tipv4_next_level_protocol: {:?},\n\
    \tipv4_flags: {:?},\n\
    \ttcp_flags: {:?},\n\
    \tsrc_port: {:?},\n\
    \tdst_port: {:?},\n\
    \ttimeout: {:?},\n\
    \tpacket_count: {:?},\n\
    \tinterval: {:?} \n\
    }}",
        settings.ethernet_src_mac,
        settings.ethernet_dst_mac,
        settings.ipv4_src_ip,
        settings.ipv4_dst_ip,
        settings.ipv4_version,
        settings.ipv4_header_length,
        settings.ipv4_dscp,
        settings.ipv4_ecn,
        settings.ipv4_identification,
        settings.ipv4_next_level_protocol,
        settings.ipv4_flags,
        settings.tcp_flags,
        settings.src_port,
        settings.dst_port,
        settings.timeout,
        settings.packet_count,
        settings.interval
    );
    let start_time = Instant::now();
    let mut packets_sent = 0;
    let mut last_progress_time = Instant::now();

    while packets_sent < settings.packet_count {
        let batch = rx.recv()
            .map_err(|_| {
                println!("チャネル受信エラー");
                PacketSenderError::ChannelReceiveError
            })?;

        for packet in batch {
            cap.sendpacket(&packet[..])
                .map_err(|_| {
                    println!("パケット送信エラー");
                    PacketSenderError::PacketSendError
                })?;
            packets_sent += 1;
            thread::sleep(settings.interval);
            println!("パケット送信: {:?}", packets_sent);

            if packets_sent % PROGRESS_INTERVAL == 0 {
                let now = Instant::now();
                if now.duration_since(last_progress_time) >= Duration::from_secs(1) {
                    println!(
                        "パケット {} / {} を送信しました (経過時間: {:.2} 秒)",
                        packets_sent,
                        settings.packet_count,
                        start_time.elapsed().as_secs_f64()
                    );
                    last_progress_time = now;
                }
            }

            if packets_sent >= settings.packet_count {
                break;
            }
        }
    }

    producer_thread.join()
        .map_err(|_| {
            println!("スレッド結合エラー");
            PacketSenderError::ThreadJoinError
        })?
        .map_err(|_| {
            println!("チャネル送信エラー");
            PacketSenderError::ChannelSendError
        })?;

    let elapsed_time = start_time.elapsed();
    println!("パケット送信が完了しました");
    println!("総送信パケット数: {}", packets_sent);
    println!("経過時間: {:.2} 秒", elapsed_time.as_secs_f64());

    Ok(())
}