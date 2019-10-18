#![allow(non_snake_case)]

#[macro_use] extern crate lazy_static;
extern crate libloading;

use crate::cuda::{Libcuda};
use crate::cudart::{Libcudart};
use crate::cudnn::{Libcudnn};

use libloading::{Library};

#[cfg(target_os = "linux")]
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

pub mod cuda;
pub mod cudart;
pub mod cudnn;
pub mod extra;
