use crate::bitvec_operations;
use crate::common;
use bitvec::*;

pub struct Encoder {
    n: i32,
    k: i32,
    gen_poly: BitVec,
}

impl Encoder {
    pub fn new(n: i32, k: i32, t: i32, prime_poly: &BitVec) -> Encoder {
        let degree = n - k;
        let gen_poly = common::get_gen_poly(degree, t, prime_poly);
        common::validate_params(n, k, &gen_poly, prime_poly);
        Encoder {
            n: n,
            k: k,
            gen_poly: gen_poly,
        }
    }

    pub fn encode(self, data: &BitVec) -> Result<BitVec, String> {
        if data.len() as i32 > self.k {
            return Err("Encode: to long data!".to_owned());
        }

        let control_len = self.n - self.k;
        let mut data_clone = data.clone();
        data_clone.extend(bitvec![0; control_len as usize]);

        let division_remainder = bitvec_operations::remainder_divide(&data_clone, &self.gen_poly)?;
        data_clone += division_remainder;

        Ok(data_clone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Encoder;

    #[test]
    fn encode_test() {
        let encoder = Encoder::new(7, 4, 1, &bitvec![1, 0, 1, 1]);
        let result = encoder.encode(&bitvec![1, 0, 0, 1]).unwrap();
        let expected = bitvec![1, 0, 0, 1, 1, 1, 0];
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn validate_params_fail_when_prime_poly_too_short() {
        Encoder::new(7, 4, 1, &bitvec![0, 0, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn validate_params_fail_when_prime_poly_is_empty() {
        Encoder::new(7, 4, 1, &bitvec![]);
    }
}
