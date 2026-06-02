use etherparse::SlicedPacket;
use pcap::{Device, Packet};

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
        println!("Packet received: {:?}", packet.header);
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

fn parse_packet(packet: Packet) {
    match SlicedPacket::from_ethernet(&packet) {
        Err(value) => println!("Err {:?}", value),
        Ok(value) => {
            println!("link: {:?}", value.link);
            println!("link_exts: {:?}", value.link_exts);
            println!("net: {:?}", value.net);
            println!("transport: {:?}", value.transport);
        }
    };
}
