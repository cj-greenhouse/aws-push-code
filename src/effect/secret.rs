use rusoto_core::{Region, RusotoError};
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient, GetSecretValueError};
use serde_json::Value;

pub trait SecretsTypes {
    type Error;
}


pub trait Secrets : SecretsTypes {
    fn secrets(&self) -> Result<Value, Self::Error> { unimplemented!(); }
}

pub trait InAWS {}

pub trait SecretsAWSConfig {
    fn id(&self) -> String;
}

pub type SecretsAWSError = RusotoError<GetSecretValueError>;

impl<T> Secrets for T where
    T: SecretsTypes + InAWS + SecretsAWSConfig,
    <T as SecretsTypes>::Error: From<SecretsAWSError>
{
    fn secrets(&self) -> Result<Value, Self::Error> {
        let provider = SecretsManagerClient::new(Region::default());

        let request = GetSecretValueRequest {
            secret_id: (self as &SecretsAWSConfig).id(),
            ..Default::default()
        };
        let secrets = provider.get_secret_value(request).sync()?;
        let secrets = secrets.secret_string.unwrap();           // crash if not configured as a string
        let secrets = match serde_json::from_str(&secrets) {
            Ok(v) => v,                     // try to view it as JSON
            _ => Value::String(secrets)     // otherwise, interpret as simple string converted to JSON
        };
        Ok(secrets)
     }
}
