#[derive(Debug)]
pub enum Error {
    Argument(String),
    Image(image::ImageError),
    OutOfBounds,
}

impl Error {
    pub fn new_argument<T: Into<String>>(msg: T) -> Error {
        Error::Argument(msg.into())
    }
}

impl From<image::ImageError> for Error {
    fn from(error: image::ImageError) -> Self {
        Error::Image(error)
    }
}
