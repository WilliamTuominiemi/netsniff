use std::result;

use etherparse::{Ethernet2Slice, LinkSlice, SlicedPacket};

pub struct LinkParser<'a> {
    pub link: Option<LinkSlice<'a>>,
}

impl<'a> LinkParser<'a> {
    pub fn parse(&self) {
        match &self.link {
            Some(link_slice) => match link_slice {
                LinkSlice::Ethernet2(slice) => self.parse_ethernet_2_slice(slice),
                LinkSlice::LinuxSll(slice) => println!("LinuxSll"),
                LinkSlice::EtherPayload(slice) => println!("EtherPayload"),
                LinkSlice::LinuxSllPayload(slice) => println!("LinuxSllPayload"),
            },
            _ => panic!(),
        }
    }

    fn parse_ethernet_2_slice(&self, slice: &Ethernet2Slice) {
        println!(
            "  Ethernet2 MAC {} -> {} ",
            self.mac_as_array_to_string(&slice.source()),
            self.mac_as_array_to_string(&slice.destination())
        );
    }

    fn mac_as_array_to_string(&self, mac: &[u8]) -> String {
        mac.iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(":")
    }
}
