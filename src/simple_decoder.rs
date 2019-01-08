use crate::bitvec_operations::*;
use crate::common;
use crate::decoder::*;
use bitvec::*;

struct SimpleDecoder {
    n: i32,
    k: i32,
    t: i32,
    gen_poly: BitVec,
}

impl SimpleDecoder {
    fn new(n: i32, k: i32, t: i32, gen_poly: BitVec) -> SimpleDecoder {
        common::validate_params(n, k, &gen_poly); //TODO extract to trait
        SimpleDecoder {
            n: n,
            k: k,
            t: t,
            gen_poly: gen_poly,
        }
    }
}

impl Decoder for SimpleDecoder {
    fn decode(self, encoded: &BitVec) -> Result<(BitVec, BitVec), String> {
        if encoded.len() > self.n as usize {
            return Err("Encoded data is too long!".to_owned());
        }

        let mut encoded_clone = encoded.clone();

        for i in 0..self.n {
            let mut syndrome = remainder_divide(&encoded_clone, &self.gen_poly)?;
            syndrome >>= self.n - self.k - syndrome.len() as i32;

            let hamming_weight = syndrome.count_ones();
            if hamming_weight <= self.t as usize {
                let mut extended_syndrom = bitvec![0; encoded_clone.len() - syndrome.len()];
                extended_syndrom.extend(syndrome.clone());

                let mut corrected = encoded_clone ^ extended_syndrom;

                shift_cyclic(&mut corrected, -i);
                let decoded = corrected.iter().take(self.k as usize).collect();

                let mut error = syndrome;
                shift_cyclic(&mut error, -i);

                return Ok((decoded, error));
            }
            shift_cyclic(&mut encoded_clone, 1);
        }

        Err("Could not decode".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_no_error_test() {
        let decoder = SimpleDecoder::new(7, 4, 1, bitvec![1, 0, 1, 1]);
        let (decoded_msg, error) = decoder.decode(&bitvec![1, 0, 0, 1, 1, 1, 0]).unwrap();
        let expected = bitvec![1, 0, 0, 1];
        assert_eq!(expected, decoded_msg);
        assert_eq!(error, bitvec![0, 0, 0]);
    }

    #[test]
    fn decode_correct_error_test() {
        let decoder = SimpleDecoder::new(7, 4, 1, bitvec![1, 0, 1, 1]);
        let (decoded_msg, error) = decoder.decode(&bitvec![1, 1, 0, 1, 1, 1, 0]).unwrap();
        let expected_msg = bitvec![1, 1, 0, 1];
        let expected_err = bitvec![0, 0, 1];
        assert_eq!(expected_msg, decoded_msg);
        assert_eq!(expected_err, error);
    }

    #[test]
    fn decode_error_test() {
        let decoder = SimpleDecoder::new(7, 4, 1, bitvec![1, 0, 1, 1]);
        let result = decoder.decode(&bitvec![1, 1, 1, 1, 1, 1, 0]);
        assert!(true, result.is_err());
    }
}
