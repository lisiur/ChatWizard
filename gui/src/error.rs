use askai_api::StreamContent;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Api(#[from] askai_api::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Send(#[from] tokio::sync::mpsc::error::SendError<StreamContent>),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}