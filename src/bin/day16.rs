use aoc2021_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day16";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

#[derive(Debug)]
struct OperatorPacket {
    length_type: bool,
    length: u16,
    content: Box<Vec<Packet>>,
}

#[derive(Debug)]
enum PacketContent {
    Literal(u64),
    Operator(u8, OperatorPacket),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    variable: PacketContent,
}

impl OperatorPacket {
    fn _from_bit_scanner<I: Iterator<Item=u8>>(bs: &mut BitScanner<I>) -> (u64, OperatorPacket) {
        let length_type = bs.take(1) == 1;
        if !length_type {
            let length = bs.take(15) as u16;
            let mut content = vec![];

            let mut bits = 0;
            while bits < (length as u64) {
                let (i, p) = Packet::_from_bit_scanner(bs);
                content.push(p);
                bits += i;
            }

            (bits + 16, OperatorPacket {
                length_type,
                length,
                content: Box::new(content)
            })
        } else {
            let length = bs.take(11) as u16;
            let mut content = vec![];
            let mut bits = 0;
            for _ in 0..length {
                let (i, p) = Packet::_from_bit_scanner(bs);
                content.push(p);
                bits += i;
            }

            (bits + 12, OperatorPacket {
                length_type,
                length,
                content: Box::new(content)
            })
        }
    }
}

impl PacketContent {
    fn _from_bit_scanner<I: Iterator<Item=u8>>(bs: &mut BitScanner<I>) -> (u64, PacketContent) {
        let mut bits = 3;
        match bs.take(3) {
            4 => {
                let mut res = 0;
                loop {
                    let cont = bs.take(1) == 1;
                    res = (res << 4) + bs.take(4);
                    bits += 5;
                    if !cont {
                        break;
                    }
                }
                (bits, PacketContent::Literal(res))
            },
            typ => {
                let (i, op) = OperatorPacket::_from_bit_scanner(bs);
                (bits + i, PacketContent::Operator(typ as u8, op))
            }
        }
    }
}

impl Packet {
    fn _from_bit_scanner<I: Iterator<Item=u8>>(bs: &mut BitScanner<I>) -> (u64, Packet) {
        let version = bs.take(3) as u8;
        let (i, variable) = PacketContent::_from_bit_scanner(bs);
        (i + 3, Packet {
            version,
            variable
        })
    }

    fn from_bit_scanner<I: Iterator<Item=u8>>(bs: &mut BitScanner<I>) -> Packet {
        Self::_from_bit_scanner(bs).1
    }

    fn version_sum(&self) -> u64{
        (self.version as u64) + match &self.variable {
            PacketContent::Literal(_) => 0,
            PacketContent::Operator(_, op) => op.content.iter().map(Packet::version_sum).sum()
        }
    }

    fn apply(&self) -> u64 {
       match &self.variable {
           PacketContent::Literal(n) => *n,
           PacketContent::Operator(0, op_pkt) => op_pkt.content.iter().map(Packet::apply).sum(),
           PacketContent::Operator(1, op_pkt) => op_pkt.content.iter().map(Packet::apply).product(),
           PacketContent::Operator(2, op_pkt) => op_pkt.content.iter().map(Packet::apply).min().unwrap(),
           PacketContent::Operator(3, op_pkt) => op_pkt.content.iter().map(Packet::apply).max().unwrap(),
           PacketContent::Operator(5, op_pkt) => if op_pkt.content[0].apply() > op_pkt.content[1].apply() { 1 } else { 0 },
           PacketContent::Operator(6, op_pkt) => if op_pkt.content[0].apply() < op_pkt.content[1].apply() { 1 } else { 0 },
           PacketContent::Operator(7, op_pkt) => if op_pkt.content[0].apply() == op_pkt.content[1].apply() { 1 } else { 0 },
           _ => panic!("{:?}", self)
       }
    }
}

struct BitScanner<I: Iterator<Item=u8>> {
    byte_iter: I,
    byte: Option<u8>,
    byte_index: u8,
}

impl<I: Iterator<Item=u8>> BitScanner<I> {
    fn new(mut iter: I) -> Self {
        let by = iter.next();
        Self {
            byte_iter: iter,
            byte: by,
            byte_index: 3,
        }
    }
}

impl<I: Iterator<Item=u8>> BitScanner<I> {
    fn take(&mut self, n: u8) -> u64 {
        // print!("n:{} by:{:?} byi:{} ", n, self.byte, self.byte_index);
        if n > 64 {
            panic!("Can't take more than 64 bits from BitScanner, asked for {}", n);
        };
        if self.byte.is_none() {
            return 0;
        }
        let byte = self.byte.unwrap();
        let mut n = n;
        let mut res = if n > self.byte_index {
            let mut pre = if self.byte_index < 3 {
                (byte & ((1 << self.byte_index + 1) - 1)) as u64
            } else {
                byte as u64
            };
            n -= self.byte_index + 1;
            pre <<= n;
            pre += (&mut self.byte_iter).take((n / 4) as usize)
                .fold(0, |acc, by| (acc << 4) + (by as u64))
                << n%4;
            n = n % 4;
            self.byte = self.byte_iter.next();
            self.byte_index = 3;
            pre
        } else {
            0
        };

        if self.byte.is_none() {
            // println!("r: {}", res);
            return res;
        }
        let byte = self.byte.unwrap();

        if n == 0 {
            // println!("r: {}", res);
            res
        } else {
            res += ((byte >> (self.byte_index + 1 - n)) & ((1 << n) - 1)) as u64;
            self.byte_index -= n;
            // println!("r: {}", res);
            res
        }
    }
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut bs = BitScanner::new(input[0]
        .chars()
        .map(|c| c.to_digit(16)
            .unwrap() as u8));

    let packet = Packet::from_bit_scanner(&mut bs);

    println!("{:?}", packet);

    let res1 = packet.version_sum();

    Ok([Some(res1.to_string()), Some(packet.apply().to_string())])
}

#[test]
pub fn test_day16() {
    assert!(common::run_test(DAY, &run))
}