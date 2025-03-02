use kand::TAFloat;
use rand::Rng;

/// Generate a vector of floating point values simulating price movements
///
/// # Panics
///
/// This function will panic if:
/// - Type conversion from `usize` to `TAFloat` fails
#[must_use]
#[allow(clippy::expect_used)]
pub fn generate_test_data(size: usize) -> Vec<TAFloat> {
    let mut rng = rand::rng();
    let mut data = Vec::with_capacity(size);

    // These constants are used frequently
    let price_init: TAFloat = 100.0;
    let increment: TAFloat = 0.001;
    let half: TAFloat = 0.5;
    let two: TAFloat = 2.0;

    let mut price = price_init;

    for i in 0..size {
        // Simulate small random price movements
        let random_factor: TAFloat = rng.random_range(0.0..1.0);
        price = (random_factor - half).mul_add(two, price);
        let idx = i as TAFloat;
        data.push(idx.mul_add(increment, price));
    }
    data
}
