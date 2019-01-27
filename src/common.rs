use crate::bitvec_operations;
use bitvec::*;
use itertools::Itertools;
use primes::{is_prime, PrimeSet};
use std::collections::VecDeque;

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

fn create_adding_table(alphas: &Vec<BitVec>) -> Vec<Vec<i32>> {
    let mut adding_table: Vec<Vec<i32>> = Vec::new();
    let alphas_size: usize = alphas.len();

    for row in 0..alphas_size {
        let mut row_vec: Vec<i32> = Vec::with_capacity(alphas_size);

        for col in 0..alphas_size {
            let alpha_row = alphas.get(row).unwrap();
            let alpha_col = alphas.get(col).unwrap();
            let alphas_xor = (*alpha_row).clone() ^ (*alpha_col).clone();
            let result_alpha_idx = alphas.iter().position(|alpha| alpha.eq(&alphas_xor));
            match result_alpha_idx {
                Some(idx) => row_vec.push(idx as i32),
                None => row_vec.push(-1), // TODO maybe change to Option
            }
        }

        adding_table.push(row_vec);
    }

    adding_table
}

pub fn create_gen_pol(degree: u32, t: u32, adding_table: &Vec<Vec<i32>>) -> BitVec {
    let mut min_pols = Vec::new();
    let layers: Vec<Vec<u32>> = get_n_layers(t, adding_table[0].len());
    layers
        .iter()
        .for_each(|layer| min_pols.push(calculate_layer_min_pol(layer, degree, adding_table)));

    bitvec_operations::finite_multiply_bitvecs(&min_pols)
}

fn calculate_layer_min_pol(layer: &Vec<u32>, degree: u32, adding_table: &Vec<Vec<i32>>) -> BitVec {
    let mut min_pol = bitvec![0; (degree + 1) as usize];
    min_pol.set(0, true);
    min_pol.set(degree as usize, true);

    for idx in 1..degree {
        let combinations = layer.iter().combinations(idx as usize).collect_vec();
        let coefficient = combinations.iter().fold(-1, |sum, combination| {
            let alpha = combination
                .iter()
                .fold(0, |sum, &&alpha| (sum + alpha) % (2u32.pow(degree) - 1));
            if sum != -1i32 {
                return adding_table[sum as usize][alpha as usize];
            } else {
                return alpha as i32;
            }
        });
        if coefficient != -1 {
            min_pol.set(idx as usize, true);
        }
    }

    min_pol
}

fn get_n_layers(n: u32, alphas_len: usize) -> Vec<Vec<u32>> {
    let mut layers: Vec<Vec<u32>> = Vec::new();
    let mut prime_set = PrimeSet::new();
    let mut first_primes: VecDeque<u64> = prime_set.iter().take(alphas_len as usize).collect();
    first_primes.push_front(1);

    for _i in 0..n {
        let mut layer: Vec<u32> = Vec::new();
        layer.push(first_primes.pop_front().unwrap() as u32 % (alphas_len - 1) as u32);

        loop {
            let candidate =
                (*layer.iter_mut().last().unwrap() * 2) as u32 % (alphas_len - 1) as u32;
            if layer.contains(&candidate) {
                layer.sort();
                layers.push(layer);
                break;
            } else {
                if is_prime(candidate as u64) {
                    first_primes.retain(|&item| item as u32 != candidate);;
                }
                layer.push(candidate);
            }
        }
    }

    layers
}

pub fn validate_params(n: i32, k: i32, gen_poly: &BitVec, prime_poly: &BitVec) {
    if gen_poly.len() == 0 || n != k + gen_poly.len() as i32 - 1 || gen_poly[0] == false || prime_poly[0] == false {
        panic!(
            "Bad coder parameters. n: {}, k: {}, gen: {:?}",
            n, k, gen_poly
        );
    }
}

pub fn get_gen_poly(degree: i32, t: i32, prime_poly: &BitVec) -> BitVec {
    let alphas = calculate_alphas(&prime_poly);
    let adding_table = create_adding_table(&alphas);
    create_gen_pol(degree as u32, t as u32, &adding_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_gen_poly_test() {
        let prime_poly = bitvec![1, 0, 1, 1];
        let degree = 3;
        let t = 2;
        let result = get_gen_poly(degree, t, &prime_poly);
        let expected = bitvec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn get_gen_poly_test_2() {
        let prime_poly = bitvec![1, 0, 1, 1];
        let degree = 3;
        let t = 1;
        let result = get_gen_poly(degree, t, &prime_poly);
        let expected = bitvec![1, 0, 1, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn create_gen_pol_test() {
        let alphas = calculate_alphas(&bitvec![1, 0, 1, 1]);
        let adding_table = create_adding_table(&alphas);
        let expected = bitvec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(expected, create_gen_pol(3, 2, &adding_table));
    }

    #[test]
    fn calculate_alphas_test() {
        let alphas = calculate_alphas(&bitvec![1, 0, 1, 1]);
        let expected = vec![
            bitvec![0, 0, 0, 1],
            bitvec![0, 0, 1, 0],
            bitvec![0, 1, 0, 0],
            bitvec![0, 0, 1, 1],
            bitvec![0, 1, 1, 0],
            bitvec![0, 1, 1, 1],
            bitvec![0, 1, 0, 1],
            bitvec![0, 0, 0, 1],
        ];
        assert_eq!(alphas, expected);
    }

    #[test]
    fn create_adding_table_test() {
        let alphas = calculate_alphas(&bitvec![1, 0, 1, 1]);
        let result = create_adding_table(&alphas);
        let expected = vec![
            vec![-1, 3, 6, 1, 5, 4, 2, -1],
            vec![3, -1, 4, 0, 2, 6, 5, 3],
            vec![6, 4, -1, 5, 1, 3, 0, 6],
            vec![1, 0, 5, -1, 6, 2, 4, 1],
            vec![5, 2, 1, 6, -1, 0, 3, 5],
            vec![4, 6, 3, 2, 0, -1, 1, 4],
            vec![2, 5, 0, 4, 3, 1, -1, 2],
            vec![-1, 3, 6, 1, 5, 4, 2, -1],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn get_n_layers_test() {
        let param: u32 = 3;
        let layers = get_n_layers(param, 2i32.pow(param) as usize);
        let expected = vec![vec![1, 2, 4], vec![3, 5, 6], vec![0]];
        assert_eq!(layers, expected);
    }

    #[test]
    fn get_n_layers_test_2() {
        let param: u32 = 5;
        let layers = get_n_layers(param, 2i32.pow(param) as usize);
        let expected = vec![
            vec![1, 2, 4, 8, 16],
            vec![3, 6, 12, 17, 24],
            vec![5, 9, 10, 18, 20],
            vec![7, 14, 19, 25, 28],
            vec![11, 13, 21, 22, 26],
        ];
        assert_eq!(layers, expected);
    }
}
