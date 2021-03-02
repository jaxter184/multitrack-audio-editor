
use tuix::*;

use super::track::Track;
use basedrop::Collector;
use crate::sample_player::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TrackViewEvent {
	Add,
	Remove,
	ZoomIn,
	ZoomOut,
	PanLeft,
	PanRight,
	AddTrack,
}

pub struct TrackView {
}

impl TrackView {
	pub fn new() -> Self {
		Self {
		}
	}
}

impl BuildHandler for TrackView {
	type Ret = Entity;
	fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        //self.rvbox = ResizableVBox::new().build(state, entity, |builder| {
        //    builder
        //        .set_width(Length::Pixels(900.0))
        //        .set_height(Length::Percentage(1.0))
        //        .class("container")
        //});

	//	let scroller = ScrollContainer::new().build(state, entity, |builder| builder);
		
		entity.set_element(state, "track_view")
	}
}

impl EventHandler for TrackView {
	fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
		if let Some(event) = event.message.downcast::<TrackViewEvent>() {
			match event {
				TrackViewEvent::AddTrack => {
				    // initialize gc
				    let gc = Collector::new();

				    // Create the sample player and controller
				    let (mut player, mut controller) = sample_player(&gc);
					Track::new(gc, controller).build(state, entity, |builder| builder);
				},
				_ => (),
			}
		}
		if let Some(window_event) = event.message.downcast::<WindowEvent>() {
			match window_event {
				WindowEvent::MouseDown(_button) => {
					if entity == state.hovered {
                        state.insert_event(Event::new(TrackViewEvent::AddTrack).target(entity).origin(entity));
					}
				//	state.focused = entity;
				},
			//	WindowEvent::KeyDown(code, _) => {
			//		match code {
			//			Code::KeyT => {
			//				Track::new().build(state, entity, |builder| builder);
			//			},
			//			_ => (),
			//		}
			//	},
				_ => (),
			}
		}
	}
}

