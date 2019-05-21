use std::env;

/*
 *
 *
 */

fn main() {
    let mac_addr: String = env::args().nth(1).unwrap();
    let mac_addr: &str = &mac_addr;

    println!("{:x?}", mac_addr);

    let parsed_mac_addr = mac_addr.split(|c| c == ':' || c == '-').enumerate().fold(
        [0; 6],
        |mut acc, (idx, byte)| match u8::from_str_radix(byte, 16) {
            Err(_err) => acc,
            Ok(v) => {
                acc[idx] = v;
                acc
            }
        },
    );

    println!("{:x?}", parsed_mac_addr);
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
