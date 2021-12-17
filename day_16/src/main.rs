fn main() {
    let p1_input = "E20D79005573F71DA0054E48527EF97D3004653BB1FC006867A8B1371AC49C801039171941340066E6B99A6A58B8110088BA008CE6F7893D4E6F7893DCDCFDB9D6CBC4026FE8026200DC7D84B1C00010A89507E3CCEE37B592014D3C01491B6697A83CB4F59E5E7FFA5CC66D4BC6F05D3004E6BB742B004E7E6B3375A46CF91D8C027911797589E17920F4009BE72DA8D2E4523DCEE86A8018C4AD3C7F2D2D02C5B9FF53366E3004658DB0012A963891D168801D08480485B005C0010A883116308002171AA24C679E0394EB898023331E60AB401294D98CA6CD8C01D9B349E0A99363003E655D40289CBDBB2F55D25E53ECAF14D9ABBB4CC726F038C011B0044401987D0BE0C00021B04E2546499DE824C015B004A7755B570013F2DD8627C65C02186F2996E9CCD04E5718C5CBCC016B004A4F61B27B0D9B8633F9344D57B0C1D3805537ADFA21F231C6EC9F3D3089FF7CD25E5941200C96801F191C77091238EE13A704A7CCC802B3B00567F192296259ABD9C400282915B9F6E98879823046C0010C626C966A19351EE27DE86C8E6968F2BE3D2008EE540FC01196989CD9410055725480D60025737BA1547D700727B9A89B444971830070401F8D70BA3B8803F16A3FC2D00043621C3B8A733C8BD880212BCDEE9D34929164D5CB08032594E5E1D25C0055E5B771E966783240220CD19E802E200F4588450BC401A8FB14E0A1805B36F3243B2833247536B70BDC00A60348880C7730039400B402A91009F650028C00E2020918077610021C00C1002D80512601188803B4000C148025010036727EE5AD6B445CC011E00B825E14F4BBF5F97853D2EFD6256F8FFE9F3B001420C01A88915E259002191EE2F4392004323E44A8B4C0069CEF34D304C001AB94379D149BD904507004A6D466B618402477802E200D47383719C0010F8A507A294CC9C90024A967C9995EE2933BA840";
    let p1_packets = decode_p1(p1_input);
    let p1_result = sum_version_of_packets(&p1_packets);
    println!("Part 1: {}", p1_result);
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u64,
    binary_bits: String,
    type_id: u64,
    literal_data: Option<LiteralData>,
    operator_data: Option<OperatorData>,
}

#[derive(Debug, PartialEq)]
struct LiteralData {
    literal: u64
}

#[derive(Debug, PartialEq)]
struct OperatorData {
    length_type: LengthType,
    subpackets: Vec<Packet>
}

#[derive(Debug, PartialEq)]
struct LengthType {
    type_id: u64,
    length: u64,
}

fn sum_version_of_packets(packets: &Vec<Packet>) -> u64 {
    let mut sum = 0;
    for packet in packets {
        sum += packet.version;
        if let Some(operator_data) = &packet.operator_data {
            sum += sum_version_of_packets(&operator_data.subpackets);
        }
    }
    sum
}

fn decode_p1(input: &str) -> Vec<Packet> {
    let binary_bits = convert_hex_to_binary(input);
    println!("Decoding from hex to binary: {}", binary_bits);
    decode_p1_from_binary(&binary_bits)
}

fn decode_p1_from_binary(binary_bits: &str) -> Vec<Packet> {
    let mut packets = Vec::new();
    println!("Decoding packets from binary: {}", binary_bits);
    let version = decode_version(&binary_bits);
    let type_id = decode_type_id(&binary_bits);
    let packet = if type_id == 4 {
        decode_literal_packet(&binary_bits)
    } else {
        decode_operator_packet(&binary_bits)
    };
    println!("Decoded packet: {:?}", packet);
    packets.push(packet);
    return packets;
}

