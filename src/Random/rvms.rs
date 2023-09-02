
/* ------------------------------------------------------------------------- 
 * This is an ANSI C library that can be used to evaluate the probability 
 * density functions (pdf's), cumulative distribution functions (cdf's), and 
 * inverse distribution functions (idf's) for a variety of discrete and 
 * continuous random variables.
 *
 * The following notational conventions are used
 *                 x : possible value of the random variable
 *                 u : real variable (probability) between 0.0 and 1.0 
 *  a, b, n, p, m, s : distribution-specific parameters
 *
 * There are pdf's, cdf's and idf's for 6 discrete random variables
 *
 *      Random Variable    Range (x)  Mean         Variance
 *
 *      Bernoulli(p)       0..1       p            p*(1-p)
 *      Binomial(n, p)     0..n       n*p          n*p*(1-p)
 *      Equilikely(a, b)   a..b       (a+b)/2      ((b-a+1)*(b-a+1)-1)/12 
 *      Geometric(p)       0...       p/(1-p)      p/((1-p)*(1-p))
 *      Pascal(n, p)       0...       n*p/(1-p)    n*p/((1-p)*(1-p))
 *      Poisson(m)         0...       m            m
 *
 * and for 7 continuous random variables
 *
 *      Uniform(a, b)      a < x < b  (a+b)/2      (b-a)*(b-a)/12
 *      Exponential(m)     x > 0      m            m*m
 *      Erlang(n, b)       x > 0      n*b          n*b*b
 *      Normal(m, s)       all x      m            s*s
 *      Lognormal(a, b)    x > 0         see below
 *      Chisquare(n)       x > 0      n            2*n
 *      Student(n)         all x      0  (n > 1)   n/(n-2)   (n > 2)
 *
 * For the Lognormal(a, b), the mean and variance are
 *
 *                        mean = Exp(a + 0.5*b*b)
 *                    variance = (Exp(b*b) - 1)*Exp(2*a + b*b)
 *
 * Name            : rvms.c (Random Variable ModelS)
 * Author          : Steve Park & Dave Geyer
 * Language        : ANSI C
 * Latest Revision : 11-22-97
 * Translated by : Matteo Ielacqua
 * Language : Rust
 * ------------------------------------------------------------------------- 
 */

use std::f64::consts::PI;

use once_cell::sync::Lazy;


 pub const TINY:f64 =     1.0e-10;
 pub const SQRT2PI : Lazy<f64> = Lazy::new(|| (2.0*PI).sqrt());               /* sqrt(2 * pi) */
 

 
    pub fn pdfBernoulli(p : f64, x: u32)->f64
 /* =======================================
  * NOTE: use 0.0 < p < 1.0 and 0 <= x <= 1
  * =======================================
  */
 {
    return (if(x == 0) {1.0 - p } else{p});
 }
 
    pub fn cdfBernoulli(p: f64, x : u32) -> f64
 /* =======================================
  * NOTE: use 0.0 < p < 1.0 and 0 <= x <= 1 
  * =======================================
  */
 {
    return (if(x == 0) {1.0 - p }else{ 1.0});
 }
 
    pub fn idfBernoulli(p: f64, u: f64) -> u32
 /* =========================================
  * NOTE: use 0.0 < p < 1.0 and 0.0 < u < 1.0 
  * =========================================
  */
 {
    return (1 - !(u < 1.0 - p) as u32);
 }
 
    pub fn pdfEquilikely( a : u32,  b : u32,  x : u32) -> f64
 /* ============================================ 
  * NOTE: use a <= x <= b 
  * ============================================
  */
 {
    return (1.0 / ((b - a) as f64 + 1.0));
 }
 
    pub fn cdfEquilikely( a:u32,  b:u32,  x:u32) -> f64
 /* ============================================
  * NOTE: use a <= x <= b 
  * ============================================
  */
 {
    return (((x - a) as f64 + 1.0) / ((b - a) as f64 + 1.0));
 }
 
  pub fn idfEquilikely( a:u32,  b:u32, u:f64) -> u32
 /* ============================================ 
  * NOTE: use a <= b and 0.0 < u < 1.0 
  * ============================================
  */
 {
    return (a as u32 + (u as u32 * (b - a + 1) ));
 }
 
    pub fn pdfBinomial( n:u32, p:f64,  x:u32)->f64 /* ============================================ 
  * NOTE: use 0 <= x <= n and 0.0 < p < 1.0 
  * ============================================
  */
 {
    let (mut s, mut t) =(0.0,0.0);
 
    s = LogChoose(n as f64, x as f64) ;
    t = x as f64* (p).ln() + (n - x) as f64 * (1.0 - p).ln();
    return ((s + t).exp());
 }
 
    pub fn cdfBinomial( n:u32, p: f64,  x:u32) -> f64
 /* ============================================ 
  * NOTE: use 0 <= x <= n and 0.0 < p < 1.0 
  * ============================================
  */
 {
    if (x < n){1.0 - InBeta((x + 1) as f64, (n - x) as f64, p)}
    else{(1.0)}
 }
 
    pub fn idfBinomial( n:u32,  p:f64, u:f64)->u32
 /* ================================================= 
  * NOTE: use 0 <= n, 0.0 < p < 1.0 and 0.0 < u < 1.0 
  * =================================================
  */
 {
    let mut x =(n as f64 * p) as u32;             /* start searching at the mean */
 
    if cdfBinomial(n, p, x) <= u{
      while (cdfBinomial(n, p, x) <= u){
        x+=1;
      }
    }
    else if (cdfBinomial(n, p, 0) <= u){
      while (cdfBinomial(n, p, x - 1) > u){
        x-=1;
      }
    }
    else{
      x = 0;
    }
    x
 }
 
    pub fn pdfGeometric(p:f64, x:u32) -> f64
 /* ===================================== 
  * NOTE: use 0.0 < p < 1.0 and x >= 0 
  * =====================================
  */
 {
    return ((1.0 - p) * (x as f64 * (p).ln()).exp());
 }
 
    pub fn cdfGeometric(p:f64, x:u32) -> f64
 /* ===================================== 
  * NOTE: use 0.0 < p < 1.0 and x >= 0 
  * =====================================
  */
 {
    return (1.0 - ((x + 1) * (p).ln()).exp());
 }
 
