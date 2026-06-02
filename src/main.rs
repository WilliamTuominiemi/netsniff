use pcap::Device;
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
        println!("received packet! {:?}", packet);
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
