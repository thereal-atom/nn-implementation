mod math;
mod utils;

use math::*;
use utils::*;

struct Layer {
    weights: Vec<Vec<f32>>,
    biases: Vec<f32>,
    activations: Vec<f32>,
}

impl Layer {
    fn new(num_neurones: usize, num_neurones_in: usize) -> Layer {
        Layer {
            biases: generate_random_vector(0.0, 10.0, num_neurones),
            weights: vec![generate_random_vector(0.0, 10.0, num_neurones); num_neurones_in],
            activations: generate_random_vector(0.0, 10.0, num_neurones),
        }
    }

    fn calculate_activations(&mut self, input_activations: Vec<f32>) {
        // weighted activations
        // wa1 = w11a1 + w21a2 + w31a3 + ... + wnan
        let mut activations = matrix_vector_multiplication(
            &self.weights,
            &input_activations,
        );
    
        // biased activations
        // wab1 = wa1 + b1
        activations = vector_addition(&activations, &self.biases);
    
        // squished activations
        // sigmoid(wab1)
        for i in 0..activations.len() {
            activations[i] = sigmoid(activations[i], 50.0);
        }

        self.activations = activations;
    }
    
    // fn calculate_cost(input_activations: Vec<f32>, expected_output: Vec<f32>) -> f32 {
    //     let mut cost: f32 = 0.0;
    
    //     for i in 0..input_activations.len() {
    //         cost += (input_activations[i] - expected_output[i]).powf(2.0);
    //     }
    
    //     cost
    // }
}

fn main() {
    let mut layer = Layer::new(3, 2);

    let input_activations = generate_random_vector(0.0, 10.0, 3);

    println!("layer was inputted with these activations\n {:?}", input_activations);
    println!("layer was initialized with these weights\n {:?}", layer.weights);
    println!("layer was initialized with these biases\n {:?}", layer.biases);
    println!("layer was initialized with these activations\n {:?}", layer.activations);

    layer.calculate_activations(input_activations);

    println!("layer outputs these activations\n {:?}", layer.activations);
}