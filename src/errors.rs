use core::{
    fmt::Display,
    fmt::{self, Debug},
  };
  
  #[derive(Debug, Default)]
  pub enum ProofVerifyError {
    #[default]
    InternalError,
    DecompressionError([u8; 32]),
  }