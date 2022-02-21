pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function();
    // outermost::inside::inner_function();
    // outermost::inside::secret_function();
}