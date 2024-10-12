use std::fs::File;
use std::io::{BufReader, Read};

fn read_metadata<R: Read>(reader: &mut R) -> u32 {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes).unwrap();
    u32::from_be_bytes(bytes)
}

pub fn load_dataset(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("Cannot open file");
    let mut reader = BufReader::new(file);

    let magic_number = read_metadata(&mut reader);
    let num_images = read_metadata(&mut reader);
    let num_cols = read_metadata(&mut reader);
    let num_rows = read_metadata(&mut reader);

    assert_eq!(magic_number, 2051);

    let mut images = Vec::new();
    for _ in 0..num_images {
        let mut image = vec![0u8; (num_cols*num_rows) as usize];
        reader.read_exact(&mut image).unwrap();
        images.push(image);
    }
    images
}

pub fn load_labels(filename: &str) -> Vec<u8> {
    let file = File::open(filename).expect("Cannot open file");
    let mut reader = BufReader::new(file);

    let magic_number = read_metadata(&mut reader);
    let num_labels = read_metadata(&mut reader);

    assert_eq!(magic_number, 2049);

    let mut labels = vec![0u8; num_labels as usize];
    reader.read_exact(&mut labels).unwrap();

    labels
}

