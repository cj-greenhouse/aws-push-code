use crate::repo::CodeRepository;


pub fn submit_to_pipeline<W>(_wiring: &mut W, _repo_url: &str) -> Result<(),String>
    where W: CodeRepository {
        // W::pull_repository("thing").unwrap();
        Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn happy() {
        struct Wiring;
        impl super::CodeRepository for Wiring {
            type Error = String;
            type Handle = String;
            fn pull_repository(_: &str) -> Result<Self::Handle, Self::Error> {
                Ok("repodir".to_string())
            }
        }
        let mut wiring = Wiring;

        let actual = submit_to_pipeline::<Wiring>(&mut wiring, "foo");

        assert_eq!(actual, Ok(()));
    }
}

