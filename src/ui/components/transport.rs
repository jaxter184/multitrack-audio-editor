use tuix::*;
const ICON_PLAY: &str = "\u{25b6}";
const ICON_PAUSE: &str = "\u{2389}";
const ICON_STOP: &str = "\u{25a0}";
const ICON_LOOP: &str = "\u{1f501}";

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
        entity.set_element(state, "transport")
    }
}

impl EventHandler for TransportBar {

}

