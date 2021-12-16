fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u32,
    binary_bits: String,
    type_id: u32,
    literal_data: Option<LiteralData>,
    operator_data: Option<OperatorData>,
}

#[derive(Debug, PartialEq)]
struct LiteralData {
    literal: u32
}

#[derive(Debug, PartialEq)]
struct OperatorData {
    length_type: LengthType,
    subpackets: Vec<Packet>
}

#[derive(Debug, PartialEq)]
struct LengthType {
    type_id: u32,
    length: u32,
}

fn decode_p1(input: &str) -> Vec<Packet> {
    let mut packets = Vec::new();
    let binary_bits = convert_hex_to_binary(input);
    let version = decode_version(&binary_bits);

    let type_id = decode_type_id(&binary_bits);
    
    if type_id == 4 {
        let packet = decode_literal_packet(&binary_bits);
        packets.push(packet);
        return packets
    } 
    let packet = decode_operator_packet(&binary_bits);
    packets.push(packet);
    return packets;
}

fn decode_literal_packet(binary_bits: &str) -> Packet {
    let version = decode_version(&binary_bits);
    let type_id = decode_type_id(&binary_bits);

    let packet = Packet {
        version: version,
        binary_bits: binary_bits.to_string(),
        type_id: type_id,
        literal_data: Some(LiteralData { literal: decode_literal(&binary_bits) }),
        operator_data: None,
    };
    return packet
}

fn decode_operator_packet(binary_bits: &str) -> Packet {
    let version = decode_version(&binary_bits);
    let type_id = decode_type_id(&binary_bits);
    let length_type_id = decode_length_type_id(&binary_bits);
    let length_in_bits = if length_type_id == 0 {
        decode_subpacket_bit_length(&binary_bits)
    } else {
        decode_num_subpackets(&binary_bits)
    };
    let mut packet = Packet {
        version: version,
        binary_bits: binary_bits.to_string(),
        type_id: type_id,
        literal_data: None,
        operator_data: None,
    };
    let mut operator_data = OperatorData {
        length_type: LengthType {
            type_id: length_type_id,
            length: length_in_bits,
        },
        // TODO: Find subpackets of operator data
        subpackets: Vec::new(),
    };
    return packet
}

fn build_packets(binary_bits: &str, size: u32) -> Vec<String> {
    // TODO: Redo this using packet struct
    let mut packets: Vec<String> = Vec::new();
    let mut curr_packet = String::new();
    let mut curr_packet_version = 0;
    let mut curr_packet_type_id = 0;
    let mut i = 0;
    let mut found_packet_at = 0;
    let mut binary_bits = binary_bits;
    while i < size {
        let c = binary_bits.chars().nth(i as usize).unwrap();
        if curr_packet.len() == 3 {
            curr_packet_version = decode_version(&curr_packet);
        } else if curr_packet.len() == 6 {
            curr_packet_type_id = decode_type_id(&curr_packet);
        }

        if curr_packet.len() == 6 {
            if curr_packet_type_id == 4 {
                let literals = parse_literals(&binary_bits[found_packet_at as usize..]);
                let bits_moved = literals.join("").len() - 1;
                curr_packet.push_str(&literals.join(""));
                i += bits_moved as u32;
                found_packet_at = i + 1;
                packets.push(curr_packet.clone());
                curr_packet.clear();
                curr_packet_version = 0;
                curr_packet_type_id = 0;
            } else {
                
            }
        } else {
            curr_packet.push(c);
        }
        i += 1;
    }
    return packets
}

fn convert_hex_to_binary(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c.to_ascii_lowercase() {
            '0' => result.push_str("0000"),
            '1' => result.push_str("0001"),
            '2' => result.push_str("0010"),
            '3' => result.push_str("0011"),
            '4' => result.push_str("0100"),
            '5' => result.push_str("0101"),
            '6' => result.push_str("0110"),
            '7' => result.push_str("0111"),
            '8' => result.push_str("1000"),
            '9' => result.push_str("1001"),
            'a' => result.push_str("1010"),
            'b' => result.push_str("1011"),
            'c' => result.push_str("1100"),
            'd' => result.push_str("1101"),
            'e' => result.push_str("1110"),
            'f' => result.push_str("1111"),
            _ => panic!("Invalid hex character: {}", c),
        }
    }
    result
}

fn convert_binary_to_decimal(input: &str) -> u32 {
    let mut result = 0;
    for c in input.chars() {
        match c {
            '0' => result = result * 2,
            '1' => result = result * 2 + 1,
            _ => panic!("Invalid binary character: {}", c),
        }
    }
    result
}

