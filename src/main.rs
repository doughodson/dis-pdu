
use serde::{Serialize, Deserialize};

// IEEE 1278.1-1995
#[derive(Serialize, Deserialize)]    // serde traits
#[derive(Debug)]                     // default printing
#[repr(C)]                           // C representation
struct PDUHeader {
  protocol_version: u8,
  exercise_identifier: u8,
  pdu_type: u8,
  protocol_family: u8,
  time_stamp:u32,
  length:u16,
  padding: u16,
}

fn swap_bytes(x: &mut PDUHeader) {
   x.time_stamp = u32::from_be_bytes((x.time_stamp).to_ne_bytes());
   x.length     = u16::from_be_bytes((x.length).to_ne_bytes());
}

fn main() {
   let encoded: Vec<u8> = vec![0x05, 0x01, 0x01, 0x01, 0x83, 0x3b, 0x91, 0x8c, 0x00, 0x90, 0x00, 0x00];
   let mut decoded: PDUHeader = bincode::deserialize(&encoded).unwrap();
   swap_bytes(&mut decoded);

   println!("protocol_version    : {:?}", decoded.protocol_version);
   println!("exercise_identifier : {:?}", decoded.exercise_identifier);
   println!("pdu_type            : {:?}", decoded.pdu_type);
   println!("protocol_family     : {:?}", decoded.protocol_family);
   println!("time_stamp          : {:?}", decoded.time_stamp);
   println!("length              : {:?}", decoded.length);
   println!("padding             : {:?}", decoded.padding);
}

