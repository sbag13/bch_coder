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
        let (decoded, _) = decoder.decode(&encoded).unwrap();   //TODO maybe dont return err

        assert_eq!(decoded, msg);
    }

    // #[test]
    // fn encode_test_2() {
    //     let gen_pol = bitvec![1,0,1,1,0,0,0,1,1];
    //     let encoder = Encoder::new(255, 187, 9, gen_pol.clone());
    //     let mut v = bitvec![1, 0, 0, 1];
    //     v.extend(bitvec![0; 183]);
    //     let encoded = encoder.encode(v.clone()).unwrap();

    //     let decoder = SimpleDecoder::new(255, 187, 9, gen_pol);
    //     let (decoded, error) = decoder.decode(&encoded).unwrap();

    //     assert_eq!(v, decoded);
    // }
}