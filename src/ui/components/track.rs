use tuix::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TrackEvent {
	Remove,
}

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

		TrackControls::new(Event::new(TrackEvent::Remove).target(entity).origin(entity)).build(state, entity, |builder| builder);

		entity.set_element(state, "track").set_flex_direction(state, FlexDirection::Row)
	}
}

impl EventHandler for Track {
	fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
		if let Some(event) = event.message.downcast::<TrackEvent>() {
			match event {
				TrackEvent::Remove=> {
					state.remove(entity);
				},
				_ => (),
			}
		}
	//	if let Some(window_event) = event.message.downcast::<WindowEvent>() {
	//		match window_event {
	//			WindowEvent::MouseDown(button) => {
	//				if entity == state.hovered {
	//					state.insert_event(Event::new(TrackEvent::Remove).target(entity).origin(entity));
	//				}
	//			},
	//			_ => (),
	//		}
	//	}
	}
}

pub struct TrackControls {
	pub button_event: Event,
}

impl TrackControls {
	pub fn new(event: Event) -> Self {
		Self {
			button_event: event,
		}
	}
}

impl BuildHandler for TrackControls {
	type Ret = Entity;
	fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
		let col = VBox::new().build(state, entity, |builder| builder);
		let row = HBox::new().build(state, col, |builder| builder);
		Button::new().on_release(self.button_event.clone()).build(
			state, row, |builder| builder.set_text("X").class("remove_track")
		//	.set_width(Length::Pixels(30.0))
		//	.set_height(Length::Pixels(30.0))
		);
		Textbox::new("track name").build(
			state, row, |builder| builder.class("track_name")
		);
	//	row.set_flex_direction(state, FlexDirection::Column);
		let row = HBox::new().build(
			state, col, |builder| builder
			.set_flex_direction(FlexDirection::Row)
		);
		Slider::new().build(
			state, row, |builder| builder
			.set_height(Length::Pixels(40.0))
			.set_width(Length::Pixels(200.0))
		);
		ControlKnob::new(0.0, -12.0, 12.0).build(
			state, row, |builder| builder
			.set_width(Length::Pixels(40.0))
			.set_height(Length::Pixels(40.0))
		);
	//	row.set_flex_direction(state, FlexDirection::Column);
		entity.set_element(state, "track_controls").set_flex_direction(state, FlexDirection::Row)
	}
}

impl EventHandler for TrackControls {
}
