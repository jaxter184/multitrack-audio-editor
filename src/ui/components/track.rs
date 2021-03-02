use tuix::*;
use crate::sample_player::*;
use crate::audio_stream::audio_stream;
use basedrop::Collector;
use cpal::{PlayStreamError, traits::StreamTrait};
use image::GenericImageView;
use native_dialog::FileDialog;
use std::cmp::Ordering;
use dasp_sample::{Sample, I24};
use femtovg::{
    renderer::OpenGl,
    Canvas,
    Paint,
    Path,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TrackEvent {
	Remove,
	OpenFile,
	OTHER,
}

pub struct Track {
	collector: Collector,
	controller: SamplePlayerController,
	num_of_channels: usize,
	sample_rate: f64,
	num_of_samples: usize,
	name: Entity,
}

impl Track {
    pub fn new(collector: Collector, controller: SamplePlayerController) -> Self {
		Self {
			collector,
			controller,
            num_of_samples: 0,
            num_of_channels: 0,
            sample_rate: 0.0,
            name: Entity::null(),
		}
	}
}

impl BuildHandler for Track {
	type Ret = Entity;
	fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
		let track_controls = VBox::new().build(state, entity, |builder| builder);
		let row = HBox::new().build(state, track_controls, |builder| builder.class("test"));
		Button::new().on_release(Event::new(TrackEvent::Remove).target(entity).origin(entity)).build(
			state, row, |builder| builder.set_text("X").class("remove_track")
		//	.set_width(Length::Pixels(30.0))
		//	.set_height(Length::Pixels(30.0))
		);
		self.name = Textbox::new("track name").build(
			state, row, |builder| builder.class("track_name")
		);
	//	row.set_flex_direction(state, FlexDirection::Column);
		let row = HBox::new().build(
			state, track_controls, |builder| builder.class("test")
			.set_flex_direction(FlexDirection::Row)
			.set_padding(Length::Pixels(6.0))
		);
		// Open file button
		Button::new()
			.on_release(Event::new(TrackEvent::OpenFile))
			.build(state, row, |builder| {
				builder
					.set_text("Open")
					.set_margin(Length::Pixels(10.0))
					.class("open")
			});

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
		track_controls.set_element(state, "track_controls").set_flex_direction(state, FlexDirection::Column);
		entity.set_element(state, "track").set_flex_direction(state, FlexDirection::Row)
	}
}

impl EventHandler for Track {
	fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
		if let Some(event) = event.message.downcast::<TrackEvent>() {
			use TrackEvent::*;
			match event {
				Remove=> {
					state.remove(entity);
				},
			//	LoadAudioFile(file_path) => { // Load an audio file specified on the command line
			//		//self.read_audio(file_path);
			//		self.controller.load_file(file_path);

			//		if let Some(file) = self.controller.file.as_ref() {
			//			self.num_of_channels = file.num_channels;
			//			self.sample_rate = file.sample_rate;
			//			self.num_of_samples = file.num_samples;
			//			println!("Length: {} ", self.num_of_samples);
			//		}
			//	},
				OpenFile => { // Load an audio file using a file dialog
					if let Some(file_path) = FileDialog::new()
					.show_open_single_file()
					.expect("Failed to open file dialog") {
						println!("File path = {:?}", file_path);

						//self.read_audio(file_path.as_os_str().to_str().unwrap());

						self.controller.load_file(file_path.as_os_str().to_str().unwrap());
						self.controller.seek(0.0);
						self.name.set_text(state, &file_path.file_stem().unwrap().to_str().unwrap());

						if let Some(file) = self.controller.file.as_ref() {
							self.num_of_channels = file.num_channels;
							self.sample_rate = file.sample_rate;
							self.num_of_samples = file.num_samples;
							println!("Length: {} ", file.num_samples);

						//	self.waveform_left.load(&file.data[0..self.num_of_samples], state.data.get_width(entity) as usize);
						//	self.waveform_right.load(&file.data[self.num_of_samples..self.num_of_samples*2], state.data.get_width(entity) as usize);
						}
					}
//					event.consume();
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

