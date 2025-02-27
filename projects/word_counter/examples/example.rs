#![warn(clippy::unused)]
#![warn(clippy::helloworld_functions)]

fn return_array(arr: &mut [u8; 10]) {
    arr[3] = 2;
}

fn main() {
    let mut my_array = [0_u8; 10];
    return_array(&mut my_array);
    for i in my_array {
        println!("{i}");
    }
}
