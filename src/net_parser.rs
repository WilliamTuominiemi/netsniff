use etherparse::{
    ArpPacketSlice, Ipv4HeaderSlice, Ipv4Slice, Ipv6HeaderSlice, Ipv6Slice, NetSlice,
};

pub struct NetParser<'a> {
    pub net: Option<NetSlice<'a>>,
}

impl<'a> NetParser<'a> {
    pub fn parse(&self) {
        match &self.net {
            Some(net_slice) => match net_slice {
                NetSlice::Ipv4(slice) => self.parse_ipv4_slice(slice),
                NetSlice::Ipv6(slice) => self.parse_ipv6_slice(slice),
                NetSlice::Arp(slice) => self.parse_arp_slice(slice),
            },
            _ => panic!(),
        }
    }

    fn parse_ipv4_slice(&self, slice: &Ipv4Slice<'a>) {
        self.parse_ipv4_header(slice.header());
    }

    fn parse_ipv6_slice(&self, slice: &Ipv6Slice<'a>) {
        self.parse_ipv6_header(slice.header());
    }

    fn parse_arp_slice(&self, slice: &ArpPacketSlice<'a>) {
        println!(
            "  [L3] Arp: {:?} -> {:?} ",
            slice.sender_protocol_addr(),
            slice.target_protocol_addr()
        );
    }

    fn parse_ipv4_header(&self, header: Ipv4HeaderSlice<'a>) {
        println!(
            "  [L3] Ipv4 {:?} -> {:?} ",
            header.source_addr(),
            header.destination_addr()
        );
    }

    fn parse_ipv6_header(&self, header: Ipv6HeaderSlice<'a>) {
        println!(
            "  [L3] Ipv6 {:?} -> {:?} ",
            header.source_addr(),
            header.destination_addr()
        );
    }
}
