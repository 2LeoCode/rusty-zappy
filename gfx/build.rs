use {
  bindgen::{BindgenError, Builder},
  std::{
    collections::HashSet,
    io,
    path::PathBuf,
    process::{Command, ExitCode},
  },
  thiserror::Error,
};

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
  fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
    if self.0.contains(name) {
      bindgen::callbacks::MacroParsingBehavior::Ignore
    } else {
      bindgen::callbacks::MacroParsingBehavior::Default
    }
  }
}

#[derive(Error, Debug)]
enum RaylibBuildError {
  #[error("failed to run build command: {0}")]
  MakeFailed(#[from] io::Error),
  #[error("build command exited unsuccessfully")]
  MakeUnsuccessful,
  #[error("build command interrupted")]
  MakeInterrupted,
}

#[derive(Error, Debug)]
enum RaylibGenBindingsError {
  #[error("failed to generate bindings data: {0}")]
  GenerationFailed(#[from] BindgenError),
  #[error("failed to write bindings to output file: {0}")]
  WriteFailed(#[from] io::Error),
}

#[derive(Error, Debug)]
enum BuildError {
  #[error("failed to build raylib: {0}")]
  RaylibBuild(#[from] RaylibBuildError),
  #[error("failed to create bindings for raylib: {0}")]
  RaylibGenBindings(#[from] RaylibGenBindingsError),
}

fn build_raylib() -> Result<(), RaylibBuildError> {
  if let Some(code) = Command::new("make")
    .current_dir("raylib/src")
    .status()?
    .code()
  {
    if code == 0 {
      Ok(())
    } else {
      Err(RaylibBuildError::MakeUnsuccessful)
    }
  } else {
    Err(RaylibBuildError::MakeInterrupted)
  }
}

fn generate_raylib_bindings() -> Result<(), RaylibGenBindingsError> {
  let ignored_macros = IgnoreMacros(
    vec![
      "FP_INFINITE".into(),
      "FP_NAN".into(),
      "FP_NORMAL".into(),
      "FP_SUBNORMAL".into(),
      "FP_ZERO".into(),
      "IPPORT_RESERVED".into(),
    ]
    .into_iter()
    .collect(),
  );
  Builder::default()
    .header("src/raylib/wrapper.h")
    .parse_callbacks(Box::new(ignored_macros))
    .generate()?
    .write_to_file("src/raylib/bindings.rs")?;
  Ok(())
}

fn build() -> Result<(), BuildError> {
  build_raylib()?;
  generate_raylib_bindings()?;

  let raylib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("raylib/src");

  println!("cargo:rustc-link-search=native={}", raylib_path.display());
  println!("cargo::rustc-link-lib=static=raylib");
  Ok(())
}

fn main() -> ExitCode {
  use BuildError::*;
  if let Err(err) = build() {
    match err {
      RaylibBuild(err) => {
        println!("cargo::error=An error occured during raylib build: {err}");
      }
      RaylibGenBindings(err) => {
        println!("cargo::error=An error occured during raylib bindings generation: {err}");
      }
    }
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}
