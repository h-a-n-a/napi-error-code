mod catch_unwind;
mod error;

use std::borrow::Cow;

use async_backtrace::framed;
use ctor::ctor;
use error::ModuleBuildErrorDiagnostic;
use miette::{Diagnostic, GraphicalReportHandler};
use napi::{Env, JsObject};
use napi_derive::napi;

use crate::catch_unwind::CatchUnwindFuture;

pub struct Status(Cow<'static, str>);

impl AsRef<str> for Status {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

trait IntoJsError {
  fn into_js_error(self) -> napi::Error<Status>;
}

impl<T: Diagnostic + Send + Sync + 'static> IntoJsError for T {
  fn into_js_error(self) -> napi::Error<Status> {
    let status = self
      .code()
      .as_ref()
      .map(|s| Cow::Owned(s.to_string()))
      .unwrap_or(Cow::Borrowed(""));
    let handler = GraphicalReportHandler::default();
    let mut buf = String::default();
    let _ = handler.render_report(&mut buf, &self);
    napi::Error::new(Status(status), buf)
  }
}

#[napi]
pub fn error() -> Result<(), napi::Error<Status>> {
  let d = ModuleBuildErrorDiagnostic::ParseFailed(
    anyhow::anyhow!("SWC parsing error")
      .context("ooops")
      .context("so bad"),
  );
  Err(d.into_js_error())
}

#[napi]
pub fn panic(env: Env) -> JsObject {
  async fn baz() {
    panic!("baz");
  }
  async fn bar() {
    baz().await
  }
  async fn foo() {
    bar().await
  }
  env
    .execute_tokio_future(async { CatchUnwindFuture::create(foo()).await }, |_, _| {
      Ok(())
    })
    .expect("No error")
}

#[napi]
pub fn panic_async_backtrace(env: Env) -> JsObject {
  #[framed]
  async fn baz() {
    panic!("baz");
  }
  #[framed]
  async fn bar() {
    baz().await
  }
  #[framed]
  async fn foo() {
    bar().await
  }
  env
    .execute_tokio_future(async { CatchUnwindFuture::create(foo()).await }, |_, _| {
      println!("{}", async_backtrace::taskdump_tree(true));
      Ok(())
    })
    .expect("No error")
}

#[ctor]
fn ctor() {
  human_panic::setup_panic!();
}
