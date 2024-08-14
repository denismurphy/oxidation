use std::fmt;
use reqwest::Error as RequestError;

pub fn run() {
    println!("\n--- Error Handling Examples ---");

    opaque_errors_example();
    transparent_errors_example();
    hybrid_error_handling_example();
}

fn opaque_errors_example() {
    pub struct MyError {
        inner: RequestError,  // Private field to encapsulate the underlying error
    }

    impl fmt::Debug for MyError {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            // Provide a detailed debug output, optionally just delegating to the inner error's fmt
            write!(formatter, "MyError: {:?}", self.inner)
        }
    }

    impl fmt::Display for MyError {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            // Provide a user-facing error message that abstracts away the details
            write!(formatter, "Request failed: {}", self.inner)
        }
    }

    impl std::error::Error for MyError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.inner)
        }
    }
}

fn transparent_errors_example() {

    #[derive(Debug)]
    enum FileError {
        NotFound(String),
        AccessDenied,
        Io(std::io::Error),
    }

    impl fmt::Display for FileError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                FileError::NotFound(ref path) => write!(formatter, "File not found: {}", path),
                FileError::AccessDenied => write!(formatter, "Access denied"),
                FileError::Io(ref err) => write!(formatter, "IO error: {}", err),
            }
        }
    }

    impl std::error::Error for FileError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match *self {
                FileError::Io(ref err) => Some(err),
                _ => None,
            }
        }
    }
}

fn hybrid_error_handling_example() {
    pub struct GenericError<T> {
        kind: T,
        message: String,
    }

    impl<T> fmt::Debug for GenericError<T>
    where
        T: fmt::Debug,
    {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "{:?}: {}", self.kind, self.message)
        }
    }

    impl<T> fmt::Display for GenericError<T>
    where
        T: fmt::Debug + fmt::Display,
    {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "{}: {}", self.kind, self.message)
        }
    }

    impl<T> std::error::Error for GenericError<T> where T: fmt::Debug + fmt::Display {}

    // Example usage in a hypothetical AWS SDK
    #[derive(Debug)]
    enum S3Error {
        BucketNotFound,
        AccessDenied,
    }

    impl fmt::Display for S3Error {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                S3Error::BucketNotFound => write!(formatter, "Bucket not found"),
                S3Error::AccessDenied => write!(formatter, "Access denied"),
            }
        }
    }

    type S3Result<T> = Result<T, GenericError<S3Error>>;
}