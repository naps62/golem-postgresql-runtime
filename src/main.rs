use futures::FutureExt;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use ya_runtime_sdk::error::Error;
use ya_runtime_sdk::*;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct PGCli {
    #[allow(unused)]
    path: Option<std::path::PathBuf>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct PGConf {
    host: String,
    port: i32,
    database: String,
    username: String,
    password: String,
}

#[derive(Default, RuntimeDef)]
#[cli(PGCli)]
#[conf(PGConf)]
pub struct PGRuntime;

impl Runtime for PGRuntime {
    fn deploy<'a>(&mut self, _: &mut Context<Self>) -> OutputResponse<'a> {
        async move { Ok(None) }.boxed_local()
    }

    fn start<'a>(&mut self, ctx: &mut Context<Self>) -> OutputResponse<'a> {
        let conf = ctx.conf.clone();

        async move { Ok(Some(serialize::json::json!(conf))) }.boxed_local()
    }

    fn stop<'a>(&mut self, _: &mut Context<Self>) -> EmptyResponse<'a> {
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

    // Simple command handler
    //
    // fn run_command<'a>(
    //     &mut self,
    //     command: RunProcess,
    //     _mode: RuntimeMode,
    //     ctx: &mut Context<Self>,
    // ) -> ProcessIdResponse<'a> {
    //     ctx.command(|mut run_ctx| async move {
    //         run_ctx.stdout(format!("[{:?}] stdout", command)).await;
    //         Ok(())
    //     })
    // }

    // Remaining trait functions have default implementations
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ya_runtime_sdk::run::<PGRuntime>().await
}
