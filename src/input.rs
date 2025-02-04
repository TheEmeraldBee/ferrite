use ratatui::crossterm::event::KeyCode;
use widgetui::{Events, Res, ResMut, WidgetResult};

use crate::state::MakefileState;

pub fn exit(mut events: ResMut<Events>) -> WidgetResult {
    if events.key(KeyCode::Char('q')) || events.key(KeyCode::Esc) {
        events.register_exit();
    }

    Ok(())
}

pub fn selection(mut state: ResMut<MakefileState>, events: Res<Events>) -> WidgetResult {
    if events.key(KeyCode::Char('j')) || events.key(KeyCode::Down) {
        if state.selected < state.targets.len() - 1 {
            state.selected += 1;
        }
    }

    if events.key(KeyCode::Char('k')) || events.key(KeyCode::Up) {
        if state.selected > 0 {
            state.selected -= 1;
        }
    }

    Ok(())
}

pub fn runner(mut events: ResMut<Events>, state: Res<MakefileState>) -> WidgetResult {
    if events.key(KeyCode::Enter) || events.key(KeyCode::Char(' ')) {
        state
            .run_target
            .lock()
            .unwrap()
            .replace(state.targets[state.selected].clone());
        events.register_exit();
    } else if events.key(KeyCode::Char('r')) && state.last_target.is_some() {
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
