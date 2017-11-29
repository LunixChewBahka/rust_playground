pub struct sampleImpl{
    myName:String
}

// factory method
impl sampleImpl {
    pub fn new(name:String) -> sampleImpl {
        sampleImpl{myName: name}
    } // static method

    pub fn hello_world(&self) { // takes reference to object itself
        println!("My Name is: {:?}", self.myName); // simple String replace
    }

