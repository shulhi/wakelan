use std::env;
use std::num::ParseIntError;
use std::str::FromStr;

/*
 *
 *
 */

fn main() {
    let mac_addr: String = env::args().nth(1).expect("wakelan <MAC>");
    let mac_addr: MacAddr = (&mac_addr).parse().expect("Invalid MAC address");

    println!("{:x?}", mac_addr);
}

/*
 *
 * WOL {
 *  sync: [u8; 6]
 *  mac: [u8; 96]
 * }
 *
 */
pub struct WOLPacket {
    packet: [u8; 102],
}

#[derive(Debug)]
pub struct MacAddr {
    addr: [u8; 6],
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
