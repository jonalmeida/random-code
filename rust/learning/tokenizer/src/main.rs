fn main() {
    println!("{}", string_slice("This is my string".to_string()));
}

fn string_slice(original: String) -> Vec<String> {
    let vec_of_str: Vec<&str> = original.as_slice().split(' ').collect();
    let mut vec_of_string: Vec<String> = Vec::new();
    for x in vec_of_str.iter() {
        vec_of_string.push(x.to_string());
    }
    return vec_of_string;
}
