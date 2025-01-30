use makefile_lossless::*;
use ratatui::{prelude::*, widgets::*};
use std::{
    io::{Read, Write},
    process::Command,
    sync::{Arc, Mutex},
};
use style::Stylize;
use widgetui::*;

#[derive(State)]
pub struct MakefileState {
    pub targets: Vec<String>,
    pub run_target: Arc<Mutex<Option<String>>>,
    pub selected: usize,
    pub last_target: Option<String>,
}

pub struct TitleChunk;
pub struct TargetChunk;
pub struct StatusChunk;

pub fn chunk(frame: Res<WidgetFrame>, mut chunks: ResMut<Chunks>) -> WidgetResult {
    let layouts = layout![frame.size(), (%100), (#3)];

    chunks.register_chunk::<TargetChunk>(layouts[0][0]);
    chunks.register_chunk::<StatusChunk>(layouts[1][0]);
    Ok(())
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    mut events: ResMut<Events>,
    mut state: ResMut<MakefileState>,
    chunks: Res<Chunks>,
) -> WidgetResult {
    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit();
    }

    let area = chunks.get_chunk::<TargetChunk>()?;
    frame.render_widget(
        List::new(state.targets.iter().enumerate().map(|(i, target)| {
            if state.selected == i {
                format!(" <  {} > ", target)
            } else {
                target.clone()
            }
        }))
        .block(Block::bordered().title("Ferrite")),
        area,
    );

    let area = chunks.get_chunk::<StatusChunk>()?;
    if let Some(last) = state.last_target.clone() {
        frame.render_widget(
            Paragraph::new(format!(
                "{} - Traverse Tasks, {} - Run Last Task ---- Last - {}",
                "<r>".light_blue(),
                "<jk>".green(),
                last.red()
            ))
            .block(Block::bordered()),
            area,
        );
    } else {
        frame.render_widget(
            Paragraph::new(format!("<jk> - Traverse Tasks",)).block(Block::bordered()),
            area,
        );
    }

    if events.key(crossterm::event::KeyCode::Char('j'))
        || events.key(crossterm::event::KeyCode::Down)
    {
        if state.selected < state.targets.len() - 1 {
            state.selected += 1;
        }
    }

    if events.key(crossterm::event::KeyCode::Char('k')) || events.key(crossterm::event::KeyCode::Up)
    {
        if state.selected > 0 {
            state.selected -= 1;
        }
    }

    if events.key(crossterm::event::KeyCode::Enter)
        || events.key(crossterm::event::KeyCode::Char(' '))
    {
        state
            .run_target
            .lock()
            .unwrap()
            .replace(state.targets[state.selected].clone());
        events.register_exit();
    }
    if events.key(crossterm::event::KeyCode::Char('r')) && state.last_target.is_some() {
        state.run_target.lock().unwrap().replace(
            state
                .last_target
                .clone()
                .expect("Last Target should be some"),
        );
        events.register_exit();
    }

    Ok(())
}

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
        .widgets((chunk, render))
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
