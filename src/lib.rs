mod macros;
mod proto;

use anyhow::{Context, Result};
use std::{io::Write, os::unix::net::UnixStream};

type RpcSimple = Result<bool>;
type RpcAnswer = Result<Option<String>>;
type RpcMultipleAnswers = Result<Option<Vec<String>>>;

pub struct Client {
    stream: UnixStream,
}

impl Client {
    pub fn build() -> Result<Self> {
        // TODO: What's the actual (connection) path here?
        const SOCKET_PATH: &str = "/org/freedesktop/plymouthd";
        const OLD_SOCKET_PATH: &str = "/ply-boot-protocol";

        let stream = UnixStream::connect(SOCKET_PATH).unwrap_or(
            UnixStream::connect(OLD_SOCKET_PATH).context("Failed to connect to plymouthd")?,
        );

        Ok(Self { stream })
    }

    pub fn call(&mut self, request: proto::Request) -> Result<proto::Response> {
        self.stream.write_all(&request.serialize()?)?;
        self.stream.flush()?;

        proto::Response::read(&mut self.stream)
    }
}
