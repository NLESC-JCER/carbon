use carbon::monte_carlo;

fn main() {
    let nsamples = 100_000;
    let result = monte_carlo::compute_pi(nsamples);
    println!("...And the PI number is: {}", result);
}
