/* --------------------------------------------------------------------------
 * This is a RUST library for generating random variates from six discrete
 * distributions
 *
 *      Generator         Range (x)     Mean         Variance
 *
 *      Bernoulli(p)      x = 0,1       p            p*(1-p)
 *      Binomial(n, p)    x = 0,...,n   n*p          n*p*(1-p)
 *      Equilikely(a, b)  x = a,...,b   (a+b)/2      ((b-a+1)*(b-a+1)-1)/12
 *      Geometric(p)      x = 0,...     p/(1-p)      p/((1-p)*(1-p))
 *      Pascal(n, p)      x = 0,...     n*p/(1-p)    n*p/((1-p)*(1-p))
 *      Poisson(m)        x = 0,...     m            m
 *
 * and seven continuous distributions
 *
 *      Uniform(a, b)     a < x < b     (a + b)/2    (b - a)*(b - a)/12
 *      Exponential(m)    x > 0         m            m*m
 *      Erlang(n, b)      x > 0         n*b          n*b*b
 *      Normal(m, s)      all x         m            s*s
 *      Lognormal(a, b)   x > 0            see below
 *      Chisquare(n)      x > 0         n            2*n
 *      Student(n)        all x         0  (n > 1)   n/(n - 2)   (n > 2)
 *
 * For the a Lognormal(a, b) random variable, the mean and variance are
 *
 *                        mean = exp(a + 0.5*b*b)
 *                    variance = (exp(b*b) - 1) * exp(2*a + b*b)
 *
 * Name              : rvgs.c  (Random Variate GeneratorS)
 * Author            : Steve Park & Dave Geyer
 * Implementor       : Matteo Ielacqia
 * Language          : RUST
 * Latest Revision   : 30/08/2023
 * --------------------------------------------------------------------------
 */



use super::rngs::{self, RandomGenerator};

fn Random()-> f64{
    RandomGenerator::Global().Random()
}

/** ========================================================
 * Returns 1 with probability p or 0 with probability 1 - p.
 * NOTE: use 0.0 < p < 1.0                                   
 * ========================================================
 */
pub fn Bernoulli(p: f64) -> u32 {
    let r = Random();
    if r < (1.0 - p) {
        0
    } else {
        1
    }
}

/** ================================================================
 * Returns a binomial distributed integer between 0 and n inclusive.
 * NOTE: use n > 0 and 0.0 < p < 1.0
 * ================================================================
 */
pub fn Binomial(n: u32, p: f64) -> u32 {
    let mut x: u32 = 0;
    for i in 0..n {
        x += Bernoulli(p);
    }
    x
}

pub fn Geometric(p: f64) -> u32 {
    ((1.0 - Random()).ln() / p.ln()) as u32
}

pub fn Pascal(n: u32, p: f64) -> u32 {
    let mut x: u32 = 0;
    for i in 0..n {
        x += Geometric(p);
    }
    x
}

//also called negative exponential
pub fn Exponential(m: f64) -> f64 {
    -m * (1.0 - Random()).ln()
}

pub fn Poisson(m: f64) -> u32 {
    let mut t = 0.0;
    let mut x = 0;
    while t < m {
        t += Exponential(1.0);
        x += 1;
    }
    x - 1
}

pub fn Uniform(a: f64, b: f64) -> f64 {
    a + (b - a) * Random()
}

pub fn Erlang(n: u32, b: f64) -> f64 {
    let mut x = 0.0;
    for i in 0..n {
        x += Exponential(b);
    }
    x
}

pub fn Normal(m: f64, s: f64) -> f64 {
    const p0: f64 = 0.322232431088;
    const q0: f64 = 0.099348462606;
    const p1: f64 = 1.0;
    const q1: f64 = 0.588581570495;
    const p2: f64 = 0.342242088547;
    const q2: f64 = 0.531103462366;
    const p3: f64 = 0.204231210245e-1;
    const q3: f64 = 0.103537752850;
    const p4: f64 = 0.453642210148e-4;
    const q4: f64 = 0.385607006340e-2;

    let (mut u, mut t, mut p, mut q, mut z) = (0.0, 0.0, 0.0, 0.0, 0.0);
    u = Random();
    if u < 0.5 {
        t = (-2.0 * u.ln()).sqrt();
    } else {
        t = (-2.0 * u.ln()).sqrt();
    }

    p   = p0 + t * (p1 + t * (p2 + t * (p3 + t * p4)));
    q   = q0 + t * (q1 + t * (q2 + t * (q3 + t * q4)));

    if u< 0.5{
        z=(p/q)-t;
    }
    else{
        z=t-(p/q);
    }
    m + s * z
}


pub fn Lognormal(a:f64, b:f64)->f64{
    (a+b*Normal(0.0, 1.0)).exp()
}

pub fn Chisquare(n:u32) -> f64{
    let (mut z, mut x) = (0.0,0.0);
    for i in 0..n{
        z= Normal(0.0, 1.0);
        x += z*z;
    }
    x
}

pub fn Student(n: u32) -> f64{
    Normal(0.0, 1.0)/ (Chisquare(n)/n as f64).sqrt()
}

