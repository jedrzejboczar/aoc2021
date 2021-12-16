use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    LiteralValue {
        version: u8,
        // type_id: u8,  always 4
        value: usize,
    },
    Operator {
        version: u8,
        // type_id: u8,  // we have Operation
        operation: Operation,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GraterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<u8> for Operation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Operation::*;
        match value {
            0 => Ok(Sum),
            1 => Ok(Product),
            2 => Ok(Minimum),
            3 => Ok(Maximum),
            5 => Ok(GraterThan),
            6 => Ok(LessThan),
            7 => Ok(EqualTo),
            v => Err(v),
        }
    }
}

impl Operation {
    fn eval(&self, mut values: impl Iterator<Item = usize>) -> usize {
        let mut get_pair = || {
            let first = values.next().unwrap();
            let second = values.next().unwrap();
            assert!(values.next().is_none());
            (first, second)
        };
        match self {
            Operation::Sum => values.sum(),
            Operation::Product => values.product(),
            Operation::Minimum => values.min().unwrap(),
            Operation::Maximum => values.max().unwrap(),
            Operation::GraterThan => {
                let (op1, op2) = get_pair();
                (op1 > op2) as usize
            },
            Operation::LessThan => {
                let (op1, op2) = get_pair();
                (op1 < op2) as usize
            },
            Operation::EqualTo => {
                let (op1, op2) = get_pair();
                (op1 == op2) as usize
            },
        }
    }
}

impl Packet {
    fn from_bits(bits: &mut Bits) -> Self {
        let version = bits.next(3) as u8;
        let type_id = bits.next(3) as u8;

        // println!("version =  {0} = {0:08b}", version);
        // println!("type_id =  {0} = {0:08b}", type_id);

        if type_id == 4 {
            return Self::LiteralValue {
                version,
                value: Self::literal_value(bits),
            }
        }

        // if not then we are an operator packet
        let mut subpackets = Vec::new();

        let length_id = bits.next(1);
        match length_id {
            0 => {  // total length in bits of subpackets
                let mut n_bits = bits.next(15);
                let start = bits.current();
                while bits.current() - start < n_bits {
                    let packet = Packet::from_bits(bits);
                    subpackets.push(packet);
                }
            },
            1 => {  // number of subpackets
                let n_packets = bits.next(11);
                for _ in 0..n_packets {
                    let packet = Packet::from_bits(bits);
                    subpackets.push(packet);
                }
            },
            _ => panic!(),
        }

        let operation = Operation::try_from(type_id).unwrap();

        Self::Operator {
            version,
            operation,
            subpackets
        }
    }

    fn literal_value(bits: &mut Bits) -> usize {
        let mut value = 0;
        loop {
            let five = bits.next(5);
            value |= five & 0b1111;
            let more = (five & (1 << 4)) != 0;
            if more {
                value <<= 4;
            } else {
                break;
            }
        }
        value
    }

    fn sum_versions(&self) -> usize {
        match self {
            Self::LiteralValue { version, .. } => *version as usize,
            Self::Operator { version, subpackets, .. } => {
                *version as usize + subpackets.iter()
                    .map(Self::sum_versions)
                    .sum::<usize>()
            },
        }
    }

    fn eval(&self) -> usize {
        match self {
            Self::LiteralValue { value, .. } => *value,
            Self::Operator { operation, subpackets, .. } => {
                operation.eval(subpackets.iter().map(|p| p.eval()))
            },
        }
    }
}

pub fn part_1(input: impl Read, verbose: bool) -> usize {
    let bytes = load_data(input);
    let packet = Packet::from_bits(&mut bytes.as_slice().into());
    if verbose {
        println!("{:#?}", packet);
    }
    packet.sum_versions()
}

pub fn part_2(input: impl Read, verbose: bool) -> usize {
    let bytes = load_data(input);
    let packet = Packet::from_bits(&mut bytes.as_slice().into());
    packet.eval()
}

fn load_data(mut input: impl Read) -> Vec<u8> {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();

    let digits: Vec<_> = s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(16))
        .map(Option::unwrap)
        .collect();

    digits.chunks(2)
        .map(|nibbles| (nibbles[0] << 4) | *nibbles.get(1).unwrap_or(&0))
        .map(|val| val as u8)
        .collect()
}

struct Bits<'a> {
    bytes: &'a [u8],
    current: usize,
}

impl<'a> Bits<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, current: 0 }
    }

    pub fn next(&mut self, count: usize) -> usize {
        let value = get_bits(self.bytes, self.current, count);
        self.current += count;
        value
    }

    pub fn current(&self) -> usize {
        self.current
    }
}

