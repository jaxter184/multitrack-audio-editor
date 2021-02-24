

use tuix::*;


const EFFECTS_BAR_STYLE: &str = r#"
    effects_bar {
        flex-basis: 30px;
        transition: flex-basis 0.15 0.0;
    }

    effects_bar:hover {
        flex-basis: 200px;
        transition: flex-basis 0.15 0.0;
    }
"#;

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

        state.add_theme(EFFECTS_BAR_STYLE);

        entity.set_element(state, "effects_bar")
    }
}

impl EventHandler for EffectsBar {

}