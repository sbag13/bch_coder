use bitvec::*;

pub trait BCHBitVec {
    fn finite_add(&self, vec: &BitVec) -> BitVec;
    fn truncate_preceding_zeros(&mut self);
    fn precede_with_zeros(&mut self, n: usize);
    fn remainder_divide(&self, divisor_orig: &BitVec) -> Result<BitVec, String>;
    fn shift_cyclic(&mut self, n: i32); //TODO to doc: negatives shift rights, positives shift left
}

impl BCHBitVec for BitVec {
    fn finite_add(&self, vec: &BitVec) -> BitVec {
        let mut self_clone = self.clone();
        let mut vec_clone = vec.clone();
        if self_clone.len() > vec.len() {
            vec_clone >>= self_clone.len() - vec_clone.len();
        } else if vec.len() > self_clone.len() {
            self_clone >>= vec_clone.len() - self_clone.len();
        }

        self_clone ^ vec_clone
    }

    fn truncate_preceding_zeros(&mut self) {
        let pos = self.iter().position(|bit| bit == true);
        match pos {
            Some(i) => *self = self.iter().skip(i).collect(),
            None => *self = bitvec![],
        }
    }

    fn precede_with_zeros(&mut self, n: usize) {
        *self >>= n;
    }

    fn remainder_divide(&self, divisor_orig: &BitVec) -> Result<BitVec, String> {
        // TODO to doc: result has len of divisor.len() after truncation - 1
        let mut self_clone = self.clone();
        let mut divisor = divisor_orig.clone();

        self_clone.truncate_preceding_zeros();
        divisor.truncate_preceding_zeros();

        if divisor.len() == 0 {
            return Err("Division by zero polynomial!".to_owned());
        }
        if divisor.len() > self_clone.len() {
            return Ok(self_clone);
        }

        let mut remainder: BitVec = self_clone.iter().take(divisor.len()).collect();
        remainder ^= divisor.clone();

        for bit in self_clone.iter().skip(divisor.len()) {
            remainder <<= 1;
            remainder.push(bit);
            if remainder.get(0) == true {
                remainder ^= divisor.clone();
            }
        }
        remainder <<= 1;
        Ok(remainder)
    }

    fn shift_cyclic(&mut self, n: i32) {
        let tmp_clone = self.clone();
        let len = tmp_clone.len() as i32;
        for i in 0..self.len() {
            let mut pos = (i as i32 + n) % len;
            if pos < 0 {
                pos += len;
            }
            let bit = tmp_clone.get(pos as usize);
            self.set(i as usize, bit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precede_with_zeros() {
        let mut vec = bitvec![1, 0];
        vec.precede_with_zeros(2);
        let expected = bitvec![0, 0, 1, 0];
        assert_eq!(vec, expected);
    }

    #[test]
    fn finite_multiply_test_when_self_shorter_than_multiplier() {
        let vec = bitvec![1, 0, 1];
        let multiplier = bitvec![1, 0, 0, 1];
        let expected = bitvec![1, 1, 0, 0];
        let result = vec.finite_add(&multiplier);
        assert_eq!(expected, result);
    }

    #[test]
    fn finite_multiply_test_when_multiplier_shorter() {
        let multiplier = bitvec![1, 0, 1];
        let vec = bitvec![1, 0, 0, 1];
        let expected = bitvec![1, 1, 0, 0];
        let result = vec.finite_add(&multiplier);
        assert_eq!(expected, result);
    }

    #[test]
    fn shift_cyclic_test() {
        let mut vec = bitvec![1, 0, 1, 1];
        vec.shift_cyclic(2);
        assert_eq!(vec, bitvec![1, 1, 1, 0]);

        let mut vec = bitvec![1, 0, 1, 1];
        vec.shift_cyclic(4);
        assert_eq!(vec, bitvec![1, 0, 1, 1]);

        let mut vec = bitvec![1, 0, 1, 1];
        vec.shift_cyclic(0);
        assert_eq!(vec, bitvec![1, 0, 1, 1]);

        let mut vec = bitvec![1, 0, 1, 1, 1];
        vec.shift_cyclic(-1);
        assert_eq!(vec, bitvec![1, 1, 0, 1, 1]);

        let mut vec = bitvec![1, 0, 1, 1, 1];
        vec.shift_cyclic(1);
        assert_eq!(vec, bitvec![0, 1, 1, 1, 1]);
    }

    #[test]
    fn remainder_divide_test_1() {
        let vec = bitvec![1, 1, 0, 0, 1, 1];
        let divisor = bitvec![1, 0, 1, 1];
        let expected = bitvec![0, 1, 0];
        let result = vec.remainder_divide(&divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn remainder_divide_test_2() {
        let vec = bitvec![1, 1, 0, 0, 1, 1];
        let divisor = bitvec![1, 1, 0, 1];
        let expected = bitvec![1, 1, 1];
        let result = vec.remainder_divide(&divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn remainder_divide_test_3() {
        let vec = bitvec![0, 1, 1, 1, 0, 1, 1];
        let divisor = bitvec![1, 0, 1, 1];
        let expected = bitvec![0, 0, 1];
        let result = vec.remainder_divide(&divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn remainder_divide_test_4() {
        let vec = bitvec![1, 0, 0, 1, 0, 0, 0];
        let divisor = bitvec![0, 0, 1, 1];
        let expected = bitvec![0];
        let result = vec.remainder_divide(&divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn truncate_preceding_zeros() {
        let mut vec = bitvec![0, 0, 1, 1];
        let expected = bitvec![1, 1];
        vec.truncate_preceding_zeros();
        assert_eq!(expected, vec);

        let mut vec = bitvec![1, 1];
        let expected = bitvec![1, 1];
        vec.truncate_preceding_zeros();
        assert_eq!(expected, vec);

        let mut vec = bitvec![0, 0];
        let expected = bitvec![];
        vec.truncate_preceding_zeros();
        assert_eq!(expected, vec);
    }
}
