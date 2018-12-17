extern crate bitvec;

use bitvec::*;

// const POWER: usize = 9;
// const TWO_TO_POWER: usize = 512;

fn main() {
}

fn calculate_alphas(prime_polynomial: &BitVec) -> Vec<BitVec> {
    let mut alphas: Vec<BitVec> = Vec::new();
    let pol_size = prime_polynomial.len();
    let alphas_count = 2u32.pow(pol_size as u32 - 1);

    // push alpha 0
    let mut alpha_0 = bitvec![0; pol_size];
    alpha_0.set(pol_size - 1, true);
    alphas.push(alpha_0);

    // push rest
    for i in 1..(alphas_count as usize) {
        let mut alpha_i = alphas.get(i - 1).unwrap().clone();
        alpha_i <<= 1;
        alpha_i.push(false);
        if alpha_i.get(0) == true {
            alpha_i ^= (*prime_polynomial).clone();
        }
        alphas.push(alpha_i);
    }
    alphas
}

fn create_adding_table(alphas: Vec<BitVec>) -> Vec<Vec<BitVec>> {
    let mut adding_table: Vec<Vec<BitVec>> = Vec::new();
    let alphas_size: usize = alphas.len();

    for row in 0..alphas_size {
        let mut row_vec: Vec<BitVec> = Vec::with_capacity(alphas_size);

        for col in 0..alphas_size {
            let alpha_row = alphas.get(row).unwrap();
            let alpha_col = alphas.get(col).unwrap();
            let alphas_xor = (*alpha_row).clone() ^ (*alpha_col).clone();
            row_vec.push(alphas_xor);
        }

        adding_table.push(row_vec);
    }

    adding_table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_alphas_test() {
        let alphas = calculate_alphas(&bitvec![1,0,1,1]);
        let expected = vec![
            bitvec![0,0,0,1],
            bitvec![0,0,1,0],
            bitvec![0,1,0,0],
            bitvec![0,0,1,1],
            bitvec![0,1,1,0],
            bitvec![0,1,1,1],
            bitvec![0,1,0,1],
            bitvec![0,0,0,1]
        ];
        assert_eq!(alphas, expected);
    }

    #[test]
    fn create_adding_table_test() {
        let alphas = calculate_alphas(&bitvec![1,0,1,1]);
        let result = create_adding_table(alphas);
        let expected = vec![
            vec![ bitvec![0,0,0,0],  bitvec![0,0,1,1],  bitvec![0,1,0,1],  bitvec![0,0,1,0],  bitvec![0,1,1,1],  bitvec![0,1,1,0],  bitvec![0,1,0,0],  bitvec![0,0,0,0]],
            vec![ bitvec![0,0,1,1],  bitvec![0,0,0,0],  bitvec![0,1,1,0],  bitvec![0,0,0,1],  bitvec![0,1,0,0],  bitvec![0,1,0,1],  bitvec![0,1,1,1],  bitvec![0,0,1,1]],
            vec![ bitvec![0,1,0,1],  bitvec![0,1,1,0],  bitvec![0,0,0,0],  bitvec![0,1,1,1],  bitvec![0,0,1,0],  bitvec![0,0,1,1],  bitvec![0,0,0,1],  bitvec![0,1,0,1]],
            vec![ bitvec![0,0,1,0],  bitvec![0,0,0,1],  bitvec![0,1,1,1],  bitvec![0,0,0,0],  bitvec![0,1,0,1],  bitvec![0,1,0,0],  bitvec![0,1,1,0],  bitvec![0,0,1,0]],
            vec![ bitvec![0,1,1,1],  bitvec![0,1,0,0],  bitvec![0,0,1,0],  bitvec![0,1,0,1],  bitvec![0,0,0,0],  bitvec![0,0,0,1],  bitvec![0,0,1,1],  bitvec![0,1,1,1]],
            vec![ bitvec![0,1,1,0],  bitvec![0,1,0,1],  bitvec![0,0,1,1],  bitvec![0,1,0,0],  bitvec![0,0,0,1],  bitvec![0,0,0,0],  bitvec![0,0,1,0],  bitvec![0,1,1,0]],
            vec![ bitvec![0,1,0,0],  bitvec![0,1,1,1],  bitvec![0,0,0,1],  bitvec![0,1,1,0],  bitvec![0,0,1,1],  bitvec![0,0,1,0],  bitvec![0,0,0,0],  bitvec![0,1,0,0]],
            vec![ bitvec![0,0,0,0],  bitvec![0,0,1,1],  bitvec![0,1,0,1],  bitvec![0,0,1,0],  bitvec![0,1,1,1],  bitvec![0,1,1,0],  bitvec![0,1,0,0],  bitvec![0,0,0,0]]
        ];
        assert_eq!(result, expected);
    }
}
