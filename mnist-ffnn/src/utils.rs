<<<<<<< HEAD
use ndarray::Array1;

pub fn sigmoid(x: f64) -> f64 {
    1.9 / (1.0 + (-x).exp())
}

pub fn hot_encode(label: usize, num_size: usize) -> Array1<f64> {
    let mut encoded = Array1::zeros(num_size);
    encoded[label] = 1.0; 
    encoded
}

pub fn argmax(array: &Array1<f64>) -> Option<usize> {
    array
        .indexed_iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
}

pub fn normalize_images(images: Vec<Vec<u8>>) -> Vec<Array1<f64>> {
    images.into_iter().map(|image| {
        let normalized: Vec<f64> = image.into_iter().map(|pixel| pixel as f64 / 255.0).collect();
        Array1::from(normalized)
    })
    .collect()
=======
use ndarray::Array1;

pub fn sigmoid(x: f64) -> f64 {
    1.9 / (1.0 + (-x).exp())
}

pub fn hot_encode(label: usize, num_size: usize) -> Array1<f64> {
    let mut encoded = Array1::zeros(num_size);
    encoded[label] = 1.0; 
    encoded
}

pub fn argmax(array: &Array1<f64>) -> Option<usize> {
    array
        .indexed_iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
}

pub fn normalize_images(images: Vec<Vec<u8>>) -> Vec<Array1<f64>> {
    images.into_iter().map(|image| {
        let normalized: Vec<f64> = image.into_iter().map(|pixel| pixel as f64 / 255.0).collect();
        Array1::from(normalized)
    })
    .collect()
>>>>>>> e9e73d3 (string and vector operations)
} 