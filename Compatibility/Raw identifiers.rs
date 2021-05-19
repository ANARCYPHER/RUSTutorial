extern crate foo;

fn main() {
    foo::try();
}

error: expected identifier, found keyword `try`
 --> src/main.rs:4:4
  |
4 | foo::try();
  |      ^^^ expected identifier, found keyword

  extern crate foo;

fn main() {
    foo::r#try();
}

