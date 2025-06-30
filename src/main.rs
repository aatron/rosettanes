mod greeting;

use greeting::Greeting;

fn main() {
    let greeter = Greeting::new();
    greeter.say_hello();
}
