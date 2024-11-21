mod hf;
mod together;

use anyhow::Result;

use crate::cli::Args;
use crate::services::ServiceId;

pub use self::hf::HuggingFaceClient;
pub use self::together::TogetherClient;

#[async_trait::async_trait]
pub trait Client {
    // Only return `Self` if the trait is `Sized`
    fn new(timeout: u64) -> Result<Self>
    where
        Self: Sized;

    async fn generate(
        &self,
        args: &Args,
    ) -> Result<Vec<u8>>;
}

/// Create a client based on the service
pub fn create_client(
    service: &ServiceId,
    timeout: u64,
) -> Result<Box<dyn Client>> {
    // The `dyn` keyword is used to create a trait object.
    // We return a boxed trait object for runtime polymorphism, so we can handle different types of clients.
    match service {
        ServiceId::Hf => {
            let client = HuggingFaceClient::new(timeout)?;
            Ok(Box::new(client))
        }
        ServiceId::Together => {
            let client = TogetherClient::new(timeout)?;
            Ok(Box::new(client))
        }
    }
}
