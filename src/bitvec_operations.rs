use bitvec::*;

// pub trait BCHBitVec {
//     fn finite_multiply_bitvecs(vec: &Vec<BitVec>) -> BitVec;
//     fn truncate(vec: BitVec) -> BitVec;
// } 

pub fn finite_multiply_bitvecs(vec: &Vec<BitVec>) -> BitVec {
    vec.iter().fold(bitvec![1], |folded, pol| {
        let mut to_add: Vec<BitVec> = Vec::new();
        for (i, bit) in pol.iter().rev().enumerate() {
            if bit == true {
                let mut elem: BitVec = folded.clone();
                elem.extend(bitvec![0;i]);
                to_add.push(elem);
            }
        }
        to_add.iter().fold(bitvec![0], |mut sum, element| {
            if sum.len() < element.len() {
                sum >>= element.len() - sum.len();
            }
            sum ^ element.clone()
        })
    })
}

fn truncate(vec: BitVec) -> BitVec {
    //TODO change to mut ref
    let pos = vec.iter().position(|bit| bit == true);
    match pos {
        Some(i) => return vec.iter().skip(i).collect(),
        None => return bitvec![],
    }
}

pub fn remainder_divide(dividend_orig: &BitVec, divisor_orig: &BitVec) -> Result<BitVec, String> {
    let mut dividend = dividend_orig.clone();
    let mut divisor = divisor_orig.clone();

    // truncate preceding zeros
    // TODO extract to fn
    loop {
        if dividend[0] == true {
            break;
        }
        dividend <<= 1;
    }
    loop {
        if divisor[0] == true {
            break;
        }
        divisor <<= 1;
    }

    if divisor.len() == 0 {
        return Err("Division by zero polynomial!".to_owned());
    }
    if divisor.len() > dividend.len() {
        return Ok(dividend.clone());
    }

    let mut remainder: BitVec = dividend.iter().take(divisor.len()).collect();
    remainder ^= divisor.clone();

    for bit in dividend.iter().skip(divisor.len()) {
        remainder <<= 1;
        remainder.push(bit);
        if remainder.get(0) == true {
            remainder ^= divisor.clone();
        }
    }
    remainder <<= 1;
    Ok(remainder)
}

pub fn shift_cyclic(vec: &mut BitVec, n: i32) {
    let tmp_clone = vec.clone();
    let len = tmp_clone.len() as i32;
    for i in 0..vec.len() {
        let mut pos = (i as i32 + n) % len;
        if pos < 0 {
            pos += len;
        }
        let bit = tmp_clone.get(pos as usize);
        vec.set(i as usize, bit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_cyclic_test() {
        let mut vec = bitvec![1, 0, 1, 1];
        shift_cyclic(&mut vec, 2);
        assert_eq!(vec, bitvec![1, 1, 1, 0]);

        let mut vec = bitvec![1, 0, 1, 1];
        shift_cyclic(&mut vec, 4);
        assert_eq!(vec, bitvec![1, 0, 1, 1]);

        let mut vec = bitvec![1, 0, 1, 1];
        shift_cyclic(&mut vec, 0);
        assert_eq!(vec, bitvec![1, 0, 1, 1]);

        let mut vec = bitvec![1, 0, 1, 1, 1];
        shift_cyclic(&mut vec, -1);
        assert_eq!(vec, bitvec![1, 1, 0, 1, 1]);

        let mut vec = bitvec![1, 0, 1, 1, 1];
        shift_cyclic(&mut vec, 1);
        assert_eq!(vec, bitvec![0, 1, 1, 1, 1]);
    }

    #[test]
    fn finite_multiply_bitvecs_test() {
        let to_multiply = vec![bitvec![1, 0, 1, 1, 0], bitvec![1, 1, 0, 1], bitvec![1, 1]];
        let expected = bitvec![1, 0, 0, 0, 0, 0, 0, 1, 0];
        let result = finite_multiply_bitvecs(&to_multiply);
        assert_eq!(expected, result);
    }

    #[test]
    fn remainder_divide_test_1() {  //TODO rename tests
        let dividend = bitvec![1, 1, 0, 0, 1, 1];
        let divisor = bitvec![1, 0, 1, 1];
        let expected = bitvec![0, 1, 0];
        let result = remainder_divide(&dividend, &divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn remainder_divide_test_2() {
        let dividend = bitvec![1, 1, 0, 0, 1, 1];
        let divisor = bitvec![1, 1, 0, 1];
        let expected = bitvec![1, 1, 1];
        let result = remainder_divide(&dividend, &divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn remainder_divide_test_3() {
        let dividend = bitvec![0, 1, 1, 1, 0, 1, 1];
        let divisor = bitvec![1, 0, 1, 1];
        let expected = bitvec![0, 0, 1];
        let result = remainder_divide(&dividend, &divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn remainder_divide_test_4() {
        let dividend = bitvec![1, 0, 0, 1, 0, 0, 0];
        let divisor = bitvec![1, 0, 1, 1];
        let expected = bitvec![1, 1, 0];
        let result = remainder_divide(&dividend, &divisor).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn truncate_test() {
        let vec = bitvec![0, 0, 1, 1];
        let expected = bitvec![1, 1];
        let result = truncate(vec);
        assert_eq!(expected, result);

        let vec = bitvec![1, 1];
        let expected = bitvec![1, 1];
        let result = truncate(vec);
        assert_eq!(expected, result);

        let vec = bitvec![0, 0];
        let expected = bitvec![];
        let result = truncate(vec);
        assert_eq!(expected, result);
    }
}
