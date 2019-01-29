#[cfg(test)]
mod tests {
    use crate::berlekamp_decoder::BerlekampDecoder;
    use crate::decoder::Decoder;
    use crate::encoder::Encoder;
    use crate::simple_decoder::SimpleDecoder;
    use rand::prelude::*;
    use bitvec::*;

    static n: i32 = 511;
    static k: i32 = 340;
    static t: i32 = 20;

    fn get_prime_poly() -> BitVec {
        bitvec![1, 0, 0, 0, 1, 0, 0, 0, 0, 1]
    }

    fn get_encoder() -> Encoder {
        Encoder::new(n, k, t, &get_prime_poly())
    }

    fn get_simple_decoder() -> SimpleDecoder {
        SimpleDecoder::new(n, k, t, &get_prime_poly())
    }

    fn get_berlekamp_decoder() -> BerlekampDecoder {
        BerlekampDecoder::new(n, k, t, &get_prime_poly())
    }

    fn generate_random_msg_and_code_word() -> (BitVec, BitVec) {
        let mut msg = BitVec::new();
        for _i in 0..k {
            msg.push(rand::random());
        }
        let encoder = get_encoder();
        let word = encoder.encode(&msg).unwrap();
        (msg, word)
    }

    #[test]
    #[ignore]
    fn test_single_errors() {
        let (msg, code_word) = generate_random_msg_and_code_word();
        
        let decoder = get_simple_decoder();
        let (decoded, _) = decoder.decode(&code_word).unwrap();
        assert_eq!(msg, decoded);
    } 
}