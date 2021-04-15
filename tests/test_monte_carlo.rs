use carbon::compute_pi;

#[test]
fn test_monte_carlo() {
    let result = compute_pi(100_000);
    assert!(result > 3.0 && result < 4.0);
}
