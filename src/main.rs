mod network;
mod dataset;
mod utils;

use network::NeuralNetwork;
use dataset::{load_dataset, load_labels};
use utils::{hot_encode, normalize_images};
use ndarray::Array1;

fn main() {
    let epochs = 10;
    let learning_rate = 0.1;

    let train_images = load_dataset("data/train-images.idx3-ubyte");
    let train_labels = load_labels("data/train-labels.idx1-ubyte");

    let test_images = load_dataset("data/t10k-images.idx3-ubyte");
    let test_labels = load_labels("data/t10k-labels.idx1-ubyte");

    let mut nn = NeuralNetwork::new(784, 128, 10);

    let train_data = normalize_images(train_images);
    let test_data = normalize_images(test_images);

    for epoch in 0..epochs {
        println!("Epoch: {}", epoch + 1);

        let sample_train_data = &train_data[0..100];
        let sample_train_labels = &train_labels[0..100];

        for (input, &label) in sample_train_data.iter().zip(sample_train_labels.iter()) {
            let target = hot_encode(label as usize, 10);
            nn.train(&input, &target, learning_rate);
        }

        println!("Epoch {} completed", epoch + 1);

        let accuracy = nn.evaluate(&test_data, &Array1::from(
            test_labels.iter().map(|&x| x as f64).collect::<Vec<f64>>()
        ));
        println!("Test accuracy: {:.2}%", accuracy);
    }
}