impl<'a> From<&'a [u8]> for Bits<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self::new(bytes)
    }
}

fn get_bit_msb(byte: u8, i: usize) -> bool {
    ((byte >> i) & 1) != 0
}

fn get_bits(data: &[u8], offset: usize, count: usize) -> usize {
    let mut value = 0;
    for bit in 0..count {
        value <<= 1;
        let b = offset + bit;
        let byte_index = b / 8;
        let bit_index = b % 8;
        if get_bit_msb(data[byte_index], 7 - bit_index) {
            value |= 1;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_first_byte() {
        let data = [0b10001111, 0b10110011];
        assert_eq!(get_bits(&data, 0, 6), 0b100011);
    }

    #[test]
    fn across_bytes_boundary() {
        let data = [0b10001111, 0b10110011];
        assert_eq!(get_bits(&data, 5, 8), 0b11110110);
    }

    #[test]
    fn data_loading() {
        let bytes = load_data("D2FE28".as_bytes());
        assert_eq!(bytes, vec![0xd2, 0xfe, 0x28]);
        let bytes = load_data("D2FE28a".as_bytes());
        assert_eq!(bytes, vec![0xd2, 0xfe, 0x28, 0xa0]);
    }

    #[test]
    fn literal_value() {
        let bytes = load_data("D2FE28".as_bytes());
        let packet = Packet::from_bits(&mut bytes.as_slice().into());
        assert_eq!(packet, Packet::LiteralValue {
            version: 0b110,
            value: 2021,
        });
    }

    #[test]
    fn operator_total_len() {
        let bytes = load_data("38006F45291200".as_bytes());
        let packet = Packet::from_bits(&mut bytes.as_slice().into());
        assert_eq!(packet, Packet::Operator {
            version: 0b001,
            operation: Operation::LessThan,
            subpackets: vec![
                Packet::LiteralValue {
                    version: 0b110,
                    value: 10,
                },
                Packet::LiteralValue {
                    version: 0b010,
                    value: 20,
                },
            ],
        });
    }

    #[test]
    fn operator_n_subpackets() {
        let bytes = load_data("EE00D40C823060".as_bytes());
        let packet = Packet::from_bits(&mut bytes.as_slice().into());
        assert_eq!(packet, Packet::Operator {
            version: 0b111,
            operation: Operation::Maximum,
            subpackets: vec![
                Packet::LiteralValue {
                    version: 0b010,
                    value: 1,
                },
                Packet::LiteralValue {
                    version: 0b100,
                    value: 2,
                },
                Packet::LiteralValue {
                    version: 0b001,
                    value: 3,
                },
            ],
        });
    }

    fn sum_versions(s: &str) -> usize {
        let bytes = load_data(s.as_bytes());
        let packet = Packet::from_bits(&mut bytes.as_slice().into());
        packet.sum_versions()
    }

    #[test]
    fn sum_versions_1() { assert_eq!(sum_versions("8A004A801A8002F478"), 16); }

    #[test]
    fn sum_versions_2() { assert_eq!(sum_versions("620080001611562C8802118E34"), 12); }

    #[test]
    fn sum_versions_3() { assert_eq!(sum_versions("C0015000016115A2E0802F182340"), 23); }

    #[test]
    fn sum_versions_4() { assert_eq!(sum_versions("A0016C880162017C3686B18A3D4780"), 31); }

    fn eval(s: &str) -> usize {
        let bytes = load_data(s.as_bytes());
        let packet = Packet::from_bits(&mut bytes.as_slice().into());
        packet.eval()
    }

    #[test]
    fn eval_1() { assert_eq!(eval("C200B40A82"), 3); }

    #[test]
    fn eval_2() { assert_eq!(eval("04005AC33890"), 54); }

    #[test]
    fn eval_3() { assert_eq!(eval("880086C3E88112"), 7); }

    #[test]
    fn eval_4() { assert_eq!(eval("CE00C43D881120"), 9); }

    #[test]
    fn eval_5() { assert_eq!(eval("D8005AC2A8F0"), 1); }

    #[test]
    fn eval_6() { assert_eq!(eval("F600BC2D8F"), 0); }

    #[test]
    fn eval_7() { assert_eq!(eval("9C005AC2F8F0"), 0); }

    #[test]
    fn eval_8() { assert_eq!(eval("9C0141080250320F1802104A08"), 1); }
}

