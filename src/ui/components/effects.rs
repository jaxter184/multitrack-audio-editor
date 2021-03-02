use tuix::*;

pub struct EffectsBar {

}

impl EffectsBar {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for EffectsBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "effects_bar")
    }
}

impl EventHandler for EffectsBar {

}

