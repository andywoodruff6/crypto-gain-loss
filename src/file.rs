use std::{fs::File, io::Read};

pub fn read_address_file() -> Vec<String> {
    let mut file = File::open("addresses.txt").expect("File not found");
    let mut contents = String::new();

    #[allow(unused_assignments)]
    let mut contents_as_vec = Vec::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    contents_as_vec = contents.split('\n').map(|s| s.to_string()).collect();
    print!("Addresses: {:?}", contents_as_vec);
    contents_as_vec

}