use makefile_lossless::*;
use ratatui::widgets::{Block, List};
use std::{
    process::Command,
    sync::{Arc, Mutex},
};
use widgetui::*;

#[derive(State)]
pub struct MakefileState {
    pub targets: Vec<String>,
    pub run_target: Arc<Mutex<Option<String>>>,
    pub selected: usize,
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    mut events: ResMut<Events>,
    mut state: ResMut<MakefileState>,
) -> WidgetResult {
    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit();
    }

    let area = frame.size();
    frame.render_widget(
        List::new(state.targets.iter().enumerate().map(|(i, target)| {
            if state.selected == i {
                format!(" <  {} > ", target)
            } else {
                target.clone()
            }
        }))
        .block(Block::bordered().title("Targets")),
        area,
    );

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

    Ok(())
}

fn main() -> anyhow::Result<()> {
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
        })
        .widgets(render)
        .run()?;

    if let Some(target) = run_target.lock().unwrap().take() {
        Command::new("make").arg(target).status()?;
    }
    Ok(())
}
