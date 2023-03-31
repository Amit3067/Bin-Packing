use super::bin::Bin;
pub trait Gene<T, U> where U : Bin<T>{
    fn mutate(&mut self,items: &Vec<T>, bins: &Vec<U>, p_m: f32) -> ();

    fn fitness(&self, items: &Vec<T>, bins: &Vec<U>) -> f32;

    fn new(items: &Vec<T>, bins: &Vec<U>) -> Self;

    fn crossover(&mut self, other : &mut Self);
}