use aws_nitro_enclaves_nsm_api::api::{Request, Response, Digest, ErrorCode};
use aws_nitro_enclaves_nsm_api::driver::{nsm_exit, nsm_init, nsm_process_request};
use napi::{Env, Error, JsBuffer, Result};
use serde_bytes::ByteBuf;

fn error(msg: String) -> Error {
  Error::new(napi::Status::Unknown, msg)
}

fn error_from_code(code: ErrorCode) -> Error {
  error(format!("{:?}", code))
}

#[napi(object)]
pub struct Description {
  pub version_major: u16,
  pub version_minor: u16,
  pub version_patch: u16,
  pub module_id: String,
  pub max_pcrs: u16,
  pub locked_pcrs: Vec<u16>,
  pub digest: String,
}

#[napi(object)]
pub struct PcrDescription {
  pub lock: bool,
  pub data: JsBuffer,
}

#[napi]
pub fn open() -> i32 {
  nsm_init()
}

#[napi]
pub fn init() -> i32 {
  nsm_init()
}

#[napi]
pub fn close(fd: i32) {
  nsm_exit(fd)
}

#[napi]
pub fn exit(fd: i32) {
  nsm_exit(fd)
}

#[napi]
pub fn extend_pcr(env: Env, fd: i32, index: u16, data: JsBuffer) -> Result<JsBuffer> {
  let request = Request::ExtendPCR {
    index,
    data: data.into_value()?.to_vec(),
  };
  match nsm_process_request(fd, request) {
    Response::ExtendPCR { data: pcr_data } => Ok(env.create_buffer_with_data(pcr_data)?.into_raw()),
    Response::Error(err) => Err(error_from_code(err)),
    _ => Err(error_from_code(ErrorCode::InvalidResponse)),
  }
}

#[napi]
pub fn get_pcr_description(env: Env, fd: i32, index: u16) -> Result<PcrDescription> {
  let request = Request::DescribePCR { index };
  match nsm_process_request(fd, request) {
    Response::DescribePCR { lock, data } => Ok(PcrDescription {
      lock,
      data: env.create_buffer_with_data(data)?.into_raw(),
    }),
    Response::Error(err) => Err(error_from_code(err)),
    _ => Err(error_from_code(ErrorCode::InvalidResponse)),
  }
}

#[napi]
pub fn lock_pcr(fd: i32, index: u16) -> Result<()> {
  let request = Request::LockPCR { index };
  match nsm_process_request(fd, request) {
    Response::LockPCR => Ok(()),
    Response::Error(err) => Err(error_from_code(err)),
    _ => Err(error_from_code(ErrorCode::InvalidResponse)),
  }
}

#[napi]
pub fn lock_pcrs(fd: i32, range: u16) -> Result<()> {
  let request = Request::LockPCRs { range };
  match nsm_process_request(fd, request) {
    Response::LockPCRs => Ok(()),
    Response::Error(err) => Err(error_from_code(err)),
    _ => Err(error_from_code(ErrorCode::InvalidResponse)),
  }
}

#[napi]
pub fn get_description(fd: i32) -> Result<Description> {
  let request = Request::DescribeNSM;
  match nsm_process_request(fd, request) {
    Response::DescribeNSM {
      version_major,
      version_minor,
      version_patch,
      module_id,
      max_pcrs,
      locked_pcrs,
      digest,
    } => Ok(Description {
      version_major,
      version_minor,
      version_patch,
      module_id,
      max_pcrs,
      locked_pcrs: locked_pcrs.into_iter().collect::<Vec<_>>(),
      digest: match digest {
        Digest::SHA256 => "sha256".to_string(),
        Digest::SHA512 => "sha512".to_string(),
        Digest::SHA384 => "sha384".to_string(),
      },
    }),
    Response::Error(err) => Err(error_from_code(err)),
    _ => Err(error_from_code(ErrorCode::InvalidResponse)),
  }
}

#[napi]
pub fn get_attestation_doc(
  env: Env,
  fd: i32,
  user_data: Option<JsBuffer>,
  nonce: Option<JsBuffer>,
  public_key: Option<JsBuffer>,
) -> Result<JsBuffer> {
  let request = Request::Attestation {
    user_data: user_data.map(|buf| ByteBuf::from(buf.into_value().unwrap().to_vec())),
    nonce: nonce.map(|buf| ByteBuf::from(buf.into_value().unwrap().to_vec())),
    public_key: public_key.map(|buf| ByteBuf::from(buf.into_value().unwrap().to_vec())),
  };
  match nsm_process_request(fd, request) {
    Response::Attestation { document } => Ok(env.create_buffer_with_data(document)?.into_raw()),
    Response::Error(err) => Err(error_from_code(err)),
    _ => Err(error_from_code(ErrorCode::InvalidResponse)),
  }
}

#[napi]
pub fn get_random(env: Env, fd: i32) -> Result<JsBuffer> {
  let request = Request::GetRandom;
  match nsm_process_request(fd, request) {
    Response::GetRandom { random } => Ok(env.create_buffer_with_data(random)?.into_raw()),
    Response::Error(err) => Err(error_from_code(err)),
    _ => Err(error_from_code(ErrorCode::InvalidResponse)),
  }
}
