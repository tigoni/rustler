use crate::garden::vegetables::Asparagus;

//include the code found in garden.rs
pub mod garden;

pub fn grow_vegetables() {
    //this function is public to the garden module
    println!("Growing vegetables");
    //grow some asparagus
    let asparagus: Asparagus = Asparagus {};
    println!("Growing {:?}", asparagus);
}
