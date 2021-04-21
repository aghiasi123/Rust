extern crate rand;
use magic_num::magic_number;
use rand::Rng;


fn  main() {
        let mut rng = rand::thread_rng();

        let num = magic_num::magic_number();
       
        println!("{}, {}",rng.gen_range(0..256), rng.gen_range(0..256)); // 0-255
        println!("{}, {}",rng.gen_range(0..232), rng.gen_range(0..232)); // 0-231
}

