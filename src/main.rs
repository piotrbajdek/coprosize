// COPROSIZE VERSION 1.0.3 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

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
