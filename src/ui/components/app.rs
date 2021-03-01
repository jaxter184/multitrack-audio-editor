use tuix::*;

use crate::{EffectsBar, TrackView};

//use super::toolbar::Toolbar;
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
		//let menu_bar = Element::new().build(state, entity, |builder|
		//	builder
		//		.set_flex_basis(Length::Pixels(30.0))
		//		.set_flex_direction(FlexDirection::Row)
		//);
		//Element::new().build(state, menu_bar, |builder| builder.set_text("File").set_flex_basis(Length::Pixels(100.0)));
		//Element::new().build(state, menu_bar, |builder| builder.set_text("Edit").set_flex_basis(Length::Pixels(100.0)));
		//Element::new().build(state, menu_bar, |builder| builder.set_text("View").set_flex_basis(Length::Pixels(100.0)));
		//Element::new().build(state, menu_bar, |builder| builder.set_text("Tracks").set_flex_basis(Length::Pixels(100.0)));
		//Element::new().build(state, menu_bar, |builder| builder.set_text("Help").set_flex_basis(Length::Pixels(100.0)));

		// Header
		//Toolbar::new().build(state, entity, |builder| builder.set_text("Tools"));
		// Create a tab container
		let (tab_bar, tab_container) = TabContainer::new().build(state, entity, |builder| builder);

		// Add a tab to the tab bar
		Button::with_label("Tracks")
			.on_press(Event::new(TabEvent::SwitchTab(0)))
			.build(state, tab_bar, |builder| builder.set_checked(true));
		// Tracks View
		TrackView::new().build(state, entity, |builder| builder.class("item1"));
		// Footer
	//	StatusBar::new().build(state, tab_container, |builder| builder.set_text("Status Bar"));

		Button::with_label("Effects")
			.on_press(Event::new(TabEvent::SwitchTab(1)))
			.build(state, tab_bar, |builder| builder);
		// Effects Bar
		EffectsBar::new().build(
			state, tab_container, |builder| builder
			.set_text("Effects Bar")
			.set_display(Display::None)
		);

		tab_container.set_flex_grow(state, 1.0);
		entity.set_flex_grow(state, 1.0)
	}
}

impl EventHandler for App {
	fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
		if let Some(window_event) = event.message.downcast::<WindowEvent>() {
			match window_event {
				WindowEvent::KeyDown(code, _) => {
					match code {
						Code::KeyQ => {
							panic!();
						},
						_ => (),
					}
				},
				_ => (),
			}
		}
	}
}
