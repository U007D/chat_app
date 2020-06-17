pub type Result<T, E = Error<T>> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error<T> {
    #[error(transparent)]
    PoisonLockError(#[from] std::sync::PoisonError<T>),
}
