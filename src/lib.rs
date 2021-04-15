/*!
# Carbon library
An amazing library to compute the PI number using the Monte-Carlo method!

Example:
```
use carbon::compute_pi;
let pi = compute_pi(100_1000);
println!("pi: {}", pi);
```

*/

pub mod monte_carlo;
pub use monte_carlo::compute_pi;
