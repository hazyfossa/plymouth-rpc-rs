use crate::proto;
use anyhow::{Result, anyhow};

#[macro_export]
macro_rules! rpc_method {
    // No argument
    ($method_name:ident, $char:expr, None, $response_type:ty) => {
        pub fn $method_name(&mut self) -> $response_type {
            crate::macros::ResponseTypeAdapter(self.call(proto::Request {
                method: $char,
                argument: None,
            })?)
            .into()
        }
    };

    // With argument
    ($method_name:ident, $char:expr, $arg_name:ident, $response_type:ty) => {
        pub fn $method_name(&mut self, $arg_name: String) -> $response_type {
            crate::macros::ResponseTypeAdapter(self.call(proto::Request {
                method: $char,
                argument: Some($arg_name),
            })?)
            .into()
        }
    };
}

pub struct ResponseTypeAdapter(pub proto::Response);

impl From<ResponseTypeAdapter> for Result<bool> {
    fn from(response: ResponseTypeAdapter) -> Self {
        match response.0 {
            proto::Response::Ack => Ok(true),
            proto::Response::Nak => Ok(false),
            x => Err(anyhow!("Unexpected response: {:?}", x)),
        }
    }
}

impl From<ResponseTypeAdapter> for Result<Option<String>> {
    fn from(response: ResponseTypeAdapter) -> Self {
        match response.0 {
            proto::Response::Answer(answer) => Ok(Some(answer)),
            proto::Response::NoAnswer => Ok(None),
            x => Err(anyhow!("Unexpected response: {:?}", x)),
        }
    }
}

impl From<ResponseTypeAdapter> for Result<Option<Vec<String>>> {
    fn from(response: ResponseTypeAdapter) -> Self {
        match response.0 {
            proto::Response::MultipleAnswers(answers) => Ok(Some(answers)),
            proto::Response::NoAnswer => Ok(None),
            x => Err(anyhow!("Unexpected response: {:?}", x)),
        }
    }
}
