use chrono::{DateTime, Local, Utc};
use etherparse::{LinkSlice, SlicedPacket};
use pcap::{Device, Packet, PacketHeader};

use std::io;

mod net_parser;

fn main() {
    let devices = Device::list().unwrap();
    let device = configuration(devices);

    println!("Inspecting: {:?} - {}", device.desc, device.name);

    let mut cap = pcap::Capture::from_device(device.name.as_str())
        .unwrap()
        .open()
        .unwrap();

    while let Ok(packet) = cap.next_packet() {
        parse_time(packet.header);
        parse_packet(packet);
    }
}

fn configuration(devices: Vec<Device>) -> Device {
    println!("Pick a device: ");
    for (i, d) in devices.iter().enumerate() {
        println!("  {}: {:?}", i, d.desc);
    }

    let mut device_choice = String::new();

    io::stdin()
        .read_line(&mut device_choice)
        .expect("Unable to read Stdin");

    let index: usize = device_choice
        .trim()
        .parse()
        .expect("Please enter a valid number");

    devices.into_iter().nth(index).expect("Index out of range")
}

fn parse_time(header: &PacketHeader) {
    let timestamp = header.ts;
    let nanos = (timestamp.tv_usec as u32) * 1_000;

    if let Some(dt) = DateTime::from_timestamp(timestamp.tv_sec as i64, nanos) {
        let utc_time: DateTime<Utc> = dt;
        let local_time: DateTime<Local> = DateTime::from(utc_time);
        let naive_time = local_time.time();
        println!("[{}]: ", naive_time);
    }
}

fn parse_packet(packet: Packet) {
    match SlicedPacket::from_ethernet(&packet) {
        Err(value) => println!("Err {:?}", value),
        Ok(sliced_packet) => {
            let net = net_parser::NetParser {
                net: sliced_packet.net,
            };
            net.parse();
            // parse_link(sliced_packet.link);
        }
    };
}

fn parse_link(link: Option<LinkSlice>) {
    match link {
        Some(link_slice) => match link_slice {
            LinkSlice::Ethernet2(slice) => println!("{:?}", slice),
            LinkSlice::LinuxSll(slice) => println!("{:?}", slice),
            LinkSlice::EtherPayload(slice) => println!("{:?}", slice),
            LinkSlice::LinuxSllPayload(slice) => println!("{:?}", slice),
        },
        _ => panic!(),
    }
}
