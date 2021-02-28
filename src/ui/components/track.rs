
use tuix::*;

pub struct Track {

}

impl Track {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for Track {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        Element::new().build(state, entity, |builder| builder.set_text("Track Controls").class("controls"));

        entity.set_element(state, "track").set_flex_direction(state, FlexDirection::Row)
    }
}

impl EventHandler for Track {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if entity == state.hovered {
				        state.remove(entity);
                    }
                },
                _ => (),
            }
        }
    }
}
