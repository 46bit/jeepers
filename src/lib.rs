#![feature(box_syntax, associated_consts)]

extern crate rand;
#[cfg(test)]
extern crate quickcheck;

pub mod tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
