pub trait Bin<T>{
    fn can_hold(&self, items: &Vec<&T>) -> bool;

    fn cost(&self) -> u32;
}

#[cfg(test)]
mod tests{
    use super::*;

    struct Weight{
        wt: u32
    }

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
}