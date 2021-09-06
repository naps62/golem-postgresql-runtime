mod config;
mod instance;

use futures::FutureExt;

use std::fs::OpenOptions;
use std::io::prelude::*;

use ya_runtime_sdk::error::Error;
use ya_runtime_sdk::*;

use config::{PGCli, PGConf};

macro_rules! log {
    ($ctx:ident, $str:expr) => {{
        let mut log_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open($ctx.conf.logfile.clone())
            .unwrap();

        writeln!(log_file, "{}", $str).unwrap();
    }};
}

#[derive(Default, RuntimeDef)]
#[cli(PGCli)]
#[conf(PGConf)]
pub struct PGRuntime {
    username: String,
}

impl Runtime for PGRuntime {
    fn deploy<'a>(&mut self, _: &mut Context<Self>) -> OutputResponse<'a> {
        async move { Ok(None) }.boxed_local()
    }

    fn start<'a>(&mut self, ctx: &mut Context<Self>) -> OutputResponse<'a> {
        let conf = ctx.conf.clone();

        let (username, password) = instance::create(&conf);

        self.username = username;
        let url = conf.db_url_for_user(&self.username, &password, &self.username);

        log!(ctx, format!("start {}", url));

        async move { Ok(Some(serialize::json::json!(conf))) }.boxed_local()
    }

    fn stop<'a>(&mut self, ctx: &mut Context<Self>) -> EmptyResponse<'a> {
        log!(ctx, format!("stop {}", self.username));
        instance::destroy(&ctx.conf, &self.username);

        // Gracefully shutdown the service
        async move { Ok(()) }.boxed_local()
    }

    fn run_command<'a>(
        &mut self,
        command: RunProcess,
        mode: RuntimeMode,
        ctx: &mut Context<Self>,
    ) -> ProcessIdResponse<'a> {
        use std::process::Stdio;

        log!(ctx, "command ");

        if let RuntimeMode::Command = mode {
            return Error::response("Command mode is not supported");
        }

        // Echo the executed command and its arguments
        let started = tokio::process::Command::new("/bin/echo")
            .arg(command.bin)
            .args(command.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null())
            .spawn();

        // Wraps command's lifecycle. The handler is executed in background.
        // Result of `started` is handled prior to emitting command lifecycle events.
        futures::future::ready(started).as_command(ctx, |child, mut run_ctx| async move {
            let output = child.wait_with_output().await?;
            run_ctx.stdout(output.stdout).await;
            run_ctx.stderr(output.stderr).await;
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ya_runtime_sdk::run::<PGRuntime>().await
}
