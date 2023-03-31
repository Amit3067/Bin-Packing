use crate::traits::bin::Bin;
use crate::traits::gene::Gene;
use crate::structs::solution::Solution;
use crate::structs::config::Config;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Algorithm<T, U, S = Solution> where U : Bin<T>, S: Debug + Gene<T, U>{
    config: Config,
    bins: Vec<U>,
    items: Vec<T>,
    population: Vec<S>
}

impl<T, U, S> Algorithm<T, U, S> where U : Bin<T>, S: Debug + Clone + Gene<T, U>{

    pub fn new(config: Config) -> Result<Self, &'static str>{
        
        Ok(Algorithm {
            config,
            items: Vec::<T>::new(),
            bins: Vec::<U>::new(),
            population: Vec::<S>::new()
        })

    }

    pub fn add_item(&mut self, item : T) -> () {
        self.items.push(item);
    }

    pub fn add_bin(&mut self, bin : U) -> () {
        self.bins.push(bin);
    }

    pub fn run(&mut self) -> S {
        self.init();
        let mut cur_iter = 0;
        while cur_iter < self.config.get_max_iter() {
            // println!("{:?}",self.population);
            let mut mating_pool = self.reproduce();
            mating_pool.pop();
            mating_pool.push(self.get_best());
            self.crossover(&mut mating_pool);
            self.mutate(&mut mating_pool);
            self.population = mating_pool;
            cur_iter += 1;
        }
        self.get_best()
    }

    fn init(&mut self) -> () {
        self.population.clear();
        self.population = (0..self.config.get_p_size()).map(|v| S::new(&self.items, &self.bins)).collect();
    }

    fn reproduce(&mut self) -> Vec<S> {
        let fitness = self.population_fitness();
        let mut ind : Vec<u32> = (0..self.population.len() as u32).collect();

        ind.sort_by(|a, b| fitness[*a as usize].partial_cmp(&fitness[*b as usize]).unwrap());
        
        let sel_pr : Vec<f32> = self.generate_sel_prob();

        let mut pool = Vec::<S>::new();

        while pool.len() < ind.len(){
            let rnd : f32 = thread_rng().gen_range(0.0,1.0);

            let elem_ind = match sel_pr.binary_search_by(|prob| prob.partial_cmp(&rnd).unwrap()) {
                Ok(i) => i,
                Err(e) => e
            };

            pool.push(self.population[elem_ind].clone());
        }

        return pool;
    }

    fn generate_sel_prob(&self) -> Vec<f32>{
        let s = 0.5;
        let mu = self.population.len() as f32;
        let mut sel_pr : Vec<f32> = (0..self.population.len() as u32).map(|i| {
            (2.0-s) / mu + 2.0 * i as f32 * (s-1.0) / (mu * (mu - 1.0))
        }).collect();

        for i in 1..sel_pr.len() {
            sel_pr[i] = sel_pr[i-1] + sel_pr[i];
        }

        return sel_pr;
    }

    fn population_fitness(&self) -> Vec<f32>{
        self.population.iter().map(|s| s.fitness(&self.items, &self.bins)).collect()
    }

    fn mutate(&mut self, pool: &mut Vec<S>) {
        for individual in pool {
            individual.mutate(&self.items, &self.bins, self.config.get_pr_m());
        }
    }

    fn crossover(&mut self, pool: &mut Vec<S>) {
        let mut ind : Vec<usize> = (0..pool.len()).collect();

        ind.shuffle(&mut thread_rng());

        let mut i = 0;

        while i+1  < pool.len() {
            let mut p1 = pool[i].clone();
            let mut p2 = pool[i+1].clone();
            p1.crossover(&mut p2);
            pool[i] = p2;
            pool[i+1] = p1;
            i += 2;
        }
    }

    fn get_best(&mut self) -> S {
        let mut best = self.population[0].clone();
        let mut fitness  = best.fitness(&self.items, &self.bins);

        for individual in &self.population{
            let cur_fitness = individual.fitness(&self.items, &self.bins);
            if cur_fitness > fitness {
                fitness = cur_fitness;
                best = individual.clone();
            }
        }
        return best;
    }
}