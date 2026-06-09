use chrono::{DateTime, Local, Utc};
use etherparse::{
    ArpPacketSlice, Ipv4HeaderSlice, Ipv4Slice, Ipv6HeaderSlice, Ipv6Slice, LinkSlice, NetSlice,
    SlicedPacket,
};
use pcap::{Device, Packet, PacketHeader};

use std::io;

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
            parse_net(sliced_packet.net);
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

fn parse_net(net: Option<NetSlice>) {
    match net {
        Some(net_slice) => match net_slice {
            NetSlice::Ipv4(slice) => parse_ipv4_slice(slice),
            NetSlice::Ipv6(slice) => parse_ipv6_slice(slice),
            NetSlice::Arp(slice) => parse_arp_slice(slice),
        },
        _ => panic!(),
    }
}

fn parse_ipv4_slice(slice: Ipv4Slice) {
    parse_ipv4_header(slice.header());
}

fn parse_ipv6_slice(slice: Ipv6Slice) {
    parse_ipv6_header(slice.header());
}

fn parse_arp_slice(slice: ArpPacketSlice) {
    println!(
        "  Arp: {:?} -> {:?} ",
        slice.sender_protocol_addr(),
        slice.target_protocol_addr()
    );
}

fn parse_ipv4_header(header: Ipv4HeaderSlice) {
    println!(
        "  Ipv4 {:?} -> {:?} ",
        header.source_addr(),
        header.destination_addr()
    );
}

fn parse_ipv6_header(header: Ipv6HeaderSlice) {
    println!(
        "  Ipv6 {:?} -> {:?} ",
        header.source_addr(),
        header.destination_addr()
    );
}
