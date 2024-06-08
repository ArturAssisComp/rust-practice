// Mutating the parameter does not change the original argument. The argument
// probably is copied by value.
fn increase_by (mut val: u32, how_much: u32){
    val += how_much;
    println!("You made {} points.", val);
}

fn main(){
    let score = 2000;
    increase_by(score, 12);
    println!("Original score: {}", score)
}