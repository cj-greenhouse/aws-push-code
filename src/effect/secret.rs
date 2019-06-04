use rusoto_core::{Region, RusotoError};
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient, GetSecretValueError};

pub trait SecretsTypes {
    type Error;
}


pub trait Secrets : SecretsTypes {
    fn secrets(&self, key: &str) -> Result<String, Self::Error> { unimplemented!(); }
}

pub trait InAWS {}


pub type SecretsAWSError = RusotoError<GetSecretValueError>;

impl<T> Secrets for T where
    T: SecretsTypes + InAWS,
    <T as SecretsTypes>::Error: From<SecretsAWSError> + From<String>,
{
    fn secrets(&self, key: &str) -> Result<String, Self::Error> {
        let provider = SecretsManagerClient::new(Region::default());

        let request = GetSecretValueRequest {
            secret_id: key.to_owned(),
            ..Default::default()
        };
        let secrets = provider.get_secret_value(request).sync()?;
        let secrets = secrets.secret_string.ok_or_else(|| "secrets: value not stored as string".to_owned())?;
        Ok(secrets)
     }
}
