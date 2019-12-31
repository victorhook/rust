
/* Representings the complex number used to calculate the mandelbrot */
#[derive(Copy, Debug)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    
    pub fn new(re: f64, im: f64) -> Self {
        Complex {
            re,
            im
        }
    }

    /* Simple addition for a complex number */
    pub fn add(&mut self, other: &Complex) {
        self.re += other.re;
        self.im += other.im;
    }

    /* Be careful to store the real part of the number before modifying it.
        Otherwise the imaginary part will use the newly updated real part */
    pub fn mul(&mut self, other: &Complex) {
        let temp = self.re;
        self.re = (self.re * other.re) - (self.im * other.im);
        self.im = (temp * other.im) + (self.im * other.re);
    }

    /* Returns the absolute value SQUARED. This is simply to save computing power
        and make the calculations faster. When the value is compared with in the logic
        code, we can just compare with the expected abs value squared, which makes it work */
    pub fn get_abs(&self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }
}

/* Used for debugging */
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}i", self.re, if self.im < 0 as f64 {"-"} else {"+"}, self.im)
    }
}

/* Used in the mul() function to enable a complex number to be squared
    with itself before added with the constans. */
impl Clone for Complex {
    fn clone(&self) -> Complex {
        *self
    }
}
