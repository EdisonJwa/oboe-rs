use std::f32::consts::PI;

use oboe::{
    AudioOutputCallback, AudioOutputStreamSafe, AudioStream, AudioStreamAsync, AudioStreamBase,
    AudioStreamBuilder, AudioStreamSafe, DataCallbackResult, Mono, Output, PerformanceMode,
    SharingMode,
};

struct SineWave {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: f32,
    initialized: bool,
}

impl SineWave {
    fn new(frequency: f32, gain: f32) -> Self {
        Self {
            frequency,
            gain,
            phase: 0.0,
            delta: 0.0,
            initialized: false,
        }
    }
}

impl AudioOutputCallback for SineWave {
    type FrameType = (f32, Mono);

    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [f32],
    ) -> DataCallbackResult {
        if !self.initialized {
            let sample_rate = stream.get_sample_rate() as f32;
            self.delta = self.frequency * 2.0 * PI / sample_rate;
            self.initialized = true;
            log::info!(
                "Sine callback initialized: rate={}, delta={:.6}",
                sample_rate,
                self.delta
            );
        }

        for frame in frames {
            *frame = self.gain * self.phase.sin();
            self.phase += self.delta;
            while self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }

        DataCallbackResult::Continue
    }
}

fn run_sine() -> Result<(), oboe::Error> {
    let mut stream: AudioStreamAsync<Output, SineWave> = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<f32>()
        .set_channel_count::<Mono>()
        .set_callback(SineWave::new(440.0, 0.5))
        .open_stream()?;

    log::info!("Stream opened successfully");
    log::info!("  Sample rate:     {}", stream.get_sample_rate());
    log::info!("  Channel count:   {:?}", stream.get_channel_count());
    log::info!("  Format:          {:?}", stream.get_format());
    log::info!("  Frames per burst: {}", stream.get_frames_per_burst());
    log::info!(
        "  Buffer capacity: {}",
        stream.get_buffer_capacity_in_frames()
    );
    log::info!("  Buffer size:     {}", stream.get_buffer_size_in_frames());
    log::info!("  Performance mode: {:?}", stream.get_performance_mode());
    log::info!("  Sharing mode:    {:?}", stream.get_sharing_mode());
    log::info!("  XRun supported:  {}", stream.is_xrun_count_supported());

    stream.start()?;
    log::info!("Stream started — playing 440 Hz sine for 5 seconds");

    std::thread::sleep(std::time::Duration::from_secs(5));

    let xrun_count = stream.get_xrun_count();
    log::info!("  XRun count:      {:?}", xrun_count);

    stream.stop()?;
    stream.close()?;

    log::info!("Stream stopped and closed");
    Ok(())
}

#[cfg(target_os = "android")]
#[no_mangle]
unsafe extern "C" fn ANativeActivity_onCreate(
    activity: *mut ndk_sys::ANativeActivity,
    saved_state: *mut u8,
    saved_state_size: usize,
) {
    ndk_glue::init(activity, saved_state, saved_state_size, android_main);
}

#[cfg(target_os = "android")]
fn android_main() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("oboe-sine"),
    );

    log::info!("oboe-sine-example starting on Android");

    if let Err(e) = run_sine() {
        log::error!("Error: {:?}", e);
    }

    log::info!("oboe-sine-example finished");
}

#[cfg(not(target_os = "android"))]
fn main() {
    eprintln!("This example only runs on Android. Build with:");
    eprintln!("  cargo ndk --platform 21 --target aarch64-linux-android -- build --release -p oboe-sine-example");
}
