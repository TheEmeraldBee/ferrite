use ratatui::{
    style::Stylize,
    widgets::{Block, List, Paragraph},
};
use widgetui::{Chunks, Res, ResMut, WidgetFrame, WidgetResult};

use crate::{state::MakefileState, StatusChunk, TargetChunk};

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    state: Res<MakefileState>,
    chunks: Res<Chunks>,
) -> WidgetResult {
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

    Ok(())
}

pub fn status(
    mut frame: ResMut<WidgetFrame>,
    state: Res<MakefileState>,
    chunks: Res<Chunks>,
) -> WidgetResult {
    let area = chunks.get_chunk::<StatusChunk>()?;
    if let Some(last) = state.last_target.clone() {
        frame.render_widget(
            Paragraph::new(format!(
                "<jk> - Traverse Tasks, <Esc/q> -- Quit, <r> - Run Last Task ---- Last - {}",
                last
            ))
            .block(Block::bordered()),
            area,
        );
    } else {
        frame.render_widget(
            Paragraph::new(format!("<jk> - Traverse Tasks, <Esc/q> - Quit",))
                .block(Block::bordered()),
            area,
        );
    }

    Ok(())
}
