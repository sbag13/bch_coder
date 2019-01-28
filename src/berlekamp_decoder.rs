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

    fn init_table(&self, s1: i32) -> (Vec<f32>, Vec<Vec<i32>>, Vec<i32>, Vec<i32>, Vec<i32>) {
        // prepare us
        let mut us = Vec::new();
        us.push(-0.5);
        for i in 0..(self.t + 1) {
            us.push(i as f32);
        }

        //prepare sigmas
        let mut sigmas = Vec::new();
        sigmas.push(vec![0]);
        sigmas.push(vec![0]);

        //prepare dus
        let mut dus = Vec::new();
        dus.push(0);
        dus.push(s1);

        //prepare lus
        let mut lus = Vec::new();
        lus.push(0);
        lus.push(0);

        //prepare dulus
        let mut dulus = Vec::new();
        dulus.push(-1);
        dulus.push(0);

        (us, sigmas, dus, lus, dulus)
    }

    fn find_dulu_idx(&self, dus: &Vec<i32>, dulus: &Vec<i32>) -> usize {
        let (val, idx) = dulus
            .iter()
            .take(dulus.len() - 1)
            .enumerate()
            .map(|(idx, val)| (val, idx))
            .max()
            .unwrap();
        idx
    }

    fn alpha_mod_n(&self, alpha: i32) -> i32 {
        let mut res = alpha % self.n;
        if res < 0 {
            res += self.n;
        }
        res
    }

    //TODO to doc: alphas in sigmas cant be negative
    fn multiply_sigma_by_alpha_with_x(
        &self,
        sigma: &Vec<i32>,
        x_power: i32,
        x_alpha: i32,
    ) -> Vec<i32> {
        let mut result = vec![-1; sigma.len() + x_power as usize];
        for (i, alpha) in sigma.iter().rev().enumerate() {
            let new_alpha;
            if *alpha != -1 {
                new_alpha = self.alpha_mod_n(x_alpha + *alpha);
            } else {
                new_alpha = -1;
            }
            *result.iter_mut().rev().nth(x_power as usize + i).unwrap() = new_alpha;
        }
        result
    }

    fn add_to_sigma(&self, sigma: Vec<i32>, to_add: Vec<i32>) -> Vec<i32> {
        let (mut longer, shorter) = if sigma.len() >= to_add.len() {
            (sigma, to_add)
        } else {
            (to_add, sigma)
        };
        for (longer_el, shorter_el) in longer.iter_mut().rev().zip(shorter.iter().rev()) {
            if *longer_el == -1 {
                *longer_el = *shorter_el;
            } else if *shorter_el != -1 {
                *longer_el = self.alpha_mod_n(self.add_alphas(*longer_el, *shorter_el));
            }
        }
        longer
    }
}

impl Decoder for BerlekampDecoder {
    fn decode(self, encoded: &BitVec) -> Result<(BitVec, BitVec), String> {
        let syndroms = self.compute_syndroms(encoded);
        let syndroms_alphas = self.get_syndroms_alphas(&syndroms);

        let (us, mut sigmas, mut dus, mut lus, mut dulus) = self.init_table(syndroms_alphas[0]);

        let mut u_idx: usize = 1;
        loop {
            println_layers(u_idx as i32 + 1, &us, &sigmas, &dus, &lus, &dulus);

            let u = us[u_idx];
            if dus[u as usize] == -1 {

            } else {
                let most_positive_dulu_idx = self.find_dulu_idx(&dus, &dulus);
                let up = us[most_positive_dulu_idx];

                let sigma_u = sigmas[u_idx].clone();
                let du = dus[u_idx];
                let dp_inv = dus[most_positive_dulu_idx] * -1;
                let x_power = (2 as f32 * (u as f32 - up)) as i32;
                let sigma_p = sigmas[most_positive_dulu_idx].clone();

                let x_alpha = self.alpha_mod_n(du + dp_inv);

                let mut x_poly: Vec<i32> = vec![-1; x_power as usize + 1];
                *x_poly.iter_mut().rev().nth(x_power as usize).unwrap() = x_alpha;

                let x_poly_sigma_p =
                    self.multiply_sigma_by_alpha_with_x(&sigma_p, x_power, x_alpha);
                let next_sigma = self.add_to_sigma(sigma_u, x_poly_sigma_p);
                sigmas.push(next_sigma.clone());

                if u as i32 + 1 == self.t {
                    break;
                }

                let next_lu = next_sigma.len() as i32 - 1;
                let next_dulu = 2 * (u as i32 + 1) - next_lu;
                lus.push(next_lu);
                dulus.push(next_dulu);

                let L = lus[u_idx] + 1;
                let mut alphas_to_add = Vec::new();
                for i in 0..(L + 1) {
                    if i == 0 {
                        alphas_to_add.push(syndroms_alphas[2 * u as usize + 2]); //TODO maybe f32
                    } else {
                        alphas_to_add.push(self.alpha_mod_n(
                            syndroms_alphas[2 * u as usize + 2 - i as usize]
                                + next_sigma.iter().rev().nth(i as usize).unwrap(),
                        ));
                    }
                }
                let next_du = alphas_to_add
                    .iter()
                    .fold(-1, |sum, alpha| self.add_alphas(sum, *alpha));
                dus.push(next_du);
            }
            u_idx += 1;
        }

        println!("last sigma {:?}", sigmas.iter().last().unwrap());
        let final_err_locator_poly = sigmas.iter().last().unwrap();
        unimplemented!()
    }
}

fn println_layers(
    n: i32,
    us: &Vec<f32>,
    sigmas: &Vec<Vec<i32>>,
    dus: &Vec<i32>,
    lus: &Vec<i32>,
    dulus: &Vec<i32>,
) {
    for i in 0..n as usize {
        println!(
            "{} {:?} {} {} {} ",
            us[i], sigmas[i], dus[i], lus[i], dulus[i]
        );
    }
    println!("###");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_sigma_by_alpha_with_x_test() {
        let decoder = BerlekampDecoder::new(31, 16, 3, &bitvec![1, 0, 0, 1, 0, 1]);

        let sigma = vec![0];
        let result = decoder.multiply_sigma_by_alpha_with_x(&sigma, 1, 2);
        assert_eq!(result, vec![2, -1]);

        let sigma = vec![2, 0];
        let result = decoder.multiply_sigma_by_alpha_with_x(&sigma, 2, 25);
        assert_eq!(result, vec![27, 25, -1, -1]);

        let sigma = vec![25, 26, -1, 3];
        let result = decoder.multiply_sigma_by_alpha_with_x(&sigma, 1, 25);
        assert_eq!(result, vec![19, 20, -1, 28, -1]);
    }

    #[test]
    fn add_to_sigma_test() {
        let decoder = BerlekampDecoder::new(31, 16, 3, &bitvec![1, 0, 0, 1, 0, 1]);

        let sigma = vec![24, 2, 0];
        let to_add = vec![27, 25, -1, -1];
        let result = decoder.add_to_sigma(sigma, to_add);
        assert_eq!(result, vec![27, 11, 2, 0]);
    }
}
