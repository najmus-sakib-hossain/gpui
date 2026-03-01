//! 🎬 VIDEO Player Component
//!
//! Strategy: `symphonia` demuxes the container → `openh264` decodes H.264 frames → RGBA → GPUI img.
//! Pipeline: Decode frames → RGBA buffer → paint as GPUI custom Element texture every frame.

use gpui::*;
use gpui_component::StyledExt;
use openh264::formats::YUVSource;
use image::RgbaImage;
use openh264::decoder::Decoder;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::bitmap_utils;

/// Holds decoded video frames and playback state
pub struct VideoPlayerView {
    frames: Arc<Mutex<Vec<RgbaImage>>>,
    current_frame: usize,
    fps: f64,
    is_playing: bool,
    last_frame_time: Instant,
    file_path: String,
    loaded: bool,
}

impl VideoPlayerView {
    pub fn new(file_path: &str, _window: &Window, cx: &mut Context<Self>) -> Self {
        let frames = Arc::new(Mutex::new(Vec::new()));
        let frames_clone = frames.clone();
        let path = file_path.to_string();

        // Decode video in background thread
        cx.spawn(async move |this: WeakEntity<VideoPlayerView>, cx| {
            let decoded = smol::unblock(move || {
                decode_h264_video(&path)
            }).await;

            if let Ok(decoded_frames) = decoded {
                *frames_clone.lock().unwrap() = decoded_frames;
                cx.update(|cx| {
                    this.update(cx, |view, cx: &mut Context<VideoPlayerView>| {
                        view.loaded = true;
                        cx.notify();
                    })
                }).ok();
            }
        })
        .detach();

        Self {
            frames,
            current_frame: 0,
            fps: 30.0,
            is_playing: false,
            last_frame_time: Instant::now(),
            file_path: file_path.to_string(),
            loaded: false,
        }
    }

    fn toggle_playback(&mut self, cx: &mut Context<Self>) {
        self.is_playing = !self.is_playing;
        if self.is_playing {
            self.last_frame_time = Instant::now();
            self.schedule_next_frame(cx);
        }
    }

    fn schedule_next_frame(&mut self, cx: &mut Context<Self>) {
        let frame_duration = Duration::from_secs_f64(1.0 / self.fps);
        cx.spawn(async move |this: WeakEntity<VideoPlayerView>, cx| {
            smol::Timer::after(frame_duration).await;
            cx.update(|cx| {
                this.update(cx, |view, cx: &mut Context<VideoPlayerView>| {
                    if view.is_playing {
                        let frame_count = view.frames.lock().unwrap().len();
                        if frame_count > 0 {
                            view.current_frame = (view.current_frame + 1) % frame_count;
                        }
                        view.schedule_next_frame(cx);
                        cx.notify();
                    }
                })
            }).ok();
        })
        .detach();
    }
}

impl Render for VideoPlayerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let frames = self.frames.lock().unwrap();
        let toggle = cx.listener(|this, _: &MouseDownEvent, _, cx| this.toggle_playback(cx));

        div()
            .v_flex()
            .gap_3()
            .items_center()
            .p_4()
            .child(
                div()
                    .text_color(rgb(0xcdd6f4))
                    .text_xl()
                    .child("🎬 Video Player"),
            )
            .child(
                // Video file info
                div()
                    .text_color(rgb(0x6c7086))
                    .text_sm()
                    .child(if self.file_path.is_empty() {
                        "No file loaded — pass a .mp4 path".to_string()
                    } else {
                        self.file_path.clone()
                    }),
            )
            .child(
                // Video display area
                div()
                    .w(px(640.))
                    .h(px(360.))
                    .bg(rgb(0x000000))
                    .rounded(px(8.))
                    .overflow_hidden()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(if let Some(frame) = frames.get(self.current_frame) {
                        let source = bitmap_utils::rgba_to_gpui_image(frame);
                        div().child(
                            img(source)
                                .w(px(640.))
                                .h(px(360.))
                                .object_fit(ObjectFit::Contain),
                        )
                    } else {
                        div()
                            .text_color(rgb(0x6c7086))
                            .child(if self.loaded {
                                "No frames decoded"
                            } else {
                                "Loading video..."
                            })
                    }),
            )
            .child(
                // Playback controls
                div()
                    .h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x89b4fa))
                            .text_color(rgb(0x1e1e2e))
                            .rounded(px(6.))
                            .cursor_pointer()
                            .child(if self.is_playing { "⏸ Pause" } else { "▶ Play" })
                            .on_mouse_down(MouseButton::Left, toggle),
                    )
                    .child(
                        div()
                            .text_color(rgb(0xa6adc8))
                            .child(format!(
                                "Frame {}/{}",
                                self.current_frame + 1,
                                frames.len()
                            )),
                    ),
            )
    }
}

/// Decode H.264 video using symphonia + openh264
fn decode_h264_video(path: &str) -> anyhow::Result<Vec<RgbaImage>> {
    // 1. Open the media file with symphonia (demux)
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if path.ends_with(".mp4") {
        hint.with_extension("mp4");
    }

    let probed = symphonia::default::get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    let mut format_reader = probed.format;
    let video_track = format_reader
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .ok_or_else(|| anyhow::anyhow!("No video track found"))?
        .clone();

    let track_id = video_track.id;

    // 2. Initialize OpenH264 decoder
    let mut h264_decoder = Decoder::new()?;
    let mut frames = Vec::new();

    // 3. Read packets and decode
    while let Ok(packet) = format_reader.next_packet() {
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the H.264 NAL units
        if let Some(yuv) = h264_decoder.decode(&packet.data)? {
            let (width, height) = yuv.dimensions();
            let mut rgb_buf = vec![0u8; width * height * 3];
            yuv.write_rgb8(&mut rgb_buf);

            // Convert RGB → RGBA
            let mut rgba_buf = Vec::with_capacity(width * height * 4);
            for chunk in rgb_buf.chunks(3) {
                rgba_buf.push(chunk[0]); // R
                rgba_buf.push(chunk[1]); // G
                rgba_buf.push(chunk[2]); // B
                rgba_buf.push(255);       // A
            }

            if let Some(img) =
                RgbaImage::from_raw(width as u32, height as u32, rgba_buf)
            {
                frames.push(img);
            }
        }
    }

    Ok(frames)
}
