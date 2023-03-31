extern crate GA;

use GA::traits::bin::Bin;

use GA::structs::solution::Solution;

#[cfg(test)]
mod tests{
    use GA::traits::gene::Gene;

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
    fn bin_can_hold(){
        let bin1 = Sack{cap: 34};

        let item1 = Weight{wt: 23};

        assert_eq!(bin1.can_hold(&vec![&item1]), true);
    }
    
    #[test]
    fn bin_cannot_hold(){
        let bin1 = Sack{cap: 34};

        let item1 = Weight{wt: 235};

        assert_eq!(bin1.can_hold(&vec![&item1]), false);
    }

    #[test]
    fn gene_creation(){
        let bin1 = Sack{cap: 34};
        let bin2 = Sack{cap: 34};

        let item1 = Weight{wt: 23};
        let item2 = Weight{wt:11};

        let mut items = vec![item1, item2];
        let mut bins = vec![bin1, bin2];

        let sol1 = Solution::new(&items, &bins);

        println!("{:?}",sol1);
    }

    
    #[test]
    fn gene_mutation(){
        let bin1 = Sack{cap: 34};
        let bin2 = Sack{cap: 34};

        let item1 = Weight{wt: 23};
        let item2 = Weight{wt:11};

        let mut items = vec![item1, item2];
        let mut bins = vec![bin1, bin2];

        let mut sol1 = Solution::new(&items, &bins);

        println!("Before Mutation\n{:?}",sol1);

        sol1.mutate(&items, &bins,0.5);

        println!("After Mutation\n{:?}",sol1);
    }

    #[test]
    fn gene_fitness(){
        let bin1 = Sack{cap: 34};
        let bin2 = Sack{cap: 34};

        let item1 = Weight{wt: 23};
        let item2 = Weight{wt:11};

        let mut items = vec![item1, item2];
        let mut bins = vec![bin1, bin2];

        let mut sol1 = Solution::new(&items, &bins);
        println!("Before Mutation\n{:?}\nFitness: {}",sol1, sol1.fitness(&items, &bins));

        sol1.mutate(&items, &bins,0.5);

        println!("After Mutation\n{:?}\nFitness: {}",sol1, sol1.fitness(&items, &bins));
    }
}