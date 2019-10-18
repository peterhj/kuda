use crate::extra::{Version};

use libloading::{Library, Symbol};

use std::io::{Error as IoError};

#[derive(Clone, Debug)]
pub struct RawLibcudnn<'lib> {
  pub cudnnGetVersion:          Symbol<'lib, unsafe extern "C" fn () -> usize>,
  pub cudnnGetCudartVersion:    Symbol<'lib, unsafe extern "C" fn () -> usize>,
}

impl<'lib> RawLibcudnn<'lib> {
  pub unsafe fn open(lib: &'lib Library) -> Result<RawLibcudnn<'lib>, IoError> {
    Ok(RawLibcudnn{
      cudnnGetVersion:          lib.get(b"cudnnGetVersion")?,
      cudnnGetCudartVersion:    lib.get(b"cudnnGetCudartVersion")?,
    })
  }
}

#[derive(Clone, Debug)]
pub struct Libcudnn<'lib> {
  raw:  RawLibcudnn<'lib>,
}

impl<'lib> Libcudnn<'lib> {
  pub fn open(lib: &'lib Library) -> Result<Libcudnn<'lib>, IoError> {
    unsafe {
      Ok(Libcudnn{raw: RawLibcudnn::open(lib)?})
    }
  }

  pub fn version(&self) -> Version {
    let raw_version: usize = unsafe { (self.raw.cudnnGetVersion)() };
    let major = (raw_version / 1000) as u32;
    let minor = ((raw_version % 1000) / 100) as u32;
    let patch = (raw_version % 100) as u32;
    Version{major, minor, patch}
  }

  pub fn required_runtime_version(&self) -> Version {
    let raw_version: usize = unsafe { (self.raw.cudnnGetCudartVersion)() };
    let major = (raw_version / 1000) as u32;
    let minor = ((raw_version % 1000) / 10) as u32;
    let patch = 0;
    Version{major, minor, patch}
  }
}