fn decode_version(input: &str) -> u32 {
    let version_bits = input[0..3].to_string();
    convert_binary_to_decimal(&version_bits)
}

fn decode_type_id(input: &str) -> u32 {
    let type_id_bits = input[3..6].to_string();
    convert_binary_to_decimal(&type_id_bits)
}

fn decode_literal(input: &str) -> u32 {
    let literal_bits = input[6..].to_string();
    // split literal_bits into chunks of 5 bits
    let mut chunks = Vec::new();
    for i in 0..(literal_bits.len() / 5) {
        let start = i * 5;
        let end = start + 5;
        chunks.push(literal_bits[start..end].to_string());
    }

    let mut literal_binary = String::new();
    for c in chunks {
        let binary = c[1..].to_string();
        literal_binary.push_str(&binary);
        let first_char = c[0..1].parse::<u32>().unwrap();
        if first_char == 0 {
            break;
        }
    }
    convert_binary_to_decimal(&literal_binary)
}


fn parse_literals(input: &str) -> Vec<String> {
    println!("parse_literals: {}", input);
    let literal_bits = input[6..].to_string();
    // split literal_bits into chunks of 5 bits
    let mut chunks = Vec::new();
    for i in 0..(literal_bits.len() / 5) {
        let start = i * 5;
        let end = start + 5;
        chunks.push(literal_bits[start..end].to_string());
    }

    let mut result = Vec::new();
    for c in chunks {
        result.push(c.clone());
        let first_char = c[0..1].parse::<u32>().unwrap();
        if first_char == 0 {
            break;
        }
    }
    result
}

fn decode_length_type_id(input: &str) -> u32 {
    let length_type_id_bit = input[6..7].to_string();
    return length_type_id_bit.parse::<u32>().unwrap();
}

fn decode_subpacket_bit_length(input: &str) -> u32 {
    let subpacket_bit_length_bits = input[7..22].to_string();
    return convert_binary_to_decimal(&subpacket_bit_length_bits);
}

fn decode_num_subpackets(input: &str) -> u32 {
    let num_subpackets_bits = input[7..18].to_string();
    return convert_binary_to_decimal(&num_subpackets_bits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_p1_literal_packet() {
        let packets = decode_p1("D2FE28");
        let packet = &packets[0];
        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(Some(LiteralData { literal: 2021 }), packet.literal_data);
        assert_eq!(None, packet.operator_data)
    }

    
    #[test]
    fn test_decode_p1_operator_packet() {
        let packets = decode_p1("38006F45291200");
        let packet = &packets[0];
        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);
        assert_eq!(None, packet.literal_data);
        let subpackets: Vec<Packet> = vec![
            decode_literal_packet("11010001010"),
            decode_literal_packet("0101001000100100"),
        ];
        let operator_data = OperatorData {
            length_type: LengthType {
                type_id: 0,
                length: 27,
            },
            subpackets: subpackets
        };
        assert_eq!(Some(operator_data), packet.operator_data);

    }

    #[test]
    fn test_convert_hex_to_binary() {
        let input = convert_hex_to_binary("D2FE28");
        assert_eq!(input, "110100101111111000101000");
    }

    #[test]
    fn test_convert_binary_to_decimal() {
        let input = convert_binary_to_decimal("01010");
        assert_eq!(input, 10);
    }

    #[test]
    fn test_decode_version() {
        let input = decode_version("110100101111111000101000");
        assert_eq!(input, 6);
    }

    #[test]
    fn test_decode_type_id() {
        let input = decode_type_id("110100101111111000101000");
        assert_eq!(input, 4);
    }

    #[test]
    fn test_decode_literal() {
        let input = decode_literal("110100010100101001000100100");
        assert_eq!(input, 10);
    }

    #[test]
    fn test_parse_literas() {
        let input = parse_literals("110100010100101001000100100");
        assert_eq!(input, vec!["01010".to_string()]);
    }

    #[test]
    fn test_decode_length_type_id() {
        let input = decode_length_type_id("00111000000000000110111101000101001010010001001000000000");
        assert_eq!(input, 0);
    }

    #[test]
    fn test_decode_subpacket_bit_length() {
        let input = decode_subpacket_bit_length("00111000000000000110111101000101001010010001001000000000");
        assert_eq!(input, 27);
    }

    #[test]
    fn test_decode_num_subpackets() {
        let input = decode_num_subpackets("11101110000000001101010000001100100000100011000001100000");
        assert_eq!(input, 3);
    }

    #[test]
    fn test_build_packets() {
        let input = build_packets("1101000101001010010001001000000000", 27);
        assert_eq!(input, vec!["11010001010", "0101001000100100"]);
    }
}
