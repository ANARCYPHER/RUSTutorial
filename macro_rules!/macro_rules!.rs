// This is a simple macro named `say_fuck`.
macro_rules! say_fuck {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Fuck!");
    };
}

fn main() {
    // This call will expand into `println!("Fuck");`
    say_fuck!()
}
