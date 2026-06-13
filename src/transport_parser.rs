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
            "  [L4] UDP PORT: {} -> {}",
            slice.source_port(),
            slice.destination_port()
        );
    }

    fn parse_tcp_slice(&self, slice: &TcpSlice) {
        println!(
            "  [L4] TCP PORT: {} -> {} | Flags: {:?}",
            slice.source_port(),
            slice.destination_port(),
            self.get_tcp_flag(slice)
        );
    }

    fn get_tcp_flag(&self, slice: &TcpSlice) -> Vec<&str> {
        let mut result: Vec<&str> = vec![];

        if slice.syn() {
            result.push("SYN");
        } else if slice.ack() {
            result.push("ACK");
        } else if slice.fin() {
            result.push("FIN");
        } else if slice.rst() {
            result.push("RST");
        } else if slice.psh() {
            result.push("PSH");
        } else if slice.urg() {
            result.push("URG");
        }

        result
    }
}
