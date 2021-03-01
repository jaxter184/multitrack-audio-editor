
use tuix::*;

use super::track::Track;


const TRACK_VIEW_STYLE: &str = r#"
	track_view {
		flex-grow: 1.0;
		background-color: #181818;
	}
"#;

const TRACK_STYLE: &str = r#"
	track {
		flex-basis: 100px;
		border-width: 0px;
		background-color: #242424;
	}

	track>.track_controls {
		flex-basis: 200px;
		margin: 3px;
		background-color: #363636;
		border-width: 2px;
		border-color: #727272;
	}
	knob {
		background-color: #505050;
	}

	knob>.back {
		background-color: #353535;
	}

	knob>.slider {
		background-color: #f74c00;
	}

	.track_name {
		background-color: #363636;
		border-radius: 6px;
		margin: 3.0;
		background_color: 0x36,0x36,0x36;
		height: 30.0;
		width: 200.0;
	}

	eqchannel:disabled knob>.slider {
		background-color: #353535;
	}

	knob>.tick {
		background-color: #c8c8c8;
	}

	eqchannel:disabled knob>.tick {
		background-color: #505050;
	}
	button.remove_track {
		background-color: #ff2080;
		margin: 5px;
		width: 20px;
		height: 20px;
		border-radius: 2px;

		border-width: 0px; /* delete */
	}
"#;

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
	rvbox: Entity,
}

impl TrackView {
	pub fn new() -> Self {
		Self {
			rvbox: Entity::null(),
		}
	}
}

impl BuildHandler for TrackView {
	type Ret = Entity;
	fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

		state.add_theme(TRACK_VIEW_STYLE);
		state.add_theme(TRACK_STYLE);

        //self.rvbox = ResizableVBox::new().build(state, entity, |builder| {
        //    builder
        //        .set_width(Length::Pixels(900.0))
        //        .set_height(Length::Percentage(1.0))
        //        .class("container")
        //});

	//	let scroller = ScrollContainer::new().build(state, entity, |builder| builder);
		
		Track::new().build(state, entity, |builder| builder);

		entity.set_element(state, "track_view")
	}
}

impl EventHandler for TrackView {
	fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
		if let Some(event) = event.message.downcast::<TrackViewEvent>() {
			match event {
				TrackViewEvent::AddTrack => {
					Track::new().build(state, entity, |builder| builder);
				},
				_ => (),
			}
		}
		if let Some(window_event) = event.message.downcast::<WindowEvent>() {
			match window_event {
				WindowEvent::MouseDown(button) => {
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

