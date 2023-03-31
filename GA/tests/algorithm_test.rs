extern crate GA;

use GA::traits::bin::Bin;

use GA::structs::solution::Solution;
use GA::structs::config::Config;
use GA::algorithm::Algorithm;

#[cfg(test)]
mod tests{
    use GA::{traits::gene::Gene, structs::config};

    use super::*;

    #[derive(Debug)]
    struct Weight{
        wt: u32
    }

    #[derive(Debug)]
    struct Sack{
        cap: u32
    }

    impl Bin<Weight> for Sack{
        fn can_hold(&self, items: &Vec<&Weight>) -> bool{
            let mut sum = 0;
            for i in items{
                sum += i.wt;
            }
            return sum <= self.cap;
        }
        fn cost(&self) -> u32{
            return 1;
        }
    }


    #[test]
    fn algorithm_test() {
        let mut config = Config::new(10, 0.3, 10);

        let mut my_algo = Algorithm::<Weight, Sack>::new(config).unwrap();


        my_algo.add_item(Weight { wt: 12 });
        my_algo.add_item(Weight { wt: 15 });
        my_algo.add_item(Weight { wt: 4 });
        my_algo.add_item(Weight { wt: 2 });

        my_algo.add_bin(Sack { cap: 10 });
        my_algo.add_bin(Sack { cap: 20 });
        my_algo.add_bin(Sack { cap: 30 });


        println!("{:?}",my_algo);

        println!("Solution {:?}",my_algo.run());
    }
}