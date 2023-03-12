pub fn matrix_vector_multiplication(matrix: &Vec<Vec<f32>>, vector: &Vec<f32>) -> Vec<f32> {
    let mut output_activations = vec![0.0; vector.len()];

    for i in 0..matrix.len() {
        let mut sum = 0.0;

        for j in 0..matrix[i].len() {
            sum += matrix[i][j] * vector[j]
        };

        output_activations[i] = sum;
    };

    output_activations
}

pub fn vector_addition(vector1: &Vec<f32>, vector2: &Vec<f32>) -> Vec<f32> {
    let mut output_vector = vec![0.0; vector1.len()];

    for i in 0..vector1.len() {
        output_vector[i] = vector1[i] + vector2[i];
    }

    output_vector
}

pub fn sigmoid(x: f32, k: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x / k))
}

pub fn relu(x: f32) -> f32 {
    if x > 0.0 { x } else { 0.0 }
}