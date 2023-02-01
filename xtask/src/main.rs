use std::path::Path;
use std::process::Command;
use xtask_wasm::{anyhow::Result, clap, default_dist_dir};

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
    GenerateProtobuf(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();
    if !Path::exists(
        std::env::current_dir()
            .unwrap()
            .join("xtask/src/main.rs")
            .as_path(),
    ) {
        panic!("Please move into the root directory of the project.")
    }

    match opt {
        Opt::Dist(dist) => {
            log::info!("Generating package...");

            dist.dist_dir_path("overlay/dist")
                .static_dir_path(std::env::current_dir().unwrap().join("overlay/static")) // cursed but ok
                .app_name("overlay")
                .run_in_workspace(true)
                .run("overlay")?;
        }
        Opt::Watch(watch) => {
            log::info!("Watching for changes and check...");

            let mut command = Command::new("cargo");
            command.arg("check");

            watch.run(command)?;
        }
        Opt::Start(mut dev_server) => {
            log::info!("Starting the development server...");

            dev_server
                .arg("dist")
                .start(std::env::current_dir().unwrap().join("overlay/dist"))?;
        }
        Opt::GenerateProtobuf(..) => prost_build::compile_protos(
            &[
                std::env::current_dir()
                    .unwrap()
                    .join("overlay/src/proto/discord.proto")
                    .as_path(),
                std::env::current_dir()
                    .unwrap()
                    .join("overlay/src/proto/models.proto")
                    .as_path(),
                std::env::current_dir()
                    .unwrap()
                    .join("overlay/src/proto/packets.proto")
                    .as_path(),
            ],
            &[std::env::current_dir()
                .unwrap()
                .join("overlay/src/proto/")
                .as_path()],
        )
        .unwrap(),
    }

    Ok(())
}
