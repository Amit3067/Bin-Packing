use rand::{thread_rng, Rng};

use super::super::traits::gene::Gene;
use super::super::traits::bin::Bin;
use std::{collections::{HashSet, HashMap}, ops::Not};

#[derive(Debug)]
pub struct Solution{
    gene: Vec<usize>
}

impl Clone for Solution{
    fn clone(&self) -> Self {
        Self { 
            gene: self.gene.iter().map(|v| *v).collect()
        }
    }
}

impl<'a, T, U> Gene< T, U> for Solution where U : Bin<T>{

    fn mutate(&mut self, items: &Vec<T>, bins: &Vec<U>, p_m: f32) -> (){
        for allele in &mut self.gene {
            let pr = thread_rng().gen_bool(p_m as f64);

            if pr{
                *allele = thread_rng().gen_range(0, bins.len());
            }
        }
    }

    fn fitness(&self, items: &Vec<T>, bins: &Vec<U>) -> f32{
        
        let mut mp = HashMap::<usize, Vec<&T>>::new();

        let mut cur = 0;

        while cur < self.gene.len() {
            mp.entry(self.gene[cur]).or_insert(Vec::<&T>::new()).push(&items[cur]);
            cur += 1;
        }

        for k in mp.keys(){
            if bins[*k].can_hold(&mp[k]).not() {
                return 0.0;
            }
        }
        
        return 1.0 / (mp.len() as f32);
    }

    fn new(items: &Vec<T>, bins: &Vec<U>) -> Self{
        let item_cnt = items.len();
        let bin_cnt = bins.len();
        Solution {
            gene: (0..item_cnt).map(|v| thread_rng().gen_range(0, bin_cnt)).collect()
        }
    }

    
    fn crossover(&mut self, other : &mut Self) {
        let sz = self.gene.len();
        let mut ind : usize = 0;

        while ind < sz {
            let p1 = self.gene[ind];
            let p2 = other.gene[ind];

            let pr_1 = thread_rng().gen_bool(0.5);
            let pr_2 = thread_rng().gen_bool(0.5);

            if pr_1 {
                self.gene[ind] = p2;
            }
            else{
                self.gene[ind] = p1;
            }

            if pr_2 {
                other.gene[ind] = p1; 
            }
            else{
                other.gene[ind] = p2;
            }

            ind+=1;
        }
    }
}