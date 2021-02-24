
use tuix::*;

use super::transport::TransportBar;

const TOOLBAR_STYLE: &str = r#"
    toolbar {
        flex-basis: 100px;
        flex-direction: row;
    }
"#;

pub struct Toolbar {

}

impl Toolbar {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for Toolbar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // Fine to do this here because there will only be one header
        state.add_theme(TOOLBAR_STYLE);

        TransportBar::new().build(state, entity, |builder| builder.set_text("Transport Controls"));

        entity.set_element(state, "toolbar")
    }
}

impl EventHandler for Toolbar {

}