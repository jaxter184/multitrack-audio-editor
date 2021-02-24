use tuix::*;

use crate::{EffectsBar, TrackView};

use super::toolbar::Toolbar;
use super::status::StatusBar;

pub struct App {

}

impl App {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for App {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // Menus (Replace with MenuBar widget when working)
        let menu_bar = Element::new().build(state, entity, |builder| 
            builder
                .set_flex_basis(Length::Pixels(30.0))
                .set_flex_direction(FlexDirection::Row)
        );
        Element::new().build(state, menu_bar, |builder| builder.set_text("File").set_flex_basis(Length::Pixels(100.0)));
        Element::new().build(state, menu_bar, |builder| builder.set_text("Edit").set_flex_basis(Length::Pixels(100.0)));
        Element::new().build(state, menu_bar, |builder| builder.set_text("View").set_flex_basis(Length::Pixels(100.0)));
        Element::new().build(state, menu_bar, |builder| builder.set_text("Tracks").set_flex_basis(Length::Pixels(100.0)));
        Element::new().build(state, menu_bar, |builder| builder.set_text("Help").set_flex_basis(Length::Pixels(100.0)));

        // Header
        Toolbar::new().build(state, entity, |builder| builder.set_text("Tools"));
        // Tracks View
        TrackView::new().build(state, entity, |builder| builder.set_text("Tracks View"));
        // Effects Bar
        EffectsBar::new().build(state, entity, |builder| builder.set_text("Effects Bar"));
        // Footer
        StatusBar::new().build(state, entity, |builder| builder.set_text("Status Bar"));

        entity.set_flex_grow(state, 1.0)
    }
}

impl EventHandler for App {

}