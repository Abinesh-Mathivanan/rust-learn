<<<<<<< HEAD
use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use crate::utils::{sigmoid, argmax};

pub struct NeuralNetwork {
    weights_input_hidden: Array2<f64>,
    weights_hidden_output: Array2<f64>,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let weights_input_hidden = Array2::random((input_size, hidden_size), Uniform::new(-1.0, 1.0));
        let weights_hidden_output = Array2::random((hidden_size, output_size), Uniform::new(-1.0, 1.0));
        
        NeuralNetwork {
            weights_input_hidden,
            weights_hidden_output,
        }
    }

    pub fn forward(&mut self, input: &Array1<f64>) -> Array1<f64> {
        let input_hidden = input.dot(&self.weights_input_hidden);
        let input_activation = input_hidden.mapv(sigmoid);
        let hidden_output = input_activation.dot(&self.weights_hidden_output);

        hidden_output.mapv(sigmoid)
    }

    pub fn train(&mut self, input: &Array1<f64>, target: &Array1<f64>, learning_rate: f64) {
        let input_hidden = input.dot(&self.weights_input_hidden);
        let input_activation = input_hidden.mapv(sigmoid);
        let hidden_output = input_activation.dot(&self.weights_hidden_output);
        let output_activation = hidden_output.mapv(sigmoid);

        let output_error = target - &output_activation;
        let output_delta = &output_error * &output_activation.mapv(|x| x * (1.0 - x));

        let hidden_error = output_delta.dot(&self.weights_hidden_output.t());
        let hidden_delta = &hidden_error * &input_activation.mapv(|x| x * (1.0 - x));

        let weight_update_hidden_output = input_activation.insert_axis(ndarray::Axis(1)).dot(&output_delta.insert_axis(ndarray::Axis(0)));
        self.weights_hidden_output += &(weight_update_hidden_output * learning_rate);

        let weight_update_input_hidden = input.clone().insert_axis(ndarray::Axis(1)).dot(&hidden_delta.insert_axis(ndarray::Axis(0)));
        self.weights_input_hidden += &(weight_update_input_hidden * learning_rate);
    }

    pub fn evaluate(&mut self, test_data: &Vec<Array1<f64>>, test_labels: &Array1<f64>) -> f64 {
        let mut correct_predictions = 0;
        for (input, &label) in test_data.iter().zip(test_labels.iter()) {
            let output = self.forward(input);
            let predicted = argmax(&output).unwrap();
            if predicted == label as usize {
                correct_predictions += 1;
            }
        }

        (correct_predictions as f64 / test_labels.len() as f64) * 100.0
    }
}
=======
use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use crate::utils::{sigmoid, argmax};

pub struct NeuralNetwork {
    weights_input_hidden: Array2<f64>,
    weights_hidden_output: Array2<f64>,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let weights_input_hidden = Array2::random((input_size, hidden_size), Uniform::new(-1.0, 1.0));
        let weights_hidden_output = Array2::random((hidden_size, output_size), Uniform::new(-1.0, 1.0));
        
        NeuralNetwork {
            weights_input_hidden,
            weights_hidden_output,
        }
    }

    pub fn forward(&mut self, input: &Array1<f64>) -> Array1<f64> {
        let input_hidden = input.dot(&self.weights_input_hidden);
        let input_activation = input_hidden.mapv(sigmoid);
        let hidden_output = input_activation.dot(&self.weights_hidden_output);

        hidden_output.mapv(sigmoid)
    }

    pub fn train(&mut self, input: &Array1<f64>, target: &Array1<f64>, learning_rate: f64) {
        let input_hidden = input.dot(&self.weights_input_hidden);
        let input_activation = input_hidden.mapv(sigmoid);
        let hidden_output = input_activation.dot(&self.weights_hidden_output);
        let output_activation = hidden_output.mapv(sigmoid);

        let output_error = target - &output_activation;
        let output_delta = &output_error * &output_activation.mapv(|x| x * (1.0 - x));

        let hidden_error = output_delta.dot(&self.weights_hidden_output.t());
        let hidden_delta = &hidden_error * &input_activation.mapv(|x| x * (1.0 - x));

        let weight_update_hidden_output = input_activation.insert_axis(ndarray::Axis(1)).dot(&output_delta.insert_axis(ndarray::Axis(0)));
        self.weights_hidden_output += &(weight_update_hidden_output * learning_rate);

        let weight_update_input_hidden = input.clone().insert_axis(ndarray::Axis(1)).dot(&hidden_delta.insert_axis(ndarray::Axis(0)));
        self.weights_input_hidden += &(weight_update_input_hidden * learning_rate);
    }

    pub fn evaluate(&mut self, test_data: &Vec<Array1<f64>>, test_labels: &Array1<f64>) -> f64 {
        let mut correct_predictions = 0;
        for (input, &label) in test_data.iter().zip(test_labels.iter()) {
            let output = self.forward(input);
            let predicted = argmax(&output).unwrap();
            if predicted == label as usize {
                correct_predictions += 1;
            }
        }

        (correct_predictions as f64 / test_labels.len() as f64) * 100.0
    }
}
>>>>>>> e9e73d3 (string and vector operations)
