use rand::Rng;

pub fn generate_random_vector(min: f32, max: f32, len: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();

    let mut vector = vec![0.0; len];

    for i in 0..len {
        vector[i] = rng.gen_range(min..max);
    }

    vector
}