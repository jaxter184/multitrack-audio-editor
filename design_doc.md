# multitrack-audio-editor (Name in progress) Design Document

RustyDAW is a community-driven project. If you are interested in participating, feel free to join the RustAudio Discord server, you can find us in the channels #rusty-daw and #rusty-daw-gui.

# Overview

Why create a multitrack audio editor? Don't we want to create a DAW?

We want to limit the scope of our projects before we create the big stuff. A multitrack audio editor (like Audacity) is an excellent staring point to experiment and design our framework. With Rust's modularity, many parts can be reused for a full DAW project.

This program's features and workflow will be heavily inspired by [`Audacity`], a popular and well-established FOSS audio editor.

# Milestones

# Stage 1

## Playback on a single track
* Define and implement a backend "sampler" that can take "audio clips" that point to areas in a file and play them back in order.
* Have the backend "sampler" playback a looped region.
* Support adding/removing "audio clips" onto the track.

## Timeline UI
* Show various points in time on top of the timeline. When zooming in, more points will be added to show more precision. When zooming out, remove those points to avoid cluttering this display
* Add grid lines to timeline
* Create a single track that can display waveforms of audio clips
* Use mouse wheel to scroll up and down
* Zoom in-and-out by dragging up-and-down on the timeline header (like Bitwig)
* Scroll horizontally by dragging left-and-right on the timeline header (like Bitwig)
* Also add a horizontal scrollbar as another options to move left and right.
* Add playhead that can be dragged with mouse

# Stage 2

## Backend
* Have multiple backend "samplers" that playback together and mix into a master output.
* Support adding/removing these "samplers" (tracks).
* Support recording audio into a new track (no loop recording yet).
* Support recording audio into multiple tracks from multiple sources.

## Main toolbar UI (These all don't need to be functional yet)
* save, undo, redo buttons
* Play/Pause, Stop buttons
* Toggle to enable/disable looping on playback
* Prominent record button. When no track is currently armed, gray this out to show that a track needs to be armed first.
  * When a track or multiple tracks are armed, the record button is un-grayed out.
  * When the record button is pressed, recording starts and the record button flashes red.
* A menu for selecting playhead behavior when stopping playback (seek to beginning of file, seek to beginning of playback, don't seek).
* A clock display
* A master peak/rms meter
* A place for tool icons (slice, trim, etc.)
* Timeline zoom controls (zoom in, zoom out, select zoom from drop-down, default zoom)
* Timeline/Mixer tabs

# Stage 3

## Backend
* Define and create a framework for a simple effect graph (a single effect per track).
* Define and support basic "effect" clips that are processed after loading the raw samples from file.
  * Create simple effects like gain, pan, and silence.

## Track header UI (These all don't need to be functional yet)
* Track name. Double-click on this to edit with keyboard.
* Fader & pan controls
* Display an "arm" button. When clicked, show a menu or dialog where the user can select the source (microphone, etc.)
* Add/remove tracks
* Duplicate track
* Arrange order of tracks by dragging with mouse
* Adjust height of track by dragging bottom border with mouse

# Stage 4

## Backend
* Export resampling
* Export dithering

## Multitrack editing
* split tool that splits one audio clip into two
* highlight area with mouse
* delete highlighted area with silence button in toolbar (or delete key)
* trim highlighted are with trim button in toolbar
* drag clips left and right with mouse
* drag clips between tracks
* Fade controls on the ends of each clip

# Stage 5

## UI
* Import audio clip into a new track.

## Export Menu UI
* A dialog for exporting to a wav file (with more codecs later)

## Settings Menu UI
* set up audio devices
* select theme
* select language (we only support English for now)
* autosave and backup save settings

# End of MVP!

# Stage 6

## Backend
* Create functioning audio graph system that can process multiple layers of effect clips on a single track.
* Resampling audio files to match project sample rate. This resampling process will be inserted into the audio graph before other effects are applied.

## Effect Clips UI
* Add ability to insert layers of "effect" clips that appear below the waveform view of the track.
* Resize and move these "effect" clips
* When an effect clip is selected, show a panel somewhere with the controls of that effect.
* Create the panel controls for basic effects (gain, pan, silence)
* Fade controls on the ends of each effect clip

# Stage 7

## Backend
* Create more basic effects (like noise, beep, clip, etc.)
* Enable hosting LV2 effect plugins (and possibly VST?). Only support audio effects. No MIDI or automation.

## Mixer View UI
* horizontal mixer view of each track (including master track)
* fader & pan controls on each track
* peak/rms meters on each track
* several vertical slots above each track where a plugin can be inserted
* a generic plugin view

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

