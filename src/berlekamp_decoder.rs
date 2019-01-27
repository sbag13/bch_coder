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

    fn compute_syndroms(self, encoded: &BitVec) -> Vec<BitVec> {
        let mut syndroms: Vec<BitVec> = Vec::new();
        let min_pols = self.get_first_n_min_pols(2 * self.t as u32);
        for m in min_pols {
            syndroms.push(encoded.remainder_divide(&m).unwrap());
        }
        println!("{:?}", syndroms);
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
            let min_poly =  common::calculate_layer_min_pol(layer, &self.adding_table);
            for elem in layer.iter() {
                if *elem <= n {
                    min_pols[*elem as usize - 1] = min_poly.clone();
                }
            }
        });

        min_pols
    }
}

impl Decoder for BerlekampDecoder {
    fn decode(self, encoded: &BitVec) -> Result<(BitVec, BitVec), String> {
        let syndroms = self.compute_syndroms(encoded);

        unimplemented!()
    }
}
