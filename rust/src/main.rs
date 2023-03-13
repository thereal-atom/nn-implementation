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
}

struct NeuralNetwork {
    layers: Vec<Layer>
}

impl NeuralNetwork {
    fn new(layer_sizes: Vec<usize>) -> NeuralNetwork {
        let mut layers: Vec<Layer> = vec![];

        for i in 1..layer_sizes.len() {
            layers.push(Layer::new(layer_sizes[i], layer_sizes[i - 1]))
        }

        NeuralNetwork {
            layers
        }
    }

    fn calculate_cost(&self, input_activations: Vec<f32>, expected_outputs: Vec<f32>) -> f32 {
        let mut total_cost = 0.0;

        for i in 0..self.layers.len() {
            for j in 0..self.layers[i].activations.len() {
                total_cost += (if i == 0 { input_activations[j] } else { self.layers[i - 1].activations[j] } - expected_outputs[i]).powf(2.0);
            }
        }

        total_cost / expected_outputs.len() as f32
    }

    // // cost of the network across all data points
    // fn calculate_overall_cost(&self, input_data: &Vec<&Vec<f32>>, expected_outputs: &Vec<f32>) -> f32 {
    //     let mut total_cost = 0.0;

    //     for i in 0..input_data.len() {
    //         total_cost += self.calculate_cost(input_data[i].to_vec(), expected_outputs.to_vec());
    //     };

    //     total_cost / input_data.len() as f32
    // }
}

fn main() {
    // let mut layer = Layer::new(3, 2);

    // let input_activations = generate_random_vector(0.0, 10.0, 3);

    // println!("layer was inputted with these activations\n {:?}", input_activations);
    // println!("layer was initialized with these weights\n {:?}", layer.weights);
    // println!("layer was initialized with these biases\n {:?}", layer.biases);
    // println!("layer was initialized with these activations\n {:?}", layer.activations);

    // layer.calculate_activations(input_activations);

    // println!("layer outputs these activations\n {:?}", layer.activations);

    let nn = NeuralNetwork::new(Vec::from([2, 3, 2]));

    // println!("{:?}", nn.calculate_overall_cost(
    //     &vec![
    //         &vec![0.1, 0.2, 0.3],
    //         &vec![0.2, 0.4, 0.6],
    //         &vec![0.3, 0.2, 0.1]
    //     ],
    //     &vec![1.0, 0.0, 0.0]
    // )
}