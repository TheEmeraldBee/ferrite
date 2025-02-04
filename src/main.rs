use makefile_lossless::*;
use std::{
    io::{Read, Write},
    process::Command,
    sync::{Arc, Mutex},
};
use widgetui::*;

mod chunks;
mod input;
mod render;
mod state;

use chunks::*;
use input::*;
use render::*;
use state::MakefileState;

fn main() -> anyhow::Result<()> {
    let last_target = match std::fs::File::open("/tmp/ferrite/target.txt") {
        Ok(mut f) => {
            let mut string = String::new();
            f.read_to_string(&mut string)?;
            Some(string.trim().to_string())
        }
        Err(_) => None,
    };

    let file = match std::fs::File::open("Makefile") {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file due to error: {:?}", e);
            return Ok(());
        }
    };

    let makefile = Makefile::read(file)?;
    let targets = makefile
        .rules()
        .map(|x| x.targets().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>();

    if targets.len() == 0 {
        println!("This makefile has no targets!");
    }

    let run_target = Arc::new(Mutex::new(None));

    App::new(100)?
        .handle_panics()
        .states(MakefileState {
            targets,
            run_target: run_target.clone(),
            selected: 0,
            last_target,
        })
        .widgets((chunk, exit, selection, runner, status, render))
        .run()?;

    if let Some(target) = run_target.lock().unwrap().take() {
        std::fs::create_dir_all("/tmp/ferrite/")?;
        write!(
            std::fs::File::create("/tmp/ferrite/target.txt")?,
            "{}",
            target
        )?;
        Command::new("make").arg(target).status()?;
    }
    Ok(())
}
