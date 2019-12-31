mod complex;
use complex::Complex;

/* A recursive method to calculate the mandelbrot for a complex starting number
    z and the constant c. */
fn mandelbrot_recursive(mut z: Complex, c: &Complex, iteration: i32) -> (Complex, &Complex, i32) {
    if iteration >= ITERATIONS || z.get_abs() >= 4 as f64 {
        (z, c, iteration)
    } else {
        z.mul(&z.clone());
        z.add(c);
        mandelbrot_recursive(z, &c, iteration + 1)
    }
}

/* Wrapper method for the mandelbrot calculation, since all we want to know (in this case)
    is wether the complex number belongs to the mandelbrot or not */
fn calculate_mandelbrot(c: &Complex) -> bool {
    let (_, _, iterations) = mandelbrot_recursive(Complex::new(0.0, 0.0), c, 0);
    iterations == ITERATIONS
}

/* Creates the a matrice containing all the complex numbers that are required */
fn make_matrice(screen: &Screen) -> Vec<Vec<Complex>> {
    
    let mut matrice: Vec<Vec<Complex>> = Vec::with_capacity(SCREEN_Y);
    
    let re_step_size = f64::abs(screen.re_max - screen.re_min) / SCREEN_X as f64;
    let im_step_size = f64::abs(screen.im_max - screen.im_min) / SCREEN_Y as f64;

    for row in 0..SCREEN_Y {
        matrice.push(Vec::with_capacity(SCREEN_X));
        for col in 0..SCREEN_X {
            &matrice[row].push(Complex::new(
                    screen.re_min + (col as f64 * re_step_size),
                    screen.im_max - (row as f64 * im_step_size),
            ));
        }
    }
    matrice
}

/* Prints the mandelbrot series. It also stores the values in a matrice
    as chars (this is not really needed, but if further improvements wants
    to be done, it's nice to have the values in a matrice) */ 
fn print_mandelbrot(matrice: &Vec<Vec<Complex>>) {
    let mut map: Vec<Vec<char>> = Vec::with_capacity(SCREEN_Y);

    for row in 0..SCREEN_Y {
        map.push(Vec::with_capacity(SCREEN_X));

        for col in 0..SCREEN_X {
            if calculate_mandelbrot(&&matrice[row][col]) {
                &map[row].push('X');
            } else {
                &map[row].push(' ');
            }
            print!("{}", &map[row][col]);
        }
        println!("\n");
    }
}

/* Containts the screen values to help make a proper mapping of
    the complex number matrice, together with the screen resolution */
struct Screen {
    re_max: f64,
    re_min: f64,
    im_max: f64,
    im_min: f64,
}

const SCREEN_X: usize = 100;
const SCREEN_Y: usize = 30;
const ITERATIONS: i32 = 200;

fn main() {
    
    let screen = Screen {
        re_max: 0.8,
        re_min: -2.4,
        im_max: 1.4,
        im_min: -1.4,
    };

    let matrice = make_matrice(&screen);
    print_mandelbrot(&matrice);

}
