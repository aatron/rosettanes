pub struct Greeting;

impl Greeting {
    pub fn new() -> Self {
        Greeting
    }

    pub fn say_hello(&self) {
        let greeting = String::from("Hello world");
        println!("{}", greeting);
    }
}
