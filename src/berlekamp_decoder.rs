use crate::bch_bitvec::*;
use crate::common;
use crate::decoder::*;
use bitvec::*;

pub struct BerlekampDecoder {
    n: i32,
    k: i32,
    t: i32,
    gen_poly: BitVec,
    adding_table: Vec<Vec<i32>>,
}

impl BerlekampDecoder {
    pub fn new(n: i32, k: i32, t: i32, prime_poly: &BitVec) -> BerlekampDecoder {
        let prime_degree = prime_poly.len() as i32 - 1;
        let (gen_poly, adding_table) =
            common::get_gen_poly_and_adding_table(prime_degree, t, prime_poly);
        common::validate_params(n, k, &gen_poly, prime_poly);
        BerlekampDecoder {
            n: n,
            k: k,
            t: t,
            gen_poly: gen_poly,
            adding_table: adding_table,
        }
    }

    fn compute_syndroms(&self, encoded: &BitVec) -> Vec<BitVec> {
        let mut syndroms: Vec<BitVec> = Vec::new();
        let min_pols = self.get_first_n_min_pols(2 * self.t as u32);
        for m in min_pols {
            syndroms.push(encoded.remainder_divide(&m).unwrap());
        }
        syndroms
    }

    fn get_first_n_min_pols(&self, n: u32) -> Vec<BitVec> {
        let mut min_pols = Vec::new();
        min_pols.resize_default(n as usize);
        let layers = common::get_n_first_layers(n, self.adding_table[0].len());

        //retain unique
        let mut unique_layers = Vec::new();
        for l in layers.iter() {
            if !unique_layers.contains(&l) {
                unique_layers.push(l);
            }
        }

        unique_layers.iter().for_each(|layer| {
            let min_poly = common::calculate_layer_min_pol(layer, &self.adding_table);
            for elem in layer.iter() {
                if *elem <= n {
                    min_pols[*elem as usize - 1] = min_poly.clone();
                }
            }
        });

        min_pols
    }

    fn get_syndroms_alphas(&self, syndroms: &Vec<BitVec>) -> Vec<i32> {
        let mut syndroms_alphas = Vec::new();

        for (i, syndrome) in syndroms.iter().enumerate() {
            let mut alphas_to_add: Vec<i32> = Vec::new();
            for (j, coef) in syndrome.iter().rev().enumerate() {
                if coef == true {
                    alphas_to_add.push((j * (i + 1)) as i32);
                }
            }
            //TODO to doc: -1 means no element
            syndroms_alphas.push(
                alphas_to_add
                    .iter()
                    .fold(-1, |sum, alpha| self.add_alphas(sum, *alpha)),
            );
        }

        syndroms_alphas
    }

    fn add_alphas(&self, a1: i32, a2: i32) -> i32 {
        if a1 == -1 {
            return a2;
        } else if a2 == -1 {
            return a1;
        } else {
            return self.adding_table[a1 as usize][a2 as usize];
        }
    }

    fn init_table(&self, s1: i32) -> (Vec<f32>, Vec<BitVec>, Vec<i32>, Vec<i32>, Vec<i32>) {
        // prepare us
        let mut us = Vec::new();
        us.push(0.5);
        for i in 0..(self.t + 1) {
            us.push(i as f32);
        }

        //prepare sigmas
        let mut sigmas = Vec::new();
        sigmas.push(bitvec![1]);
        sigmas.push(bitvec![1]);

        //prepare dus
        let mut dus = Vec::new();
        dus.push(1);
        dus.push(s1);

        //prepare lus
        let mut lus = Vec::new();
        lus.push(0);
        lus.push(0);

        //prepare last
        let mut last = Vec::new();
        last.push(-1);
        last.push(0);

        (us, sigmas, dus, lus, last)
    }
}

impl Decoder for BerlekampDecoder {
    fn decode(self, encoded: &BitVec) -> Result<(BitVec, BitVec), String> {
        let syndroms = self.compute_syndroms(encoded);
        let syndroms_alphas = self.get_syndroms_alphas(&syndroms);

        //TODO rename last column
        let (us, sigmas, dus, lus, last) = self.init_table(syndroms_alphas[0]);

        println!("{} {} {} {} {}", us[0], sigmas[0], dus[0], lus[0], last[0]);
        println!("{} {} {} {} {}", us[1], sigmas[1], dus[1], lus[1], last[1]);

        unimplemented!()
    }
}
