use crate::encoder::Encoder;
use crate::simple_decoder::SimpleDecoder;
use crate::decoder::Decoder;
use bitvec::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_n7_k4_t1_test() {
        let msg = bitvec![1, 0, 0, 1];
        let prime_poly = bitvec![1, 0, 1, 1];

        let encoder = Encoder::new(7, 4, 1, &prime_poly);
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(7, 4, 1, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();

        assert_eq!(decoded, msg);
    }

    #[test]
    fn encode_decode_without_errors_n15_k7_t2_test() {
        let n = 15;
        let k = 7;
        let t = 2;

        let mut msg = bitvec![1, 0, 0, 1];
        msg.extend(bitvec![0; 3]);

        let prime_poly = bitvec![1,0,0,1,1];

        let encoder = Encoder::new(n, k, t, &prime_poly);        
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();

        assert_eq!(msg, decoded);
    }

    #[test]
    fn encode_decode_with_single_error_n15_k7_t2_test() {
        let n = 15;
        let k = 7;
        let t = 2;

        let mut msg = bitvec![1, 0, 0, 1];
        msg.extend(bitvec![0; 3]);

        let prime_poly = bitvec![1,0,0,1,1];

        let encoder = Encoder::new(n, k, t, &prime_poly);        
        let mut encoded = encoder.encode(&msg).unwrap();

        encoded.set(1, true);

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();

        assert_eq!(msg, decoded);
    }

    #[test]
    fn encode_decode_with_double_error_decodable_by_simple_decoder_n15_k7_t2_test() {
        let n = 15;
        let k = 7;
        let t = 2;

        let mut msg = bitvec![1, 0, 0, 1];
        msg.extend(bitvec![0; 3]);

        let prime_poly = bitvec![1,0,0,1,1];

        let encoder = Encoder::new(n, k, t, &prime_poly);        
        let mut encoded = encoder.encode(&msg).unwrap();

        encoded.set(0, false);
        encoded.set(1, true);

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn encode_decode_with_double_error_decodable_by_simple_decoder_n31_k21_t2_test() {
        let n = 31;
        let k = 21;
        let t = 2;

        let mut msg = bitvec![1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1];
        msg.extend(bitvec![0; 10]);

        let prime_poly = bitvec![1,0,0,1,0,1];

        let encoder = Encoder::new(n, k, t, &prime_poly);        
        let mut encoded = encoder.encode(&msg).unwrap();

        encoded.set(0, false);
        encoded.set(10, false);

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let result = decoder.decode(&encoded);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn encode_decode_without_errors_n255_k191_t8_test_full_layers() {
        let n = 255;
        let k = 191;
        let t = 8;

        let mut msg = bitvec![1,0,1,1];
        msg.extend(bitvec![0; 187]);

        let prime_poly = bitvec![1,0,0,0,1,1,1,0,1];

        let encoder = Encoder::new(n, k, t, &prime_poly);        
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();

        assert_eq!(msg, decoded);
    }

    #[test]
    fn encode_decode_without_errors_n255_k187_t9_test_not_full_layers() {
        let n = 255;
        let k = 187;
        let t = 9;

        let mut msg = bitvec![1,0,1,1];
        msg.extend(bitvec![0; 183]);

        let prime_poly = bitvec![1,0,1,1,0,0,0,1,1];

        let encoder = Encoder::new(n, k, t, &prime_poly);        
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();

        assert_eq!(msg, decoded);
    }
}