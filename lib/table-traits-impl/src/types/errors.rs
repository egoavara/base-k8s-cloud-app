#[derive(thiserror::Error, Debug)]
pub enum CursorDecodeError {
    #[error("Invalid cursor")]
    InvalidCursor,
    #[error(transparent)]
    InvalidBase64(#[from] data_encoding::DecodeError),
    #[error(transparent)]
    InvalidPostcard(#[from] postcard::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum PageCursorVariantError {
    #[error("after with last not allowed")]
    AfterWithLast,
    #[error("before with first not allowed")]
    BeforeWithFirst,
    #[error("both first and last not allowed")]
    BothFirstAndLast,
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    PageCursorVariantError(#[from] PageCursorVariantError),
    
}
