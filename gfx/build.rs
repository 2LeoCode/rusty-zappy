use {
  bindgen::{BindgenError, Builder},
  std::{
    io,
    path::PathBuf,
    process::{Command, ExitCode},
  },
  thiserror::Error,
};

#[derive(Error, Debug)]
enum RaylibBuildError {
  #[error("failed to run build command")]
  CmakeFailed(#[from] io::Error),
  #[error("build command exited unsuccessfully")]
  CmakeUnsuccessful,
  #[error("build command interrupted")]
  CmakeInterrupted,
}

#[derive(Error, Debug)]
enum RaylibGenBindingsError {
  #[error("failed to generate bindings data")]
  GenerationFailed(#[from] BindgenError),
  #[error("failed to write bindings to output file")]
  WriteFailed(#[from] io::Error),
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
      Err(RaylibBuildError::CmakeUnsuccessful)
    }
  } else {
    Err(RaylibBuildError::CmakeInterrupted)
  }
}

fn generate_raylib_bindings() -> Result<(), RaylibGenBindingsError> {
  Builder::default()
    .header("raylib/src/raylib.h")
    .generate()?
    .write_to_file("src/raylib/bindings.rs")?;
  Ok(())
}

fn main() -> ExitCode {
  if let Err(err) = build_raylib() {
    println!("cargo::error=An error occured during raylib build: {err}");
    ExitCode::FAILURE
  } else {
    if let Err(err) = generate_raylib_bindings() {
      println!("cargo::error=An error occured during raylib bindings generation: {err}");
    }

    let raylib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("raylib/src");

    println!("cargo:rustc-link-search=native={}", raylib_path.display());
    println!("cargo::rustc-link-lib=static=raylib");
    ExitCode::SUCCESS
  }
}
