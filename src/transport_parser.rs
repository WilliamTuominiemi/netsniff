use etherparse::{TcpSlice, TransportSlice, UdpSlice};

pub struct TransportParser<'a> {
    pub transport: Option<TransportSlice<'a>>,
}

impl<'a> TransportParser<'a> {
    pub fn parse(&self) {
        match &self.transport {
            Some(transport_slice) => match transport_slice {
                TransportSlice::Udp(slice) => self.parse_udp_slice(slice),
                TransportSlice::Tcp(slice) => self.parse_tcp_slice(slice),
                TransportSlice::Icmpv4(_) => println!(" icmpv4"),
                TransportSlice::Icmpv6(_) => println!(" icmpv6"),
            },
            _ => println!(" No port detected"),
        }
    }

    fn parse_udp_slice(&self, slice: &UdpSlice) {
        println!(
            "  UDP PORT: {} -> {}",
            slice.source_port(),
            slice.destination_port()
        );
    }

    fn parse_tcp_slice(&self, slice: &TcpSlice) {
        println!(
            "  TCP PORT: {} -> {}",
            slice.source_port(),
            slice.destination_port()
        );
    }
}
