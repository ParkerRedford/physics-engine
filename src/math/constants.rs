pub mod constants {
    pub struct Constants {
        pub E: f64,
        pub PI: f32
    }

    impl Constants {
        pub fn new() -> Self {
            let mut e: f64 = 0.0;
            for n in 0..20 {
                e += 1.0f64 / factorial(n) as f64;
            }

            Constants {
                E: e,
                PI: 3.14
            }
        }
    }

    pub fn factorial(n: i64) -> i64 {
        if n == 0 { return 1; }

        n * factorial(n - 1)
    }

    fn precomputed_factorial(n: i32) -> i64 {
        const FACT: [i64; 21] = [
            1, 1, 2, 6, 24, 120,
            720, 5040, 40320, 362880,
            3628800, 39916800, 479001600,
            6227020800, 87178291200, 1307674368000,
            20922789888000, 355687428096000,
            6402373705728000, 121645100408832000,
            2432902008176640000,
        ];
        FACT[n as usize]
    }

    fn eulerconstant() {
        
    }
}