//! 🔊 Audio Player Component
//!
//! Strategy: `rodio` + `symphonia` for decode & playback. UI controls via GPUI.
//! Talks directly to OS-native audio APIs — CoreAudio / WASAPI / ALSA.

use gpui::*;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub struct AudioPlayerView {
    sink: Option<Arc<Sink>>,
    _stream: Option<OutputStream>,
    _stream_handle: Option<OutputStreamHandle>,
    file_path: String,
    is_playing: bool,
    volume: f32,
    track_name: String,
}

impl AudioPlayerView {
    pub fn new(file_path: &str, _window: &Window, _cx: &mut Context<Self>) -> Self {
        let track_name = std::path::Path::new(file_path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".into());

        Self {
            sink: None,
            _stream: None,
            _stream_handle: None,
            file_path: file_path.to_string(),
            is_playing: false,
            volume: 0.75,
            track_name,
        }
    }

    fn play(&mut self, cx: &mut Context<Self>) {
        if self.sink.is_some() {
            // Resume
            if let Some(ref sink) = self.sink {
                sink.play();
            }
            self.is_playing = true;
            cx.notify();
            return;
        }

        // Create new audio output stream
        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                // Load and decode the audio file
                if let Ok(file) = File::open(&self.file_path) {
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        sink.set_volume(self.volume);
                        sink.append(source);
                        self.sink = Some(Arc::new(sink));
                        self._stream = Some(stream);
                        self._stream_handle = Some(stream_handle);
                        self.is_playing = true;
                    }
                }
            }
        }
        cx.notify();
    }

    fn pause(&mut self, cx: &mut Context<Self>) {
        if let Some(ref sink) = self.sink {
            sink.pause();
        }
        self.is_playing = false;
        cx.notify();
    }

    fn stop(&mut self, cx: &mut Context<Self>) {
        if let Some(ref sink) = self.sink {
            sink.stop();
        }
        self.sink = None;
        self._stream = None;
        self._stream_handle = None;
        self.is_playing = false;
        cx.notify();
    }

    fn set_volume(&mut self, vol: f32, cx: &mut Context<Self>) {
        self.volume = vol.clamp(0.0, 1.0);
        if let Some(ref sink) = self.sink {
            sink.set_volume(self.volume);
        }
        cx.notify();
    }
}

impl Render for AudioPlayerView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let vol_pct = (self.volume * 100.0) as u32;

        div()
            .v_flex()
            .gap_4()
            .p_6()
            .w(px(400.))
            .bg(rgb(0x1e1e2e))
            .rounded(px(12.))
            .shadow_lg()
            .child(
                div()
                    .text_color(rgb(0xcdd6f4))
                    .text_xl()
                    .child("🔊 Audio Player"),
            )
            // Track info
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .child(
                        div()
                            .text_color(rgb(0xcdd6f4))
                            .child(format!("🎵 {}", self.track_name)),
                    )
                    .child(
                        div()
                            .text_color(rgb(0x6c7086))
                            .text_sm()
                            .child(self.file_path.clone()),
                    ),
            )
            // Playback controls
            .child(
                div()
                    .h_flex()
                    .gap_2()
                    .justify_center()
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(if self.is_playing {
                                rgb(0xf9e2af)
                            } else {
                                rgb(0xa6e3a1)
                            })
                            .text_color(rgb(0x1e1e2e))
                            .rounded(px(6.))
                            .cursor_pointer()
                            .child(if self.is_playing { "⏸ Pause" } else { "▶ Play" })
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.notify();
                            }),
                    )
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(rgb(0xf38ba8))
                            .text_color(rgb(0x1e1e2e))
                            .rounded(px(6.))
                            .cursor_pointer()
                            .child("⏹ Stop")
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.notify();
                            }),
                    ),
            )
            // Volume
            .child(
                div()
                    .h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div().text_color(rgb(0xa6adc8)).child("🔉"),
                    )
                    .child(
                        // Volume bar (visual)
                        div()
                            .flex_1()
                            .h(px(8.))
                            .bg(rgb(0x313244))
                            .rounded(px(4.))
                            .child(
                                div()
                                    .h_full()
                                    .w(relative(self.volume))
                                    .bg(rgb(0x89b4fa))
                                    .rounded(px(4.)),
                            ),
                    )
                    .child(
                        div()
                            .text_color(rgb(0xa6adc8))
                            .text_sm()
                            .child(format!("{}%", vol_pct)),
                    ),
            )
    }
}
