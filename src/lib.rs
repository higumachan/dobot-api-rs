pub mod api;
mod communicator;
mod connector;
mod protocol;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
