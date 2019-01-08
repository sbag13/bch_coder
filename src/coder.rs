use crate::bitvec_operations;
use crate::common;
use bitvec::*;

struct BCHCoder {
    n: i32,
    k: i32,
    gen_poly: BitVec,
}

impl BCHCoder {
    fn new(n: i32, k: i32, gen_poly: BitVec) -> BCHCoder {
        common::validate_params(n, k, &gen_poly);
        BCHCoder {
            n: n,
            k: k,
            gen_poly: gen_poly,
        }
    }

    fn encode(self, mut data: BitVec) -> Result<BitVec, String> {
        if data.len() as i32 > self.k {
            return Err("Encode: to long data!".to_owned());
        }

        let control_len = self.n - self.k;
        data.extend(bitvec![0; control_len as usize]);

        let division_result = bitvec_operations::remainder_divide(&data, &self.gen_poly)?;
        data += division_result;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let encoder = BCHCoder::new(7, 4, bitvec![1, 0, 1, 1]);
        let result = encoder.encode(bitvec![1, 0, 0, 1]).unwrap();
        let expected = bitvec![1, 0, 0, 1, 1, 1, 0];
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn validate_params_fail_test_1() {
        BCHCoder::new(7, 3, bitvec![1, 0, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn validate_params_fail_test_2() {
        BCHCoder::new(7, 4, bitvec![0, 0, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn validate_params_fail_test_3() {
        BCHCoder::new(7, 4, bitvec![]);
    }
}
