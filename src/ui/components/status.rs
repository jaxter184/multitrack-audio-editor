
use tuix::*;

const STATUS_BAR_STYLE: &str = r#"
    status_bar {
        flex-basis: 30px;
    }
"#;

pub struct StatusBar {

}

impl StatusBar {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for StatusBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // Fine to do this here because there will only be one status bar
        state.add_theme(STATUS_BAR_STYLE);
        
        entity.set_element(state, "status_bar")
    }
}

impl EventHandler for StatusBar {

}