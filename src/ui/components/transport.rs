
use tuix::*;

const TRANSPORT_STYLE: &str = r#"
    transport {
        flex-basis: 300px;
    }
"#;

#[derive(Debug,Clone,PartialEq)]
pub enum TransportEvent {
    Play,
    Pause,
    Stop,
    Record,
}

pub struct TransportBar {

}

impl TransportBar {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for TransportBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        state.add_theme(TRANSPORT_STYLE);

        entity.set_element(state, "transport")
    }
}

impl EventHandler for TransportBar {

}