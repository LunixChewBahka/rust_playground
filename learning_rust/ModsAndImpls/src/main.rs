// This statement imports the module. rustc will ack & say hello i see a module named sample -> sample_module -> mod.rs
mod sample_modules;
// using namespace std; from c++
use sample_modules::sampleImpl;

fn main() {
    let i = 5;
    let five = String::from("5");
    println!("num to string {}", i.to_string());
    println!("{}", five);
    let myObject = sampleImpl::new("Matt".to_string());
    myObject.hello_world();
}


