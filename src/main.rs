use std::env;
use std::fmt;
use std::net;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mac_addr: String = env::args().nth(1).expect("wakelan <MAC>");
    let mac_addr: MacAddr = (&mac_addr).parse().expect("Invalid MAC address");

    let magic_packet = MagicPacket::new(&mac_addr);

    println!("Sending magic packet to {}", &mac_addr);

    match magic_packet.send() {
        Ok(_) => println!("Sent!"),
        Err(_) => println!("There was an error"),
    };
}

/// ```
/// WOL {
///  sync: [u8; 6]
///  mac: [u8; 96]
/// }
/// ```
pub struct MagicPacket {
    packet: [u8; 102],
}

impl MagicPacket {
    pub fn new(mac_addr: &MacAddr) -> Self {
        let mut packet = [0xff; 102];

        for i in 0..16 {
            for j in 0..6 {
                packet[6 + (i * 6) + j] = mac_addr.addr[j];
            }
        }

        MagicPacket { packet }
    }

    /// Broadcast magic packet to `255:255:255:255:9`
    pub fn send(&self) -> std::io::Result<()> {
        let socket = net::UdpSocket::bind((net::Ipv4Addr::new(0, 0, 0, 0), 0))?;

        socket.set_broadcast(true)?;
        socket.send_to(&self.packet, (net::Ipv4Addr::new(255, 255, 255, 255), 9))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct MacAddr {
    addr: [u8; 6],
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xs: String = self
            .addr
            .iter()
            .map(|x| format!("{:X}", x))
            .collect::<Vec<String>>()
            .join(":");

        write!(f, "{}", xs)
    }
}

impl FromStr for MacAddr {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value.split(|c| c == ':' || c == '-').enumerate().fold(
            [0; 6],
            |mut acc, (idx, byte)| match u8::from_str_radix(byte, 16) {
                Err(_err) => acc,
                Ok(v) => {
                    acc[idx] = v;
                    acc
                }
            },
        );

        Ok(MacAddr { addr: parsed })
    }
}
