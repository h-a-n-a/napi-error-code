use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(code("ModuleBuildError"))]
pub enum ModuleBuildErrorDiagnostic {
  #[error("Parse failed")]
  ParseFailed(#[from] anyhow::Error),
}