pub fn idfGeometric( p:f64,  u:f64)-> u32
 /* ========================================= 
  * NOTE: use 0.0 < p < 1.0 and 0.0 < u < 1.0 
  * =========================================
  */
 {
    return (((1.0 - u).ln() / (p).ln()));
 }
 
    pub fn pdfPascal(n: u32, p:f64, x:u32)-> f64
 /* =========================================== 
  * NOTE: use n >= 1, 0.0 < p < 1.0, and x >= 0 
  * ===========================================
  */
 {
    let (mut s, mut t) = (0.0,0.0);
 
    s = LogChoose(n + x - 1, x);
    t = x * (p).ln() + n * (1.0 - p).ln();
    return ((s + t).exp());
 }
 
    pub fn cdfPascal(n: u32, p: f64, x:u32)-> f64
 /* =========================================== 
  * NOTE: use n >= 1, 0.0 < p < 1.0, and x >= 0 
  * ===========================================
  */
 {
    return (1.0 - InBeta(x + 1, n, p));
 }
 
    pub fn idfPascal(n: u32, p : f64, u: f64) -> u32
 /* ================================================== 
  * NOTE: use n >= 1, 0.0 < p < 1.0, and 0.0 < u < 1.0 
  * ==================================================
  */
 {
    let mut x = (n as f64 * p / (1.0 - p));    /* start searching at the mean */
 
    if cdfPascal(n, p, x) <= u{
      while (cdfPascal(n, p, x) <= u){
        x+=1;
      }
    }
    else if (cdfPascal(n, p, 0) <= u){
      while (cdfPascal(n, p, x - 1) > u){
        x+=1;
      }
    }
    else{
      x = 0;
    }
    return (x);
 }
 
    pub fn pdfPoisson(m: f64, x : u32)-> f64
 /* ===================================
  * NOTE: use m > 0 and x >= 0 
  * ===================================
  */
 {
    let mut t = 0.0;
 
    t = - m + x * (m).ln() - LogFactorial(x);
    return (exp(t));
 }
 
    pub fn cdfPoisson(m:f64, x:u32)->f64
 /* =================================== 
  * NOTE: use m > 0 and x >= 0 
  * ===================================
  */
 {
    return (1.0 - InGamma(x + 1, m));
 }
 
    pub fn idfPoisson(m:f64, u: f64)-> u32
 /* =================================== 
  * NOTE: use m > 0 and 0.0 < u < 1.0 
  * ===================================
  */
 {
    let mut x = m as u32;                    /* start searching at the mean */
 
    if (cdfPoisson(m, x) <= u){
      while (cdfPoisson(m, x) <= u){
        x+=1;
      }
    }
    else if (cdfPoisson(m, 0) <= u){
      while (cdfPoisson(m, x - 1) > u){
        x-=1;
      }
    }
    else{
      x = 0;
    }
    return (x);
 }
 
    pub fn pdfUniform( a: f64, b:f64, x:f64)-> f64
 /* =============================================== 
  * NOTE: use a < x < b 
  * ===============================================
  */
 {
    return (1.0 / (b - a));
 }
 
  pub fn cdfUniform( a:f64,  b:f64,  x:f64)-> f64
 /* =============================================== 
  * NOTE: use a < x < b 
  * ===============================================
  */
 {
    return ((x - a) / (b - a));
 }
 
  pub fn dfUniform( a:f64,  b:f64,  u:f64)->f64
 /* =============================================== 
  * NOTE: use a < b and 0.0 < u < 1.0 
  * ===============================================
  */
 {
    return (a + (b - a) * u);
 }
 
    pub fn pdfExponential( m:f64,  x:f64)->f64
 /* ========================================= 
  * NOTE: use m > 0 and x > 0 
  * =========================================
  */
 {
    return ((1.0 / m) * (- x / m).exp());
 }
 
    pub fn cdfExponential( m:f64,  x:f64) -> f64
 /* ========================================= 
  * NOTE: use m > 0 and x > 0 
  * =========================================
  */
 {
    return (1.0 - (- x / m).exp());
 }
 
    pub fn idfExponential( m:f64,  u:f64)->f64
 /* ========================================= 
  * NOTE: use m > 0 and 0.0 < u < 1.0 
  * =========================================
  */
 {
    return (- m * (1.0 - u).ln());
 }
 
    pub fn pdfErlang(n:u32,  b:f64,  x:f64) -> f64
 /* ============================================ 
  * NOTE: use n >= 1, b > 0, and x > 0 
  * ============================================
  */
 {
    let mut t = 0.0;
 
    t = (n - 1) * (x / b).ln() - (x / b) - (b).ln() - LogGamma(n);
    return ((t).exp());
 }
 
    pub fn cdfErlang(n: u32,  b:f64,  x:f64) -> f64
 /* ============================================ 
  * NOTE: use n >= 1, b > 0, and x > 0 
  * ============================================
  */
 {
    return (InGamma(n, x / b));
 }
 
  pub fn idfErlang(n: u32, b:f64, u: f64) -> f64
 /* ============================================ 
  * NOTE: use n >= 1, b > 0 and 0.0 < u < 1.0 
  * ============================================
  */
 {
    let (mut t, mut x) = (0.0,n as f64 * b);                   /* initialize to the mean, then */
 
    loop {                                   /* use Newton-Raphson iteration */
      t = x;
      x = t + (u - cdfErlang(n, b, t)) / pdfErlang(n, b, t);
      if (x <= 0.0){
        x = 0.5 * t;
      }
    if !((x - t).abs() >= TINY){break;}}
    return (x);
 }
 
    pub fn pdfStandard(x:f64)->f64
 /* =================================== 
  * NOTE: x can be any value 
  * ===================================
  */
 {
    return ((- 0.5 * x * x).exp() / SQRT2PI);
 }
 
    pub fn cdfStandard(x:f64)->f64
 /* =================================== 
  * NOTE: x can be any value 
  * ===================================
  */
 { 
    let mut t = 0.0;
 
    t = InGamma(0.5, 0.5 * x * x);
    if (x < 0.0){
      return (0.5 * (1.0 - t));
    }
    else{
      return (0.5 * (1.0 + t));
    }
 }
 
    pub fn idfStandard(u:f64)-> f64
 /* =================================== 
  * NOTE: 0.0 < u < 1.0 
  * ===================================
  */
 { 
    let (mut t, mut x) = (0.0,0.0);                    /* initialize to the mean, then  */
 
    loop{                                  /* use Newton-Raphson iteration  */
      t = x;
      x = t + (u - cdfStandard(t)) / pdfStandard(t);
     if !((x - t).abs() >= TINY){break;}
  }
    return (x);
 }
 
    pub fn pdfNormal( m:f64,  s:f64,  x:f64)->f64
 /* ============================================== 
  * NOTE: x and m can be any value, but s > 0.0 
  * ==============================================
  */
 { 
    let t = (x - m) / s;
 
    return (pdfStandard(t) / s);
 }
 
  pub fn cdfNormal( m:f64,  s:f64,  x:f64)->f64
 /* ============================================== 
  * NOTE: x and m can be any value, but s > 0.0 
  * ==============================================
  */
 { 
    let t = (x - m) / s;
 
    return (cdfStandard(t));
 }
 
    pub fn idfNormal( m:f64,  s:f64,  u:f64)->f64
 /* ======================================================= 
  * NOTE: m can be any value, but s > 0.0 and 0.0 < u < 1.0 
  * =======================================================
  */
 {
    return (m + s * idfStandard(u));
 }
 
    pub fn pdfLognormal( a:f64,  b:f64,  x:f64)->f64
 /* =================================================== 
  * NOTE: a can have any value, but b > 0.0 and x > 0.0 
  * ===================================================
  */
 { 
    let t = ((x).ln() - a) / b;
    return (pdfStandard(t) / (b * x));
 }
 
    pub fn cdfLognormal( a: f64,  b: f64,  x: f64) ->f64
 /* =================================================== 
  * NOTE: a can have any value, but b > 0.0 and x > 0.0 
  * ===================================================
  */
 { 
    let mut  t = ((x).ln() - a) / b;
 
    return (cdfStandard(t));
 }
 
    pub fn idfLognormal( a: f64,  b: f64,  u: f64) -> f64 /* ========================================================= 
  * NOTE: a can have any value, but b > 0.0 and 0.0 < u < 1.0 
  * =========================================================
  */
 { 
    let mut t = 0.0;
 
    t = a + b * idfStandard(u);
    return ((t).exp());
 }
 
    pub fn pdfChisquare( n: u32, x : f64)-> f64
 /* ===================================== 
  * NOTE: use n >= 1 and x > 0.0 
  * =====================================
  */
 { 
    let (t, s )= (0.0,n as f64/ 2.0);
 
    t = (s - 1.0) * (x / 2.0).ln() - (x / 2.0) - (2.0).ln() - LogGamma(s);
    return ((t).exp());
 }
 
    pub fn cdfChisquare( n: u32, x: f64) -> f64
 /* ===================================== 
  * NOTE: use n >= 1 and x > 0.0 
  * =====================================
  */
 {
    return (InGamma(n as f64 / 2.0, x as f64 / 2));
 }
 
    pub fn idfChisquare(n: u32, u: f64) -> f64
 /* ===================================== 
  * NOTE: use n >= 1 and 0.0 < u < 1.0 
  * =====================================
  */
 { 
    let (mut t, mut x) = (0.0,n);                         /* initialize to the mean, then */
 
    loop {                                     /* use Newton-Raphson iteration */
      t = x;
      x = t + (u - cdfChisquare(n, t)) / pdfChisquare(n, t);
      if (x <= 0.0){
        x = 0.5 * t;
     if !((x - t).abs() >= TINY){break;}
      }
    return (x);
 }
}
 
    pub fn pdfStudent( n:u32, x: f64) -> f64
 /* =================================== 
  * NOTE: use n >= 1 and x > 0.0 
  * ===================================
  */
 { 
    let (mut s, mut t) = (0.0,0.0); 
    s = -0.5 * (n + 1) * (1.0 + ((x * x) / n as f64).ln());
    t = -LogBeta(0.5, n / 2.0);
    return ((s + t).exp() / ( n as f64).sqrt());
 }
 
    pub fn cdfStudent(n:f64, x:f64)-> f64
 /* =================================== 
  * NOTE: use n >= 1 and x > 0.0 
  * ===================================
  */
 { 
    let (mut s, mut t) = (0.0,0.0);
 
    t = (x * x) / (n + x * x);
    s = InBeta(0.5, n / 2.0, t);
    if (x >= 0.0){
      return (0.5 * (1.0 + s));
    }
    else{
      return (0.5 * (1.0 - s));
    }
 }
 
    pub fn idfStudent(n:u32, u:f64) -> f64
 /* =================================== 
  * NOTE: use n >= 1 and 0.0 < u < 1.0 
  * ===================================
  */
 { 
    let (mut t,mut x) = (0.0,0.0);                       /* initialize to the mean, then */
 
    loop {                                     /* use Newton-Raphson iteration */
      t = x;
      x = t + (u - cdfStudent(n, t)) / pdfStudent(n, t);
     if((x - t).abs() >= TINY){break;}
    }
    return x;
 }
 
 /* ===================================================================
  * The six functions that follow are a 'special function' mini-library
  * used to support the evaluation of pdf, cdf and idf functions.
  * ===================================================================
  */
 
    pub fn LogGamma(a:f64)-> f64
 /* ======================================================================== 
  * LogGamma returns the natural log of the gamma function.
  * NOTE: use a > 0.0 
  *
  * The algorithm used to evaluate the natural log of the gamma function is
  * based on an approximation by C. Lanczos, SIAM J. Numerical Analysis, B,
  * vol 1, 1964.  The constants have been selected to yield a relative error
  * which is less than 2.0e-10 for all positive values of the parameter a.    
  * ======================================================================== 
  */
 { 
    let mut s:[f64;6] = [f64;6];
    let (mut sum,mut temp) = (0.0,0.0);
    s[0] =  76.180091729406 / a;
    s[1] = -86.505320327112 / (a + 1.0);
    s[2] =  24.014098222230 / (a + 2.0);
    s[3] =  -1.231739516140 / (a + 3.0);
    s[4] =   0.001208580030 / (a + 4.0);
    s[5] =  -0.000005363820 / (a + 5.0);
    sum  =   1.000000000178;
    for i in 0..6{ 
      sum += s[i];
    }
    temp = (a - 0.5) * log(a + 4.5) - (a + 4.5) + log(SQRT2PI * sum);
    return (temp);
 }
 
    pub fn LogFactorial(n: u32) -> f64
 /* ==================================================================
  * LogFactorial(n) returns the natural log of n!
  * NOTE: use n >= 0
  *
  * The algorithm used to evaluate the natural log of n! is based on a
  * simple equation which relates the gamma and factorial functions.
  * ==================================================================
  */
 {
    return (LogGamma(n + 1));
 }
 
    pub fn LogBeta(a:f64, b:f64) -> f64
 /* ======================================================================
  * LogBeta returns the natural log of the beta function.
  * NOTE: use a > 0.0 and b > 0.0
  *
  * The algorithm used to evaluate the natural log of the beta function is 
  * based on a simple equation which relates the gamma and beta functions.
  *
  */
 { 
    return (LogGamma(a) + LogGamma(b) - LogGamma(a + b));
 }
 
    pub fn LogChoose( n:f64,  m:f64)-> f64
 /* ========================================================================
  * LogChoose returns the natural log of the binomial coefficient C(n,m).
  * NOTE: use 0 <= m <= n
  *
  * The algorithm used to evaluate the natural log of a binomial coefficient
  * is based on a simple equation which relates the beta function to a
  * binomial coefficient.
  * ========================================================================
  */
 {
    if (m > 0)
{      return (-LogBeta(m, n - m + 1) - log(m));}
    else
      {return (0.0);}
} 

   pub fn InGamma( a:f64,  x:f64)-> f64
 /* ========================================================================
  * Evaluates the incomplete gamma function.
  * NOTE: use a > 0.0 and x >= 0.0
  *
  * The algorithm used to evaluate the incomplete gamma function is based on
  * Algorithm AS 32, J. Applied Statistics, 1970, by G. P. Bhattacharjee.
  * See also equations 6.5.29 and 6.5.31 in the Handbook of Mathematical
  * Functions, Abramowitz and Stegum (editors).  The absolute error is less 
  * than 1e-10 for all non-negative values of x.
  * ========================================================================
  */
 { 
let mut t:f64=0.0;
let mut sum:f64=0.0;
let mut term:f64=0.0;
let mut factor:f64=0.0;
let mut f:f64=0.0;
let mut g:f64=0.0;
let mut c: [f64;2] = [0;2];
let mut p:[f64;3]= [0;3];
let mut q:[f64;3]= [0;3];
let mut factor = 0.0;
    let mut n:u32 = 0;
    if (x > 0.0){
      factor = exp(-x + a * log(x) - LogGamma(a));
    }
    else{
      factor = 0.0;
    }
    if (x < a + 1.0) {                 /* evaluate as an infinite series - */
      t    = a;                        /* A & S equation 6.5.29            */
      term = 1.0 / a;
      sum  = term;
      while (term >= TINY * sum) {     /* sum until 'term' is small */
        t+=1;
        term *= x / t;
        sum  += term;
      } 
      return (factor * sum);
    }
    else {                             /* evaluate as a continued fraction - */
      p[0]  = 0.0;                     /* A & S eqn 6.5.31 with the extended */
      q[0]  = 1.0;                     /* pattern 2-a, 2, 3-a, 3, 4-a, 4,... */
      p[1]  = 1.0;                     /* - see also A & S sec 3.10, eqn (3) */
      q[1]  = x;
      f     = p[1] / q[1];
      n     = 0;
      loop {                             /* recursively generate the continued */
        g  = f;                        /* fraction 'f' until two consecutive */
        n++;                           /* values are small                   */
        if ((n % 2) > 0) {
          c[0] = ((double) (n + 1) / 2) - a;
          c[1] = 1.0;
        }
        else {
          c[0] = (double) n / 2;
          c[1] = x;
        }
        p[2] = c[1] * p[1] + c[0] * p[0];
        q[2] = c[1] * q[1] + c[0] * q[0];
        if (q[2] != 0.0) {             /* rescale to avoid overflow */
          p[0] = p[1] / q[2];
          q[0] = q[1] / q[2];
          p[1] = p[2] / q[2];
          q[1] = 1.0;
          f    = p[1];
        }
      if !(((f - g).abs() >= TINY) || (q[1] != 1.0)){break;}
   }
      return (1.0 - factor * f);
    }
 }
 
    pub fn InBeta( a:f64,  b:f64,  x:f64)-> f64 
 /* ======================================================================= 
  * Evaluates the incomplete beta function.
  * NOTE: use a > 0.0, b > 0.0 and 0.0 <= x <= 1.0
  *
  * The algorithm used to evaluate the incomplete beta function is based on
  * equation 26.5.8 in the Handbook of Mathematical Functions, Abramowitz
  * and Stegum (editors).  The absolute error is less than 1e-10 for all x
  * between 0 and 1.
  * =======================================================================
  */
 { 
let mut t =0.0;
let mut factor =0.0;
let mut f =0.0;
let mut g =0.0;
let mut c =0.0;
let mut p :[f64;3] = [0.0;3];
let mut q :[f64;3] = [0.0;3];
let mut x1 =x;
let mut b1 = b;
let mut a1 = a;
    let mut swap = 0;
    let mut n = 0;
 
    if (x > (a + 1.0) / (a + b + 1.0)) { /* to accelerate convergence   */
      swap = 1;                          /* complement x and swap a & b */
      x1    = 1.0 - x1;
      t    = a1;
      a1    = b1;
      b1    = t;
    }
    else{                                 /* do nothing */
      swap = 0;
    }
    if (x > 0.0){
      factor = (a * (x).ln() + b * (1.0 - x).ln() - LogBeta(a,b)).exp() / a;
    }
    else{
      factor = 0.0;
    }
    p[0] = 0.0;
    q[0] = 1.0;
    p[1] = 1.0;
    q[1] = 1.0;
    f    = p[1] / q[1];
    n    = 0;
    loop{                               /* recursively generate the continued */
      g = f;                           /* fraction 'f' until two consecutive */
      n+=1;                             /* values are small                   */
      if ((n % 2) > 0) {
        t = ((n - 1) / 2) as f64;
        c = -(a + t) * (a + b + t) * x / ((a + n as f64 - 1.0) * (a + n as f64));
      }
      else {
        t =  (n / 2) as f64;
        c = t * (b - t) * x / ((a + n as f64 - 1.0) * (a + n as f64));
      }
      p[2] = p[1] + c * p[0];
      q[2] = q[1] + c * q[0];
      if (q[2] != 0.0) {                 /* rescale to avoid overflow */
        p[0] = p[1] / q[2];
        q[0] = q[1] / q[2];
        p[1] = p[2] / q[2];
        q[1] = 1.0;
        f    = p[1];
      }
    if !(((f - g).abs() >= TINY) || (q[1] != 1.0)){break;}
   }
    if swap >0 {
      return (1.0 - factor * f);
    }
    else{
      return (factor * f);
    }
 }