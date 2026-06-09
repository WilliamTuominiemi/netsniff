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
            "  Ethernet2 MAC {:X?} -> {:X?} ",
            slice.source(),
            slice.destination()
        );
    }
}
