/// Generate a vector of f32 values simulating price movements
#[must_use]
pub fn generate_test_data(size: usize) -> Vec<f32> {
    let mut data = Vec::with_capacity(size);
    let mut price = 100.0;
    for i in 0..size {
        // Simulate small random price movements
        price += (rand::random::<f32>() - 0.5) * 2.0;
        data.push((i as f32).mul_add(0.001, price)); // Add a small increment to avoid precision loss
    }
    data
}
