use std::fmt::{Debug, Display, Formatter};
use bitvec::prelude::*;

struct Packet {
    version: u8,
    type_id: u8,
    packet_type: PacketType
}

enum PacketType {
    OperatorPacket(Vec<Packet>),
    LiteralValue(u64)
}

impl Packet {
    fn new(buffer: &BitSlice<Msb0, u8>, mut index: &mut usize) -> Self {
        let version = &buffer[*index..*index+3].to_bitvec().load_be::<u8>().clone();
        let type_id = &buffer[*index+3..*index+6].to_bitvec().load_be::<u8>().clone();
        *index += 6;


        match type_id {
            4 => {
                let mut val: u64 = 0;
                for ch in  buffer[*index..].chunks_exact(5) {
                    *index += 5;
                    val = val * 16 + ch[1..5].load_be::<u64>();
                    if ch[0] == false {
                        break;
                    }
                }
                Self{
                    version: version.clone(),
                    type_id: type_id.clone(),
                    packet_type: PacketType::LiteralValue(val)
                }
            }

            _ => {
                let mut packets: Vec<Packet> = vec![];
                match &buffer[*index] {
                    false => {
                        *index += 1;
                        let arealen = &buffer[*index .. *index + 15].to_bitvec().load_be::<u16>();
                        *index += 15;
                        let origind = index.clone();

                        while origind + (*arealen as usize) > *index {
                            packets.push(Packet::new(buffer, index));
                        }
                    }
                    true => {
                        *index += 1;
                        let n_packets = &buffer[*index .. *index + 11].to_bitvec().load_be::<u16>();
                        *index += 11;
                        for _ in 0..*n_packets {
                            packets.push(Packet::new(buffer, index));
                        }

                    }
                }

                Self {
                    version: version.clone(),
                    type_id: type_id.clone(),
                    packet_type: PacketType::OperatorPacket(packets)
                }

            }
        }
    }

    fn sum_versions(&self) -> u64 {
        let ans = match &self.packet_type {
            PacketType::OperatorPacket(subpackets) => { subpackets.iter().map(|p| p.sum_versions()).sum() }
            PacketType::LiteralValue(_) => { 0 }
        } + self.version as u64;
        ans
    }

    fn get_value(&self) -> u64 {
        match &self.packet_type {
            PacketType::OperatorPacket(packets) => {
                match self.type_id {
                    0 => {
                        packets.iter().map(|p| p.get_value()).sum()
                    }
                    1 => {
                        let mut ans = 1;
                        packets.iter().for_each(|p| ans *= p.get_value());
                        ans
                    }
                    2 => {
                        packets.iter().map(|p| p.get_value()).min().unwrap()
                    }
                    3 => {
                        packets.iter().map(|p| p.get_value()).max().unwrap()
                    }
                    5 => {
                        if packets[0].get_value() > packets[1].get_value() { 1 } else { 0 }
                    }
                    6 => {
                        if packets[0].get_value() < packets[1].get_value() { 1 } else { 0 }
                    }
                    7 => {
                        if packets[0].get_value() == packets[1].get_value() { 1 } else { 0 }
                    }
                    _ => {
                        panic!("Unknown optype")
                    }
                }

            }
            PacketType::LiteralValue(val) => {
                *val
            }
        }
    }
}


pub fn run(inp: &str) -> i64 {
    let mut buffer: BitVec<Msb0, u8> = BitVec::new();

    inp.as_bytes().iter().for_each(|ch| {
        // dbg!(&buffer);
        if *ch <= ('9' as u8) {
            let parsed = ch - ('0' as u8);
            buffer.append(&mut BitVec::<Msb0, u8>::from_element(parsed)[4..].to_bitvec());
        } else if *ch >= ('A' as u8) {
            let parsed = ch - ('A' as u8) + 10;
            buffer.append(&mut BitVec::<Msb0, u8>::from_element(parsed)[4..].to_bitvec());
        }

    });

    let mut ind = 0;
    let root = Packet::new(&*buffer, &mut ind);
    // dbg!(&root);
    root.sum_versions() as i64
}
