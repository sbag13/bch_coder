#[cfg(test)]
mod tests {
    use crate::berlekamp_decoder::BerlekampDecoder;
    use crate::decoder::Decoder;
    use crate::encoder::Encoder;
    use crate::bch_bitvec::*;
    use crate::simple_decoder::SimpleDecoder;
    use bitvec::*;

    #[test]
    fn encode_decode_n7_k4_t1_test() {
        let n = 7;
        let k = 4;
        let t = 1;

        let msg = bitvec![1, 0, 0, 1];
        let prime_poly = bitvec![1, 0, 1, 1];

        let encoder = Encoder::new(7, 4, 1, &prime_poly);
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
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

        let prime_poly = bitvec![1, 0, 0, 1, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    fn encode_decode_with_single_error_n15_k7_t2_test() {
        let n = 15;
        let k = 7;
        let t = 2;

        let mut msg = bitvec![1, 0, 0, 1];
        msg.extend(bitvec![0; 3]);

        let prime_poly = bitvec![1, 0, 0, 1, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let mut encoded = encoder.encode(&msg).unwrap();

        encoded.set(1, true);

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    fn encode_decode_with_double_error_decodable_by_simple_decoder_n15_k7_t2_test() {
        let n = 15;
        let k = 7;
        let t = 2;

        let mut msg = bitvec![1, 0, 0, 1];
        msg.extend(bitvec![0; 3]);

        let prime_poly = bitvec![1, 0, 0, 1, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let mut encoded = encoder.encode(&msg).unwrap();

        encoded.set(0, false);
        encoded.set(1, true);

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    fn encode_decode_with_double_error_not_decodable_by_simple_decoder_n31_k21_t2_test() {
        let n = 31;
        let k = 21;
        let t = 2;

        let mut msg = bitvec![1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1];
        msg.extend(bitvec![0; 10]);

        let prime_poly = bitvec![1, 0, 0, 1, 0, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let mut encoded = encoder.encode(&msg).unwrap();

        encoded.set(0, false);
        encoded.set(10, false);

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let result = decoder.decode(&encoded);
        assert_eq!(result.is_err(), true);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    #[ignore]
    fn encode_decode_without_errors_n255_k191_t8_test_full_layers() {
        let n = 255;
        let k = 191;
        let t = 8;

        let mut msg = bitvec![1, 0, 1, 1];
        msg.extend(bitvec![0; 187]);

        let prime_poly = bitvec![1, 0, 1, 1, 0, 1, 0, 0, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    #[ignore]
    fn encode_decode_without_errors_n255_k187_t9_test_not_full_layers() {
        let n = 255;
        let k = 187;
        let t = 9;

        let mut msg = bitvec![1, 0, 1, 1];
        msg.extend(bitvec![0; 183]);

        let prime_poly = bitvec![1, 0, 1, 1, 0, 0, 0, 1, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let encoded = encoder.encode(&msg).unwrap();

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    #[ignore]
    fn encode_decode_with_1_error_n255_k191_t8_test() {
        let n = 255;
        let k = 191;
        let t = 8;

        let mut msg = bitvec![1, 0, 1, 1];
        msg.extend(bitvec![0; 187]);

        let prime_poly = bitvec![1, 0, 1, 1, 0, 0, 0, 1, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let mut encoded = encoder.encode(&msg).unwrap();

        encoded.inverse_nth(99);

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    #[ignore]
    fn encode_decode_with_max_error_n255_k191_t8_test() {
        let n = 255;
        let k = 191;
        let t = 8;

        let mut msg = bitvec![1, 0, 1, 1];
        msg.extend(bitvec![0; 187]);

        let prime_poly = bitvec![1, 0, 1, 1, 0, 0, 0, 1, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let mut encoded = encoder.encode(&msg).unwrap();

        for i in 0..20{
            encoded.inverse_nth(i * 3);
        }

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let res = decoder.decode(&encoded);
        assert!(res.is_err());

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    #[ignore]
    fn encode_decode_without_errors_n511_k340_t20_test() {
        let n = 511;
        let k = 340;
        let t = 20;

        let mut msg = bitvec![1, 0, 1, 1];
        msg.extend(bitvec![0; 336]);

        let prime_poly = bitvec![1, 1, 0, 1, 1, 0, 0, 0, 0, 1];

        let encoder = Encoder::new(n, k, t, &prime_poly);
        let encoded = encoder.encode(&msg).unwrap();

        println!("elo");

        let decoder = SimpleDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(msg, decoded);

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, _) = decoder.decode(&encoded).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    fn decode_with_3_errors_n31_k16_t3() {
        let n = 31;
        let k = 16;
        let t = 3;

        let mut msg = bitvec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0,
            1, 0
        ];
        let msg_only = bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1];
        let remainder_only = bitvec![1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0];

        let prime_poly = bitvec![1, 0, 0, 1, 0, 1];

        let encoded = bitvec![
            0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0,
            1, 0
        ];

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, remainder) = decoder.decode(&encoded).unwrap(); //TODO remainder can be removed
        assert_eq!(decoded, msg_only);
        assert_eq!(remainder, remainder_only);
    }

    #[test]
    fn decode_without_errors_n31_k16_t3() {
        let n = 31;
        let k = 16;
        let t = 3;

        let mut msg = bitvec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0,
            1, 0
        ];
        let msg_only = bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1];
        let remainder_only = bitvec![1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0];

        let prime_poly = bitvec![1, 0, 0, 1, 0, 1];

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let (decoded, remainder) = decoder.decode(&msg).unwrap(); //TODO remainder can be removed
        assert_eq!(decoded, msg_only);
        assert_eq!(remainder, remainder_only);
    }

    fn decode_with_4_errors_should_fail_n31_k16_t3() {
        let n = 31;
        let k = 16;
        let t = 3;

        let mut msg = bitvec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0,
            1, 0
        ];
        let msg_only = bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1];
        let remainder_only = bitvec![1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0];

        let prime_poly = bitvec![1, 0, 0, 1, 0, 1];

        let encoded = bitvec![
            0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            1, 0
        ];

        let decoder = BerlekampDecoder::new(n, k, t, &prime_poly);
        let result = decoder.decode(&encoded); //TODO remainder can be removed
        assert!(result.is_err());
    }
}
