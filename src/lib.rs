#![allow(non_snake_case)]

#[macro_use] extern crate lazy_static;
extern crate libloading;

use libloading::{Library, Symbol};

use std::cmp::{Ordering};
use std::io::{Error as IoError};
use std::os::raw::{c_char, c_int};

lazy_static! {
  static ref _CUDA: Option<Library> = Library::new("libcuda.so").ok();
  pub static ref CUDA: Option<Libcuda<'static>> = {
    _CUDA.as_ref().and_then(|lib| Libcuda::open(lib).ok())
  };

  static ref _CUDART: Option<Library> = Library::new("libcudart.so").ok();
  pub static ref CUDART: Option<Libcudart<'static>> = {
    _CUDART.as_ref().and_then(|lib| Libcudart::open(lib).ok())
  };

  static ref _CUDNN: Option<Library> = Library::new("libcudnn.so").ok();
  pub static ref CUDNN: Option<Libcudnn<'static>> = {
    _CUDNN.as_ref().and_then(|lib| Libcudnn::open(lib).ok())
  };
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Version {
  pub major:    u32,
  pub minor:    u32,
  pub patch:    u32,
}

impl PartialOrd for Version {
  fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Version {
  fn cmp(&self, other: &Version) -> Ordering {
    match self.major.cmp(&other.major) {
      Ordering::Equal => {}
      Ordering::Greater => return Ordering::Greater,
      Ordering::Less => return Ordering::Less,
    }
    match self.minor.cmp(&other.minor) {
      Ordering::Equal => {}
      Ordering::Greater => return Ordering::Greater,
      Ordering::Less => return Ordering::Less,
    }
    match self.patch.cmp(&other.patch) {
      Ordering::Equal => {}
      Ordering::Greater => return Ordering::Greater,
      Ordering::Less => return Ordering::Less,
    }
    Ordering::Equal
  }
}

#[derive(Clone, Debug)]
pub struct Libcuda<'lib> {
  pub cuDriverGetVersion:   Symbol<'lib, unsafe extern "C" fn (*mut c_int) -> c_int>,
  pub cuGetErrorName:       Symbol<'lib, unsafe extern "C" fn (c_int, *mut *const c_char) -> c_int>,
  pub cuGetErrorString:     Symbol<'lib, unsafe extern "C" fn (c_int, *mut *const c_char) -> c_int>,
  pub cuInit:               Symbol<'lib, unsafe extern "C" fn (c_int) -> c_int>,
  pub cuDeviceGetCount:     Symbol<'lib, unsafe extern "C" fn (*mut c_int) -> c_int>,
}

impl<'lib> Libcuda<'lib> {
  pub fn open(lib: &'lib Library) -> Result<Libcuda<'lib>, IoError> {
    unsafe { Ok(Libcuda{
      cuDriverGetVersion:   lib.get(b"cuDriverGetVersion")?,
      cuGetErrorName:       lib.get(b"cuGetErrorName")?,
      cuGetErrorString:     lib.get(b"cuGetErrorString")?,
      cuInit:               lib.get(b"cuInit")?,
      cuDeviceGetCount:     lib.get(b"cuDeviceGetCount")?,
    }) }
  }

  pub fn version(&self) -> Version {
    let mut raw_version: c_int = 0;
    unsafe {
      match (self.cuDriverGetVersion)(&mut raw_version as *mut c_int) {
        0 => {}
        _ => panic!(),
      }
    }
    let major = (raw_version / 1000) as u32;
    let minor = ((raw_version % 1000) / 10) as u32;
    let patch = 0;
    Version{major, minor, patch}
  }
}

#[derive(Clone, Debug)]
pub struct Libcudart<'lib> {
  pub cudaDriverGetVersion:     Symbol<'lib, unsafe extern "C" fn (*mut c_int) -> c_int>,
  pub cudaRuntimeGetVersion:    Symbol<'lib, unsafe extern "C" fn (*mut c_int) -> c_int>,
  pub cudaGetErrorString:       Symbol<'lib, unsafe extern "C" fn (c_int) -> *const c_char>,
}

impl<'lib> Libcudart<'lib> {
  pub fn open(lib: &'lib Library) -> Result<Libcudart<'lib>, IoError> {
    unsafe { Ok(Libcudart{
      cudaDriverGetVersion:     lib.get(b"cudaDriverGetVersion")?,
      cudaRuntimeGetVersion:    lib.get(b"cudaRuntimeGetVersion")?,
      cudaGetErrorString:       lib.get(b"cudaGetErrorString")?,
    }) }
  }

  pub fn version(&self) -> Version {
    let mut raw_version: c_int = 0;
    unsafe {
      match (self.cudaRuntimeGetVersion)(&mut raw_version as *mut c_int) {
        0 => {}
        _ => panic!(),
      }
    }
    let major = (raw_version / 1000) as u32;
    let minor = ((raw_version % 1000) / 10) as u32;
    let patch = 0;
    Version{major, minor, patch}
  }

  pub fn driver_supported_version(&self) -> Option<Version> {
    let mut raw_version: c_int = 0;
    unsafe {
      match (self.cudaDriverGetVersion)(&mut raw_version as *mut c_int) {
        0 => {}
        _ => panic!(),
      }
    }
    if raw_version == 0 {
      return None;
    }
    let major = (raw_version / 1000) as u32;
    let minor = ((raw_version % 1000) / 10) as u32;
    let patch = 0;
    Some(Version{major, minor, patch})
  }
}

#[derive(Clone, Debug)]
pub struct Libcudnn<'lib> {
  pub cudnnGetVersion:          Symbol<'lib, unsafe extern "C" fn () -> usize>,
  pub cudnnGetCudartVersion:    Symbol<'lib, unsafe extern "C" fn () -> usize>,
}

impl<'lib> Libcudnn<'lib> {
  pub fn open(lib: &'lib Library) -> Result<Libcudnn<'lib>, IoError> {
    unsafe { Ok(Libcudnn{
      cudnnGetVersion:          lib.get(b"cudnnGetVersion")?,
      cudnnGetCudartVersion:    lib.get(b"cudnnGetCudartVersion")?,
    }) }
  }

  pub fn version(&self) -> Version {
    let raw_version: usize = unsafe { (self.cudnnGetVersion)() };
    let major = (raw_version / 1000) as u32;
    let minor = ((raw_version % 1000) / 100) as u32;
    let patch = (raw_version % 100) as u32;
    Version{major, minor, patch}
  }

  pub fn runtime_version(&self) -> Version {
    let raw_version: usize = unsafe { (self.cudnnGetCudartVersion)() };
    let major = (raw_version / 1000) as u32;
    let minor = ((raw_version % 1000) / 10) as u32;
    let patch = 0;
    Version{major, minor, patch}
  }
}
