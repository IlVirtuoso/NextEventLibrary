use super::{rngs::RandomGenerator, rvgs::Uniform};
use std::process::Output;

struct RandomVariable<'generator> {
    stream: usize,
    generator: &'generator mut RandomGenerator,
    func: &'generator dyn Fn() -> f64,
}

impl<'generator> RandomVariable<'generator> {
    pub fn new(
        stream: usize,
        generator: &'generator mut RandomGenerator,
        func: &'generator dyn Fn() -> f64,
    ) -> Self {
        RandomVariable {
            stream,
            generator,
            func,
        }
    }

    pub fn from_static_generator(stream: usize, func: &'generator dyn Fn() -> f64) -> Self {
        RandomVariable::new(stream, RandomGenerator::Global(), func)
    }

    pub fn call(&mut self) -> f64 {
        self.generator.SelectStream(self.stream);
        (self.func)()
    }
}

fn selector(weights: &Vec<f64>, generator: &mut RandomGenerator)-> usize{
    
    let mut a  = vec![0.0;weights.len()];
    a[0] =  weights[0];
    for i in 1..a.len(){
        a[i] = a[i-1]+  weights[i];
    }
    let y =  generator.Random();
    let mut r = 0;
    while y>= a[r]{
        r+=1;
    }
    r
}

struct Chooser<'generator, T> {
    stream: usize,
    generator: &'generator mut RandomGenerator,
    weights: Vec<f64>,
    func: Vec<&'generator dyn Fn() -> T>,
}

impl<'generator, T> Chooser<'generator, T> {
    pub fn new(
        stream: usize,
        generator: &'generator mut RandomGenerator,
        weights: Vec<f64>,
        func: Vec<&'generator dyn Fn() -> T>,
    ) -> Self {
        debug_assert!(weights.iter().sum::<f64>() == 1.0, "Sum of weights should be 1");
        Chooser {
            stream,
            generator,
            weights,
            func,
        }
    }


    pub fn from_static_generator(stream: usize, weights: Vec<f64>,func: Vec<&'generator dyn Fn() -> T>)->Self{
        Chooser::new(stream, RandomGenerator::Global(), weights, func)
    }

    pub fn call(&mut self) -> T{
        self.generator.SelectStream(self.stream);
        let selected = selector(&self.weights, &mut self.generator);
        self.func[selected]()
    }
    
}

#[cfg(test)]
mod tests {
    use crate::Random::rvgs::Exponential;

    use super::*;

    #[test]
    fn test_random_variables() {
        let mut variable = RandomVariable::from_static_generator(2, &|| Exponential(100.0));
        let mut value: f64 = 0.0;
        for i in 0..10000 {
            value += variable.call();
        }
        println!("Mean:{}", value / 10000.0);
    }

    #[test]
    fn test_selector(){
        let mut chooser = Chooser::<u32>::from_static_generator(1, vec![0.3,0.7], vec![&||{0},&||{1}]);
        let mut a:[u32;2]= [0;2];        
        for _i in 0..100{
            a[chooser.call() as usize] += 1;
        }
        println!("{:?}",a);

    }
}
