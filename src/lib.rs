#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;
pub mod elements;
pub mod tokenizer;
mod tokens;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
