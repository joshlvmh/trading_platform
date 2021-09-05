//mod errors {
use std::error;
use std::fmt;

    #[derive(Debug,Clone)]
    pub struct OrderError;
    impl error::Error for OrderError {}
    impl fmt::Display for OrderError {
        fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "")
        }
    }
//}
