#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]

pub mod borrow_lifetime;
pub mod ownership;
pub mod traits;
// pub mod control;
// pub mod variable_mutability;
