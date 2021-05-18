// A reference with 'static lifetime:
let s: &'static str = "fuck world";

// 'static as part of a trait bound:
fn generic<T>(x: T) where T: 'static {}
