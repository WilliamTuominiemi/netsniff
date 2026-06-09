use etherparse::{LinkSlice, SlicedPacket};

pub struct LinkParser<'a> {
    pub link: Option<LinkSlice<'a>>,
}

impl<'a> LinkParser<'a> {
    pub fn parse(&self) {
        match &self.link {
            Some(link_slice) => match link_slice {
                LinkSlice::Ethernet2(slice) => println!("{:?}", slice),
                LinkSlice::LinuxSll(slice) => println!("{:?}", slice),
                LinkSlice::EtherPayload(slice) => println!("{:?}", slice),
                LinkSlice::LinuxSllPayload(slice) => println!("{:?}", slice),
            },
            _ => panic!(),
        }
    }
}
