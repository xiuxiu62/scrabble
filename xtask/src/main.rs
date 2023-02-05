use std::process::Command;
use xtask_wasm::{anyhow, clap, default_dist_dir};

const APP_NAME: &'static str = "scrabble";
const PACKAGE_NAME: &'static str = "scrabble-ui";

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = clap::Parser::parse();

    match opt {
        Opt::Dist(dist) => {
            log::info!("Generating package...");

            dist
                // .dist_dir_path("dist")
                .static_dir_path("scrabble-ui/static")
                .app_name(APP_NAME)
                .run_in_workspace(true)
                .run(PACKAGE_NAME)?;

            Ok(())
        }
        Opt::Watch(watch) => {
            log::info!("Starting watch server...");

            let mut command = Command::new("cargo");
            command.arg("check");

            watch.run(command)
        }
        Opt::Start(server) => {
            log::info!("Starting development server...");

            server.arg("dist").start(default_dist_dir(false))
        }
    }
}
