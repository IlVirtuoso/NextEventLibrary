use std::process::Output;

use super::rngs::RandomGenerator;



struct RandomVariable<'generator>{
    stream: usize,
    generator: &'generator mut RandomGenerator,
    func: &'generator dyn Fn() -> f64,
}

impl<'generator> RandomVariable<'generator>{
    pub fn new(stream: usize, generator: &'generator mut RandomGenerator, func: &'generator dyn Fn()->f64) -> Self{
        RandomVariable{
            stream,
            generator,
            func
        }
    }

    pub fn from_static_generator(stream: usize, func:  &'generator dyn Fn()->f64)-> Self{
        RandomVariable::new(stream, RandomGenerator::Global(), func)
    }

    pub fn call(&mut self)->f64{
        self.generator.SelectStream(self.stream);
        (self.func)()
    }

}



#[cfg(test)]
mod tests {
    use crate::Random::rvgs::Exponential;

    use super::*;

    #[test]
    fn test_random_variables() {
        let mut variable = RandomVariable::from_static_generator(1, &||{
            Exponential(10.0)
        });

        for i in 0..10{
            println!("{:?}",variable.call());
            println!("{:?}",RandomGenerator::Global().Random());
        }

    }
}