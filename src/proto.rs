extern crate enum_repr;
use anyhow::{Context, Result, bail};
use enum_repr::EnumRepr;
use std::{ffi::CString, io};

#[EnumRepr(type = "char")]
pub enum Method {
    Ping = 'P',
    Update = 'U',
    ChangeMode = 'C',
    SystemUpdate = 'u',
    SystemInitialized = 'S',
    Deactivate = 'D',
    Reactivate = 'r',
    Quit = 'Q',
    Reload = 'l',
    Password = '*',
    CachedPassword = 'c',
    Question = 'W',
    ShowMessage = 'M',
    HideMessage = 'm',
    Keystroke = 'K',
    KeystrokeRemove = 'L',
    ProgressPause = 'A',
    ProgressUnpause = 'a',
    ShowSplash = '$',
    HideSplash = 'H',
    Newroot = 'R',
    HasActiveVt = 'V',
    Error = '!',
}

fn read_byte(stream: &mut impl io::Read) -> io::Result<u8> {
    let mut buf = vec![0 as u8; 1];
    stream.read_exact(&mut buf)?;
    Ok(buf[0])
}

#[derive(Debug)]
pub struct Request {
    pub method: char,
    pub argument: Option<String>,
}

impl Request {
    pub fn read(stream: &mut impl io::Read) -> Result<Self> {
        todo!()
    }

    pub fn serialize(self) -> Result<Vec<u8>> {
        let argument: String = self.argument.unwrap_or_default();

        // TODO: Construct a CString directly to minimize allocations
        Ok(CString::new(format!(
            "{}\x02{}{}",
            self.method,
            argument.len() as u8 as char,
            argument
        ))
        .context("Cannot serialize request as a C string")?
        .into())
    }
}

#[derive(Debug)]
pub enum Response {
    Ack,
    Nak,
    Answer(String),
    MultipleAnswers(Vec<String>),
    NoAnswer,
}

impl Response {
    // TODO: There is likely a better way to do this
    const RET_ACK: u8 = 6;
    const RET_NAK: u8 = 15;
    const RET_ANSWER: u8 = 2;
    const RET_MULTIPLE_ANSWERS: u8 = 9;
    const RET_NO_ANSWER: u8 = 5;

    pub fn read(stream: &mut impl io::Read) -> Result<Self> {
        let ret = read_byte(stream)?;

        let return_type = match ret {
            Self::RET_ACK => Self::Ack,
            Self::RET_NAK => Self::Nak,

            Self::RET_ANSWER => {
                let size = read_byte(stream)?;
                let mut buf = vec![0; size as usize];
                stream.read_exact(&mut buf)?;

                Self::Answer(String::from_utf8(buf)?)
            }

            Self::RET_MULTIPLE_ANSWERS => {
                todo!()
            }
            Self::RET_NO_ANSWER => Self::NoAnswer,

            _ => bail!("Unknown response code: {}", ret),
        };

        Ok(return_type)
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        todo!()
    }
}