fn decode_literal_packet(binary_bits: &str) -> Packet {
    println!("Decoding literal packet {}", binary_bits);
    let version = decode_version(&binary_bits);
    let type_id = decode_type_id(&binary_bits);
    println!("literal version: {}, type_id: {}", version, type_id);
    let literal_tuple = decode_literal(&binary_bits);
    let packet = Packet {
        version: version,
        binary_bits: literal_tuple.1,
        type_id: type_id,
        literal_data: Some(LiteralData { literal: literal_tuple.0 }),
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
    let mut operator_data = OperatorData {
        length_type: LengthType {
            type_id: length_type_id,
            length: length_in_bits,
        },
        subpackets: Vec::new(),
    };
    let subpackets = decode_operator_subpackets(&binary_bits, &operator_data);
    let header_bits = decode_operator_header_bits(&binary_bits);
    let packet_bits = combine_bits_in_packets(&subpackets);
    operator_data.subpackets = subpackets;
    let packet = Packet {
        version: version,
        binary_bits: format!("{}{}", header_bits, packet_bits),
        type_id: type_id,
        literal_data: None,
        operator_data: Some(operator_data),
    };
    return packet
}

fn decode_operator_header_bits(binary_bits: &str) -> String {
    let length_type_id = decode_length_type_id(&binary_bits);
    let initial_count = 7;
    let count = if length_type_id == 0 {
        15
    } else {
        11
    };
    binary_bits[..initial_count + count].to_string()
}


// combine bits in packets
fn combine_bits_in_packets(packets: &Vec<Packet>) -> String {
    let mut combined_bits = String::new();
    for packet in packets {
        combined_bits.push_str(&packet.binary_bits);
    }
    return combined_bits
}

fn decode_operator_subpackets(binary_bits: &str, operator_data: &OperatorData) -> Vec<Packet> {
    let mut packets = Vec::new();
    println!("Decoding operator subpackets {:?}", operator_data);
    let subpackets = if operator_data.length_type.type_id == 0 {
        decode_subpackets_by_bit_length(binary_bits, operator_data.length_type.length)
    } else {
        decode_subpackets_by_num_packets(binary_bits, operator_data.length_type.length)
    };
    packets.extend(subpackets);
    return packets
}

fn decode_subpackets_by_bit_length(binary_bits: &str, length: u64) -> Vec<Packet> {
    let packet_bits_starting = 22 as usize;
    let packet_bits_ending = packet_bits_starting + length as usize;
    let mut packet_bits = binary_bits[packet_bits_starting..packet_bits_ending].to_string();
    let mut packets: Vec<Packet> = Vec::new();
    let mut tmp_packets: Vec<Packet> = Vec::new();
    let mut checked_bits = 0;
    while checked_bits < length {
        tmp_packets = decode_p1_from_binary(&packet_bits);
        let sum_packet_bits = tmp_packets.iter().fold(0, |acc, packet| acc + packet.binary_bits.len());
        packets.extend(tmp_packets);
        checked_bits += sum_packet_bits as u64;
        packet_bits = packet_bits[sum_packet_bits..].to_string();
    }

    return packets
}

fn remove_leading_zeros(binary_bits: &str) -> String {
    let mut binary_bits = binary_bits.to_string();
    while binary_bits.starts_with("0") {
        binary_bits = binary_bits[1..].to_string();
    }
    return binary_bits
}

fn decode_subpackets_by_num_packets(binary_bits: &str, length: u64) -> Vec<Packet> {
    println!("Decoding subpackets by {} packets", length);
    let packet_bits_starting = 18 as usize;
    let mut packet_bits = binary_bits[packet_bits_starting..].to_string();
    println!("Packet bits: {}", packet_bits);
    let mut packets = Vec::new();
    let mut checked_bits = 0;
    while packets.len() < length as usize {
        let packet = decode_p1_from_binary(&packet_bits);
        println!("Packet: {:?}", packet);
        let sum_packet_bits = combine_bits_in_packets(&packet).len();
        packets.extend(packet);
        checked_bits += sum_packet_bits;
        packet_bits = packet_bits[sum_packet_bits..].to_string();
        println!("sum_packet_bits: {}, checked_bits: {}", sum_packet_bits, checked_bits);
        println!("Packet bits: {}", packet_bits);
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

fn convert_binary_to_decimal(input: &str) -> u64 {
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

fn decode_version(input: &str) -> u64 {
    let version_bits = input[0..3].to_string();
    convert_binary_to_decimal(&version_bits)
}

fn decode_type_id(input: &str) -> u64 {
    let type_id_bits = input[3..6].to_string();
    convert_binary_to_decimal(&type_id_bits)
}

fn decode_literal(input: &str) -> (u64, String) {
    let literal_bits = input[6..].to_string();
    // split literal_bits into chunks of 5 bits
    let mut chunks: Vec<String> = Vec::new();
    for i in 0..(literal_bits.len() / 5) {
        let start = i * 5;
        let end = start + 5;
        let chunk = literal_bits[start..end].to_string();
        chunks.push(chunk.clone());
        if chunk[0..1].parse::<u64>().unwrap() == 0 {
            break;
        }
    }
    let mut literal_binary = String::new();
    for c in chunks.clone() {
        let binary = c[1..].to_string();
        literal_binary.push_str(&binary);
    }
    let packet_bits = format!("{}{}", input[..6].to_string(), chunks.clone().join(""));
    (convert_binary_to_decimal(&literal_binary), packet_bits)
}

fn parse_literals(input: &str) -> Vec<String> {
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
        let first_char = c[0..1].parse::<u64>().unwrap();
        if first_char == 0 {
            break;
        }
    }
    result
}

fn decode_length_type_id(input: &str) -> u64 {
    let length_type_id_bit = input[6..7].to_string();
    return length_type_id_bit.parse::<u64>().unwrap();
}

fn decode_subpacket_bit_length(input: &str) -> u64 {
    let subpacket_bit_length_bits = input[7..22].to_string();
    return convert_binary_to_decimal(&subpacket_bit_length_bits);
}

fn decode_num_subpackets(input: &str) -> u64 {
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
    fn test_decode_subpackets_bit_length() {
        let packets = decode_subpackets_by_bit_length("00111000000000000110111101000101001010010001001000000000", 27);
        assert_eq!(packets.iter().map(|p| p.binary_bits.clone()).collect::<Vec<String>>(),
                   vec!["11010001010", "0101001000100100"]);
    }

    #[test]
    fn test_decode_subpackets_by_num_packets() {
        let packets = decode_subpackets_by_num_packets("11101110000000001101010000001100100000100011000001100000", 3);
        assert_eq!(packets.iter().map(|p| p.binary_bits.clone()).collect::<Vec<String>>(),
                   vec!["01010000001", "10010000010", "00110000011"]);
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
    fn test_decode_p1_operator_packet2() {
        let packets = decode_p1("EE00D40C823060");
        let packet = &packets[0];
        assert_eq!(7, packet.version);
        assert_eq!(3, packet.type_id);
        assert_eq!(None, packet.literal_data);
        let subpackets: Vec<Packet> = vec![
            decode_literal_packet("01010000001"),
            decode_literal_packet("10010000010"),
            decode_literal_packet("00110000011"),
        ];
        let operator_data = OperatorData {
            length_type: LengthType {
                type_id: 1,
                length: 3,
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
        let input = decode_literal("11010001010");
        assert_eq!(input.0, 10);
        assert_eq!(input.1, "11010001010");
    }

    #[test]
    fn test_parse_literals() {
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
    fn test_sum_version_of_packets() {
        let packets = decode_p1("8A004A801A8002F478");
        let sum = sum_version_of_packets(&packets);
        assert_eq!(sum, 16);
    }

    #[test]
    fn test_sum_version_of_packets2() {
        let packets = decode_p1("620080001611562C8802118E34");
        assert_eq!(packets[0].operator_data.as_ref().unwrap().subpackets.len(), 2);
        println!("Packets {:?}", packets);
        let sum = sum_version_of_packets(&packets);
        assert_eq!(sum, 12);
    }
    
    #[test]
    fn test_subpackets_by_num_packets2() {
        // 011 000 1 00000000010 000 000 0 000000000010110 000 100 01010 101 100 01011 001 000 1 00000000010 000 100 01100 011 100 01101 00
        // VVV TTT I LLLLLLLLLLL VVV TTT I LLLLLLLLLLLLLLL VVV TTT AAAAA VVV TTT AAAAA VVV TTT I LLLLLLLLLLL VVV TTT AAAAA VVV TTT AAAAA            
        // 3   0   1 2             0 0   0 22              0   4   10    5   4   11    1   0   1 2           0   4   12    3   4   13   
        //[                      [first op packet        [literal      ][literal   ]]  [second op packet    [literal      ][literal   ]]
        // 620080001611562C8802118E34 represents an operator packet (version 3) 
        // which contains two sub-packets; each sub-packet is an operator packet that contains two literal values. 
        // This packet has a version sum of 12.
        //101 100 0 01000101010110001011001000100000000010000100011000111000110100
        let subpackets = decode_subpackets_by_num_packets("01100010000000001000000000000000000101100001000101010110001011001000100000000010000100011000111000110100", 2);
        assert_eq!(subpackets.len(), 2);
    }

}
