use crate::nsm_lib::{
  nsm_describe_pcr, nsm_extend_pcr, nsm_get_attestation_doc, nsm_get_description, nsm_get_random,
  nsm_lib_exit, nsm_lib_init, nsm_lock_pcr, nsm_lock_pcrs, Digest, ErrorCode, NsmDescription,
};
use napi::{Error, Result};
use std::slice;

const PCR_MAX_LEN: usize = 8192;
const ATT_DOC_MAX_LEN: usize = 8192;
const RANDOM_MAX_LEN: usize = 8192;

fn error(msg: String) -> Error {
  Error::new(napi::Status::Unknown, msg)
}

fn error_from_code(code: ErrorCode) -> Error {
  error(format!("{:?}", code))
}

fn get_vec_from_raw<T: Clone>(data: *const T, data_len: u32) -> Result<Vec<T>> {
  if data.is_null() {
    return Err(error("invalid raw data".to_string()));
  }

  let slice = unsafe { slice::from_raw_parts(data, data_len as usize) };
  Ok(slice.to_vec())
}

#[napi(object)]
pub struct Description {
  pub version_major: u16,
  pub version_minor: u16,
  pub version_patch: u16,
  pub module_id: Vec<u8>,
  pub module_id_len: u32,
  pub max_pcrs: u16,
  pub locked_pcrs: Vec<u16>,
  pub locked_pcrs_len: u32,
  pub digest: String,
}

#[napi(object)]
pub struct PcrDescription {
  pub lock: bool,
  pub data: Vec<u8>,
}

#[napi]
pub fn open() -> i32 {
  nsm_lib_init()
}

#[napi]
pub fn init() -> i32 {
  nsm_lib_init()
}

#[napi]
pub fn close(fd: i32) {
  nsm_lib_exit(fd)
}

#[napi]
pub fn exit(fd: i32) {
  nsm_lib_exit(fd)
}

#[napi]
pub fn extend_pcr(fd: i32, index: u16, data: Vec<u8>) -> Result<Vec<u8>> {
  let data_ptr = data.as_ptr() as *const u8;
  let data_len = data.len() as u32;
  let pcr_ptr = vec![0u8; PCR_MAX_LEN].as_mut_ptr();
  let mut pcr_len = PCR_MAX_LEN as u32;
  let err_code = unsafe { nsm_extend_pcr(fd, index, data_ptr, data_len, pcr_ptr, &mut pcr_len) };

  match err_code {
    ErrorCode::Success => Ok(get_vec_from_raw(pcr_ptr, pcr_len)?),
    err_code => Err(error_from_code(err_code)),
  }
}

#[napi]
pub fn get_pcr_description(fd: i32, index: u16) -> Result<PcrDescription> {
  let pcr_ptr = vec![0u8; PCR_MAX_LEN].as_mut_ptr();
  let mut pcr_len = PCR_MAX_LEN as u32;
  let mut lock = false;
  let err_code = unsafe { nsm_describe_pcr(fd, index, &mut lock, pcr_ptr, &mut pcr_len) };

  match err_code {
    ErrorCode::Success => Ok(PcrDescription {
      lock,
      data: get_vec_from_raw(pcr_ptr, pcr_len)?,
    }),
    err_code => Err(error_from_code(err_code)),
  }
}

#[napi]
pub fn lock_pcr(fd: i32, index: u16) -> Result<()> {
  match nsm_lock_pcr(fd, index) {
    ErrorCode::Success => Ok(()),
    err_code => return Err(error_from_code(err_code)),
  }
}

#[napi]
pub fn lock_pcrs(fd: i32, range: u16) -> Result<()> {
  match nsm_lock_pcrs(fd, range) {
    ErrorCode::Success => Ok(()),
    err_code => return Err(error_from_code(err_code)),
  }
}

#[napi]
pub fn get_description(fd: i32) -> Result<Description> {
  let mut nsm_description = &mut NsmDescription {
    version_major: 0u16,
    version_minor: 0u16,
    version_patch: 0u16,
    module_id: [0u8; 100],
    module_id_len: 0u32,
    max_pcrs: 0u16,
    locked_pcrs: [0u16; 64],
    locked_pcrs_len: 0u32,
    digest: Digest::SHA256,
  };

  match nsm_get_description(fd, &mut nsm_description) {
    ErrorCode::Success => Ok(Description {
      version_major: nsm_description.version_major,
      version_minor: nsm_description.version_minor,
      version_patch: nsm_description.version_patch,
      module_id: nsm_description.module_id.to_vec(),
      module_id_len: nsm_description.module_id_len,
      max_pcrs: nsm_description.max_pcrs,
      locked_pcrs: nsm_description.locked_pcrs.to_vec(),
      locked_pcrs_len: nsm_description.locked_pcrs_len,
      digest: format!("{:?}", nsm_description.digest),
    }),
    err_code => return Err(error_from_code(err_code)),
  }
}

#[napi]
pub fn get_attestation_doc(
  fd: i32,
  user_data: Vec<u8>,
  nonce_data: Vec<u8>,
  pub_key_data: Vec<u8>,
) -> Result<Vec<u8>> {
  let user_ptr: *const u8 = user_data.as_ptr();
  let user_len = user_data.len() as u32;
  let nonce_ptr = nonce_data.as_ptr() as *const u8;
  let nonce_len = nonce_data.len() as u32;
  let pub_key_ptr: *const u8 = pub_key_data.as_ptr();
  let pub_key_len = pub_key_data.len() as u32;
  let att_doc_ptr = vec![0u8; ATT_DOC_MAX_LEN].as_mut_ptr();
  let mut att_doc_len = ATT_DOC_MAX_LEN as u32;
  let err_code = unsafe {
    nsm_get_attestation_doc(
      fd,
      user_ptr,
      user_len,
      nonce_ptr,
      nonce_len,
      pub_key_ptr,
      pub_key_len,
      att_doc_ptr,
      &mut att_doc_len,
    )
  };

  match err_code {
    ErrorCode::Success => Ok(get_vec_from_raw(att_doc_ptr, att_doc_len)?),
    err_code => Err(error_from_code(err_code)),
  }
}

#[napi]
pub fn get_random(fd: i32) -> Result<Vec<u8>> {
  let buf = vec![0u8; RANDOM_MAX_LEN].as_mut_ptr();
  let mut buf_len = RANDOM_MAX_LEN;
  let err_code = unsafe { nsm_get_random(fd, buf, &mut buf_len) };

  match err_code {
    ErrorCode::Success => Ok(get_vec_from_raw(buf, buf_len as u32)?),
    err_code => Err(error_from_code(err_code)),
  }
}
