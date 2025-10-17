#![allow(unused)]

#[derive(Copy, Clone)]
struct Float {
    data: u32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative
}
