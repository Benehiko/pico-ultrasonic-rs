use heapless::{String, Vec};

const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

const MULTIPLES: [u8; 16] = [
    0, 16, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240,
];

pub fn mac_addr_to_str(mac: [u8; 6]) -> String<18> {
    mac.iter()
        .map(|&b| {
            let mut v: Vec<char, 3> = Vec::new();
            let item = u8_to_hex(b);
            for i in 0..2 {
                v.push(item[i]).unwrap();
            }
            v.push(':').unwrap();
            v
        })
        .flatten()
        .collect::<Vec<char, 18>>()[0..17]
        .iter()
        .collect::<String<18>>()
}

fn u8_to_hex(num: u8) -> [char; 2] {
    let mut results: [char; 2] = ['0', '0'];
    let mut quotient = num;
    let mut counter = 0;
    while quotient > 1 {
        for i in (0..=15).rev() {
            if quotient >= MULTIPLES[i] {
                results[counter] = HEX_CHARS[(quotient - MULTIPLES[i]) as usize];
                quotient = i as u8;
                counter += 1;
                break;
            }
        }
    }
    results.reverse();

    results
}
