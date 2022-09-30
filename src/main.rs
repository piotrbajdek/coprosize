// COPROSIZE VERSION 1.0.2 / MIT LICENSE © 2022 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod list;
pub mod menu;

// MAIN

fn main() {
    menu::documentation();
    list::models();
}
