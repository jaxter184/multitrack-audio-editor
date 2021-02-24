
use tuix::*;

use super::track::Track;


const TRACK_VIEW_STYLE: &str = r#"
    track_view {
        flex-grow: 1.0;
    }
"#;

const TRACK_STYLE: &str = r#"
    track {
        flex-basis: 100px;
    }

    track>.controls {
        flex-basis: 200px;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
pub enum TrackViewEvent {
    Add,
    Remove,
    ZoomIn,
    ZoomOut,
    PanLeft,
    PanRight,
}

pub struct TrackView {

}

impl TrackView {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for TrackView {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        state.add_theme(TRACK_VIEW_STYLE);
        state.add_theme(TRACK_STYLE);
        
        // Track View Header - to be made a widget
        Element::new().build(state, entity, |builder| builder.set_flex_basis(Length::Pixels(30.0)).set_text("Track View Header"));

        // Example track - to be moved
        Track::new().build(state, entity, |builder| builder);

        entity.set_element(state, "track_view")
    }
}

impl EventHandler for TrackView {

}