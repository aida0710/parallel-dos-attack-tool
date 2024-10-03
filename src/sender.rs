use pcap::{Active, Capture};
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use crossbeam_channel::{bounded, Sender, Receiver, SendError};
use rayon::prelude::*;

use super::packet_builder::build_packet;
use super::settings::{SendPacketSettings, SettingsLocator, SettingsPattern};

const BATCH_SIZE: usize = 1000;
const PROGRESS_INTERVAL: usize = 10000;

#[derive(Debug)]
pub enum PacketSenderError {
    SettingsNotFound,
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

pub fn packet_sender(cap: &mut Capture<Active>, pattern: SettingsPattern) -> Result<(), PacketSenderError> {
    let locator = SettingsLocator::new();
    let settings = locator.get_settings(&pattern)
        .ok_or_else(|| {
            println!("設定が見つかりません");
            PacketSenderError::SettingsNotFound
        })?;

    send_packets(cap, settings)
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