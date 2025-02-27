use num_traits::{Float, FromPrimitive};
use rand::Rng;

/// Generate a vector of floating point values simulating price movements
///
/// # Panics
///
/// This function will panic if:
/// - The floating point type `T` cannot represent common values like 0.0, 0.5, 1.0, 2.0, 100.0
/// - Type conversion from `usize` to type `T` fails
#[must_use]
#[allow(clippy::expect_used)]
pub fn generate_test_data<T: Float + FromPrimitive + rand::distr::uniform::SampleUniform>(
    size: usize,
) -> Vec<T> {
    let mut rng = rand::rng();
    let mut data = Vec::with_capacity(size);

    // These constants are used frequently, so we create them once
    let price_init = T::from_f64(100.0).expect("Failed to convert 100.0 to target type");
    let increment = T::from_f64(0.001).expect("Failed to convert 0.001 to target type");
    let half = T::from_f64(0.5).expect("Failed to convert 0.5 to target type");
    let two = T::from_f64(2.0).expect("Failed to convert 2.0 to target type");

    let mut price = price_init;

    for i in 0..size {
        // Simulate small random price movements
        price = price + (rng.random_range(T::zero()..T::one()) - half) * two;
        let idx = T::from_usize(i).expect("Failed to convert index to target type");
        data.push(idx.mul_add(increment, price));
    }
    data
}
