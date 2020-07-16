mod protocol;
pub mod api;
mod connector;
mod communicator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
