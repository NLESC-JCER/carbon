
use ndarray::prelude::*; // Import the most used types, functions, etc.
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

/// Compute the PI number using the Monte-Carlo method
/// * `nsamples` - number of samples to use
pub fn compute_pi(nsamples: usize) -> f64 {
    // Code comments on the right are the equivalent Python (Numpy) operations
    let xs = Array1::random(nsamples, Uniform::new(0., 1.)); // xs = np.random.uniform(size=samples)
    let ys = Array1::random(nsamples, Uniform::new(0., 1.)); // ys = np.random.uniform(size=samples)
    let xs = xs.mapv(|x| f64::powi(x, 2)); // xs =  xs ** 2
    let ys = ys.mapv(|y| f64::powi(y, 2)); // ys = ys ** 2
    let rs = (xs + ys).mapv(f64::sqrt); // rs = np.sqrt( xs + ys)
    let sum = rs.fold(0.0, |acc, &r| if r < 1.0 { acc + 1. } else { acc }); // rs = rs[rs > 1.0]; sum = float(len(rs))
    sum * 4.0 / (nsamples as f64)
}
