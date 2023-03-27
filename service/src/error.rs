#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] diesel::result::Error),
}
