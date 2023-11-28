#![feature(error_generic_member_access)]
mod error;

use error::ModuleBuildErrorDiagnostic;
use miette::Error;

fn main() -> Result<(), Error> {
  human_panic::setup_panic!();
  Err(
    ModuleBuildErrorDiagnostic::ParseFailed(
      anyhow::anyhow!("SWC parsing error")
        .context("something is wrong")
        .context("so bad"),
    )
    .into(),
  )
}
