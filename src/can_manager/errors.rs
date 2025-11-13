use thiserror::Error;

#[derive(Debug, Error)]
pub enum SendFrameError {
    #[error("invalid CAN ID: {raw_id}")]
    InvalidId { raw_id: u16 },

    #[error("cannot build CAN FD frame: data length {len} is invalid")]
    InvalidFrameLength { len: usize },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
