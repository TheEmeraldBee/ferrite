use ratatui::prelude::*;
use widgetui::*;

pub struct TargetChunk;
pub struct StatusChunk;

pub fn chunk(frame: Res<WidgetFrame>, mut chunks: ResMut<Chunks>) -> WidgetResult {
    let layouts = layout![frame.size(), (%100), (#3)];

    chunks.register_chunk::<TargetChunk>(layouts[0][0]);
    chunks.register_chunk::<StatusChunk>(layouts[1][0]);
    Ok(())
}
