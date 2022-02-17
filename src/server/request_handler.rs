use crate::server::InputData;
use crate::server::server_error::ServerError;

pub trait RequestHandler: Sync + Send {
    fn method(&self) -> &'static str;
    fn process(&self, input: &InputData) -> Result<String, ServerError>;
    fn on_error(&self, e: ServerError) -> Result<String, ServerError>;
}