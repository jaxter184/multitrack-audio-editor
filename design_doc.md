# multitrack-audio-editor (Name in progress) Design Document

RustyDAW is a community-driven project. If you are interested in participating, feel free to join the RustAudio Discord server, you can find us in the channels #rusty-daw and #rusty-daw-gui.

# Overview

Why create a multitrack audio editor? Don't we want to create a DAW?

We want to limit the scope of our projects before we create the big stuff. A multitrack audio editor (like Audacity) is an excellent staring point to experiment and design our framework. With Rust's modularity, many parts can be reused for a full DAW project.

This program's features and workflow will be heavily inspired by [`Audacity`], a popular and well-established FOSS audio editor.

# Goals

## Overall UI
* three views: timeline, effects, & mixer. Use tabs to select between these view
* undo/redo with buttons
* save button

## Recording
* Realtime-recording of audio into tracks, with the ability to record from multiple sources at once
* Loop-recording (comping)
### UI
* On each track, display a an "arm" button. When clicked, show a menu where the user can select the source (microphone, etc.)
* Prominent record button. When no track is currently armed, gray this out to show that a track needs to be armed first.
* When a track or multiple tracks are armed, the record button is un-grayed out.
* When the record button is pressed, recording starts and the record button flashes red.
* Menu for choosing record mode (loop with comping, no looping)
* When comping, automatically add track underneath the current track when recording a new take
* When comping, automatically put current track and any new tracks into its own track folder
* A menu to select recording quality (float32, 24bit PCM, 16bit PCM, 8bit PCM)

## Playback
* Realtime playback of audio, with loop points
### UI
* Play, pause, stop buttons
* A large display of the current time of the playhead
* Playhead on the timeline can be dragged with the mouse
* When double-clicking on the playhead handle, show a text box for entering the time with the keyboard
* A menu to select between returning the playhead to the start of the file when playback stops, returning it to the last start position, or keeping it where it is.
* A button to enable/disable looping
* Loop points on the timeline that can be dragged with the mouse
* When double-cliking on a loop point handle, show a text box for entering the time with the keyboard

## Timeline UI
* Show various points in time on top of the timeline. When zooming in, more points will be added to show more precision. When zooming out, remove those points to avoid cluttering this display.
* Add/remove custom markers on top of timeline. Edit the text displayed in the custom marker with keyboard, & add color selector for color coding.
* Display waveforms of audio clips
* Use mouse wheel to scroll up and down
* Zoom in-and-out by dragging up-and-down on the timeline header (like Bitwig)
* Scroll horizontally by dragging left-and-right on the timeline header (like Bitwig)
* Also add a horizontal scrollbar as another options to move left and right.
* Put zoom controls in menu (zoom in, zoom out, select zoom from drop-down, default zoom)
* An audio file can be dragged from outside onto a new track

## Track headers UI
* Track name, double-click on this to use keyboard to type name
* Color selector for color-coding tracks
* Arm track button (with text displaying selected audio source). This will flash red while recording
* Peak/RMS meters. When a track is armed, show the levels of the current audio source (even while not recording)
* Fader & pan controls
* Add/remove tracks
* Duplicate tracks
* Arrange order of tracks with mouse or keyboard
* Folder tracks for organization
* Separate stereo track into two mono tracks
* Join two mono tracks into one stereo track

## Multitrack Editing UI
* Slice audio clips with mouse (by selecting a slice button in the menu)
* Moving audio clips with mouse or keyboard across the track
* Move audio clips into other tracks
* Cross-fade controls on the ends of each audio clip
* Create/remove tracks
* Select area with mouse and delete section by splitting the clips and adding silence in the middle
* Select area with mouse and delete section by splitting clips and moving everyting on the right over (no silence in middle)
* Comping editing by quickly selecting wich takes to use in a given area.

## Effects View UI
* select an area of an audio clip on the timeline view with the mouse
* go the the "effects" view (alternatively, simple effects like gain and replace with beep can have a keyboard shortcut and a pop-up dialog)
* In the effects view, show a large display of the isolated waveform in the selection
* a menu for effects like gain, pan, fade, clip, reverse, stretch, and pitch that can be applied to the selection
* a menu to replace current selection with a beep or noise (This could be a keyboard shortcut on the timeline view as well)
* undo/redo/apply effects

## Mixer View UI
* horizontal mixer with faders, pan control, and peak/rms meter for each track
* Slots in each track where an LV2 (and possibly VST if we can get the licensing) can be instered. Only effect plugins will be supported, no midi or synthesizers.
* Master track

## Export Menu
* A dialog for exporting to a wav file (with more codecs later)

## Settings Menu
* set up audio devices
* select theme
* select language (we only support English for now)
* autosave and backup save settings

# Non-Goals

This is meant to be a simpler project to experiment and design our core framework. This is *not* a full DAW, so we will not include:
* MIDI recording & playback
* MIDI editor (like a piano roll)
* Automation & automation tracks
* Synthesizer plugin hosting
* A suite of built-in plugin effects (except for possibly basic EQ, compression and analyzers)
* Sample & preset browser

[`RustAudio Discord`]: https://discord.com/invite/8rPCp9Q
[`Audacity`]: https://www.audacityteam.org/

