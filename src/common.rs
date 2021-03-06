use crate::bch_bitvec::*;
use bitvec::*;
use itertools::Itertools;
use primes::{is_prime, PrimeSet};
use std::collections::VecDeque;
use rand::Rng;

pub fn get_random_places(n: i32, modulo: i32) ->Vec<usize> {
    let mut nums = Vec::new();
    for i in 0..n {
        loop {
            let num = rand::thread_rng().gen_range(0, modulo) as usize;
            if !nums.contains(&num) {
                nums.push(num); 
                break;
            }
        }
    }
    nums
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

fn create_adding_table(alphas: &Vec<BitVec>) -> Vec<Vec<i32>> {
    //TODO to doc: organization of table
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
                None => row_vec.push(-1),
            }
        }

        adding_table.push(row_vec);
    }

    adding_table
}

fn create_gen_pol(degree: u32, t: u32, adding_table: &Vec<Vec<i32>>) -> BitVec {
    let mut min_pols = Vec::new();
    let layers: Vec<Vec<u32>> = get_n_disjunctive_layers(t, adding_table[0].len());
    layers.iter().for_each(|layer| {
        min_pols.push(calculate_layer_min_pol(layer, adding_table));
    });
    finite_multiply_bitvecs_vec(&min_pols)
}

pub fn calculate_layer_min_pol(layer: &Vec<u32>, adding_table: &Vec<Vec<i32>>) -> BitVec {
    let layer_degree = layer.len() as u32;
    let mut min_pol = bitvec![0; (layer_degree + 1) as usize];
    min_pol.set(0, true);
    min_pol.set(layer_degree as usize, true);

    for idx in 1..layer_degree {
        let combinations = layer.iter().combinations(idx as usize).collect_vec();
        let coefficient = combinations.iter().fold(-1, |sum, combination| {
            let alpha = combination.iter().fold(0, |sum, &&alpha| {
                (sum + alpha) % (2u32.pow(layer_degree) - 1)
            });
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

pub fn get_n_first_layers(n: u32, alphas_len: usize) -> Vec<Vec<u32>> {
    let mut layers: Vec<Vec<u32>> = Vec::new();

    for i in 1..(n + 1) {
        let mut layer = Vec::new();
        layer.push(i);
        loop {
            let candidate_alpha =
                (*layer.iter_mut().last().unwrap() * 2) as u32 % (alphas_len - 1) as u32;
            if layer.contains(&candidate_alpha) {
                layer.sort();
                layers.push(layer);
                break;
            } else {
                layer.push(candidate_alpha);
            }
        }
    }

    layers
}

fn get_n_disjunctive_layers(n: u32, alphas_len: usize) -> Vec<Vec<u32>> {
    //TODO to doc: layers start with prime powers
    let mut layers: Vec<Vec<u32>> = Vec::new();

    let mut start_numbers: VecDeque<u32> = VecDeque::new();
    for i in 0..(alphas_len / 2) as usize {
        start_numbers.push_back(1 + i as u32 * 2);
    }

    for _i in 0..n {
        if *start_numbers.front().unwrap() > 2 * n {
            break;
        }
        let mut layer: Vec<u32> = Vec::new();
        layer.push(start_numbers.pop_front().unwrap() % (alphas_len as u32 - 1));
        loop {
            let candidate = (layer.iter().last().unwrap() * 2) % (alphas_len as u32 - 1);
            if layer.contains(&candidate) {
                layer.sort();
                layers.push(layer);
                break;
            } else {
                layer.push(candidate);
                start_numbers.retain(|n| *n != candidate);
            }
        }
    }
    layers
}

pub fn validate_params(n: i32, k: i32, gen_poly: &BitVec, prime_poly: &BitVec) {
    if gen_poly.len() == 0
        // || n != k + gen_poly.len() as i32 - 1
        || gen_poly[0] == false
        || prime_poly[0] == false
    {
        panic!(
            "Bad coder parameters. n: {}, k: {}, gen: {:?}",
            n, k, gen_poly
        );
    }
}

pub fn get_gen_poly(degree: i32, t: i32, prime_poly: &BitVec) -> BitVec {
    //TODO move to classes
    let alphas = calculate_alphas(&prime_poly);
    let adding_table = create_adding_table(&alphas);
    let gen_poly = create_gen_pol(degree as u32, t as u32, &adding_table);
    gen_poly
}

pub fn get_gen_poly_and_adding_table(
    degree: i32,
    t: i32,
    prime_poly: &BitVec,
) -> (BitVec, Vec<Vec<i32>>) {
    //TODO move to classes
    let alphas = calculate_alphas(&prime_poly);
    let adding_table = create_adding_table(&alphas);
    let gen_poly = create_gen_pol(degree as u32, t as u32, &adding_table);
    (gen_poly, adding_table)
}

pub fn finite_multiply_bitvecs_vec(vec: &Vec<BitVec>) -> BitVec {
    vec.iter().fold(bitvec![1], |folded, pol| {
        let mut to_add: Vec<BitVec> = Vec::new();
        for (i, bit) in pol.iter().rev().enumerate() {
            if bit == true {
                let mut elem: BitVec = folded.clone();
                elem.extend(bitvec![0;i]);
                to_add.push(elem);
            }
        }
        to_add
            .iter()
            .fold(bitvec![0], |mut sum, element| sum.finite_add(element))
    })
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
    fn get_n_disjunctive_layers_test() {
        let param: u32 = 3;
        let layers = get_n_disjunctive_layers(param, 2i32.pow(param) as usize);
        let expected = vec![vec![1, 2, 4], vec![3, 5, 6]];
        assert_eq!(layers, expected);
    }

    #[test]
    fn get_n_disjunctive_layers_test_2() {
        let param: u32 = 5;
        let layers = get_n_disjunctive_layers(param, 2i32.pow(param) as usize);
        let expected = vec![
            vec![1, 2, 4, 8, 16],
            vec![3, 6, 12, 17, 24],
            vec![5, 9, 10, 18, 20],
            vec![7, 14, 19, 25, 28],
        ];
        assert_eq!(layers, expected);
    }

    #[test]
    fn finite_multiply_bitvecs_test() {
        let to_multiply = vec![bitvec![1, 0, 1, 1, 0], bitvec![1, 1, 0, 1], bitvec![1, 1]];
        let expected = bitvec![1, 0, 0, 0, 0, 0, 0, 1, 0];
        let result = finite_multiply_bitvecs_vec(&to_multiply);
        assert_eq!(expected, result);
    }

    #[test]
    fn finite_multiply_bitvecs_vec_test() {
        let to_multiply = vec![
            bitvec![1, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            bitvec![1, 0, 0, 1, 1, 1, 1, 1, 0, 1],
            bitvec![1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
            bitvec![1, 1, 0, 1, 0, 0, 1, 0, 0, 1],
            bitvec![1, 1, 0, 1, 1, 0, 1, 0, 1, 1],
            bitvec![1, 0, 1, 0, 1, 1, 0, 1, 1, 1],
            bitvec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            bitvec![1, 1, 1, 1, 1, 0, 1, 0, 0, 1],
            bitvec![1, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            bitvec![1, 0, 1, 1, 0, 0, 1, 1, 1, 1],
            bitvec![1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            bitvec![1, 0, 1, 0, 1, 0, 0, 0, 1, 1],
            bitvec![1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
            bitvec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1],
            bitvec![1, 0, 1, 0, 0, 0, 0, 1, 1, 1],
            bitvec![1, 0, 0, 1, 0, 1, 1, 1, 1, 1],
            bitvec![1, 0, 1, 0, 0, 1, 1, 0, 0, 1],
            bitvec![1, 0, 0, 0, 1, 0, 1, 1, 0, 1],
            bitvec![1, 0, 0, 1, 1, 0, 1, 1, 1, 1],
        ];
        let expected = bitvec![
            1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1,
            1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1,
            1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1,
            1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1,
            0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1
        ];
        let result = finite_multiply_bitvecs_vec(&to_multiply);
        assert_eq!(expected, result);
    }
}
