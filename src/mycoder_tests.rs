#[cfg(test)]
mod tests {
    use crate::bch_bitvec::*;
    use crate::berlekamp_decoder::BerlekampDecoder;
    use crate::decoder::Decoder;
    use crate::encoder::Encoder;
    use crate::simple_decoder::SimpleDecoder;
    use crate::common::get_random_places;
    use bitvec::*;
    use rand::prelude::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};

    static n: i32 = 255;
    static k: i32 = 123;
    static t: i32 = 19;

    fn get_prime_poly() -> BitVec {
        bitvec![1, 0, 1, 1, 0, 0, 1, 0, 1]
    }

    fn get_gen_poly() -> BitVec {
        bitvec![
            1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1,
            1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1
        ]
    }

    fn get_encoder() -> Encoder {
        Encoder::new(n, k, t, &get_prime_poly())
    }

    fn get_ready_encoder() -> Encoder {
        Encoder::new_with_gen_poly(n, k, t, get_gen_poly())
    }

    fn get_simple_decoder() -> SimpleDecoder {
        SimpleDecoder::new(n, k, t, &get_prime_poly())
    }

    fn get_ready_simple_decoder() -> SimpleDecoder {
        SimpleDecoder::new_with_gen_poly(n, k, t, get_gen_poly())
    }

    fn get_berlekamp_decoder() -> BerlekampDecoder {
        BerlekampDecoder::new(n, k, t, &get_prime_poly())
    }

    // fn get_ready_berlekamp_decoder() -> BerlekampDecoder {
    //     BerlekampDecoder::new_with_gen_poly(n, k, t, get_gen_poly(), get_adding_table())
    // }

    fn generate_random_msg_and_code_word() -> (BitVec, BitVec) {
        let mut msg = BitVec::new();
        for _i in 0..k {
            msg.push(rand::random());
        }
        // let encoder = get_encoder();
        let encoder = get_ready_encoder();
        let word = encoder.encode(&msg).unwrap();
        (msg, word)
    }

    #[test]
    #[ignore]
    fn all_single_errors_simple_decoder() {
        let (msg, code_word) = generate_random_msg_and_code_word(); //TODO save coders to file
        let simple = get_ready_simple_decoder();

        let mut time: Duration = Duration::new(0, 0);
        for i in 0..255 {
            println!("i: {}", i);
            let mut code_word_clone = code_word.clone();
            code_word_clone.inverse_nth(i);
            let start = Instant::now();
            let (decoded, _) = simple.decode(&code_word_clone).unwrap();
            let stop = Instant::now();
            time += stop.duration_since(start);
            assert_eq!(msg, decoded);
        }

        println!("Decoding took: {:?}", time);
        println!(
            "Mean time {:?}",
            time / code_word.len() as u32
        );
    }

    #[test]
    #[ignore]
    fn all_double_errors_together_simple_decoder() {
        let (msg, code_word) = generate_random_msg_and_code_word(); //TODO save coders to file
        let simple = get_ready_simple_decoder();

        let mut time: Duration = Duration::new(0, 0);
        for i in 0..255 {
            println!("i: {}", i);
            let mut code_word_clone = code_word.clone();
            code_word_clone.inverse_nth(i);
            if i != 254 {
                code_word_clone.inverse_nth(i + 1);
            } else {
                code_word_clone.inverse_nth(0);
            }
            let start = Instant::now();
            let (decoded, _) = simple.decode(&code_word_clone).unwrap();
            let stop = Instant::now();
            time += stop.duration_since(start);
            assert_eq!(msg, decoded);
        }
        
        println!("Decoding took: {:?}", time);
        println!(
            "Mean time {:?}",
            time / code_word.len() as u32
        );
    }

    #[test]
    #[ignore]
    fn all_19_errors_together_simple_decoder() {
        let (msg, code_word) = generate_random_msg_and_code_word(); //TODO save coders to file
        let simple = get_ready_simple_decoder();

        let mut time: Duration = Duration::new(0, 0);
        for i in 0..255 {
            println!("i: {}", i);
            let mut code_word_clone = code_word.clone();
            code_word_clone.inverse_nth(i);
            if i <= 254 - 18 {
                for j in 0..19 {
                    code_word_clone.inverse_nth(i + j);
                }
            } else {
                for j in i..255 {
                    code_word_clone.inverse_nth(j);
                }
                for j in 0..(19 - (255 - i)) {
                    code_word_clone.inverse_nth(j);
                }
            }
            let start = Instant::now();
            let (decoded, _) = simple.decode(&code_word_clone).unwrap();
            let stop = Instant::now();
            time += stop.duration_since(start);
            assert_eq!(msg, decoded);
        }
        
        println!("Decoding took: {:?}", time);
        println!(
            "Mean time {:?}",
            time / code_word.len() as u32
        );
    }

    #[test]
    #[ignore]
    fn _200_double_errors_random_simple_decoder() {
        n_itr_random(2, 200);
    }

    #[test]
    #[ignore]
    fn all_double_errors_with_one_error_constant_simple_decoder() {
        let (msg, code_word) = generate_random_msg_and_code_word(); //TODO save coders to file
        let simple = get_ready_simple_decoder();

        let mut time: Duration = Duration::new(0, 0);
        let mut ok = 0;
        let mut fail = 0;
        for i in 1..255 {
            println!("i: {}", i);
            let mut code_word_clone = code_word.clone();
            code_word_clone.inverse_nth(0);
            code_word_clone.inverse_nth(i);

            let start = Instant::now();
            let result = simple.decode(&code_word_clone);
            let stop = Instant::now();
            time += stop.duration_since(start);
            match result {
                Ok((decoded, _)) => {
                    if decoded == msg {
                        ok += 1;
                    }
                },
                Err(_) => {
                    fail += 1;
                }
            }
        }
        
        println!("Decoding took: {:?}", time);
        println!(
            "Mean time {:?}",
            time / 254 as u32
        );
        println!("ok: {}, fail: {}", ok, fail);
    }

    #[test]
    #[ignore]
    fn _200_triple_random_errors_simple_decoder() {
        n_itr_random(3, 200);
    }

    #[test]
    #[ignore]
    fn _200_x7_random_errors_simple_decoder() {
        n_itr_random(7, 200);
    }

    #[test]
    #[ignore]
    fn _200_x5_random_errors_simple_decoder() {
        n_itr_random(5, 200);
    }

    #[test]
    #[ignore]
    fn _50_x20_random_errors_simple_decoder() {
        n_itr_random(20, 50);
    }

    #[test]
    #[ignore]
    fn _200_x8_random_errors_simple_decoder() {
        n_itr_random(8, 200);
    }

    #[test]
    #[ignore]
    fn _200_x10_random_errors_simple_decoder() {
        n_itr_random(10, 200);
    }

    fn n_itr_random(e: i32, itr: i32) {
        let (msg, code_word) = generate_random_msg_and_code_word(); //TODO save coders to file
        let simple = get_ready_simple_decoder();

        let mut time: Duration = Duration::new(0, 0);
        let mut ok = 0;
        let mut fail = 0;
        for i in 0..itr {
            println!("i: {}", i);
            let mut code_word_clone = code_word.clone();
            let places = get_random_places(e, 255);
            for p in places {
                code_word_clone.inverse_nth(p);
            }
            let start = Instant::now();
            let result = simple.decode(&code_word_clone);
            let stop = Instant::now();
            time += stop.duration_since(start);
            match result {
                Ok((decoded, _)) => {
                    if decoded == msg {
                        ok += 1;
                    }
                },
                Err(_) => {
                    fail += 1;
                }
            }
        }
        
        println!("Decoding took: {:?}", time);
        println!(
            "Mean time {:?}",
            time / itr as u32
        );
        println!("ok: {}, fail: {}", ok, fail);
    }
}
