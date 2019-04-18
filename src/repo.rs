

pub trait CodeRepository {
    type Error: std::fmt::Debug;
    type Handle;
    fn pull_repository(url: &str) -> Result<Self::Handle, Self::Error>;
}



