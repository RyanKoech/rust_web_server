use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::convert::From;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

      // match str::from_utf8(buf) {
      //   Ok(_) => {},
      //   Err(_) => return Err(ParseError::InvalidEncoding),
      // };

      // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
      //   Ok(_) => {},
      //   Err(_) => return Err(e),
      // };


      // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; // The ? will check if the returned error is the correct type and if not will try to convert to the correct error type.


      let request = str::from_utf8(buf)?; // Needed to implement From std::convert::From Utf8Error for ParseError


      let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
      let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
      let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

      if protocol != "HTTP/1.1" {
          return Err(ParseError::InvalidProtocol);
      }

      let method: Method = method.parse()?;


      let mut query_string = None;
      if let Some(i) = path.find('?') {
          query_string = Some(&path[i + 1..]);
          path = &path[..i];
      }



      unimplemented!()
    }
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}


fn get_next_word(request: &str) -> Option<(&str, &str)> {
  for (i, c) in request.chars().enumerate() {
      if c == ' ' || c == '\r' {
          return Some((&request[..i], &request[i + 1..]));
      }
  }

  None
}


impl ParseError {
    pub fn message(&self) -> &str {
      match self {
        Self::InvalidRequest => "Invalid Request",
        Self::InvalidEncoding => "Invalid Encoding",
        Self::InvalidProtocol => "Invalid Protocol",
        Self::InvalidMethod => "Invalid Method",
    }
    }
}


impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
      Self::InvalidMethod
  }
}


impl From<Utf8Error> for ParseError {
  fn from(value: Utf8Error) -> Self {
      ParseError::InvalidEncoding
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl  Error for ParseError {

}