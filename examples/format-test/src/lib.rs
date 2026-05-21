use std::f32::consts::PI;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use oboe::{
    AudioInputCallback, AudioInputStreamSafe, AudioOutputCallback, AudioOutputStreamSafe,
    AudioStream, AudioStreamBase, AudioStreamBuilder, AudioStreamSafe, ContentType,
    DataCallbackResult, Error, InputPreset, Mono, PerformanceMode, PrivacySensitiveMode,
    SharingMode, SpatializationBehavior, Stereo, Usage,
};

fn log_stream_info<S: AudioStreamSafe + AudioStreamBase + AudioStream>(
    stream: &mut S,
    label: &str,
) {
    log::info!("[{}] stream info:", label);
    log::info!("[{}]   sample_rate={}", label, stream.get_sample_rate());
    log::info!(
        "[{}]   channel_count={:?}",
        label,
        stream.get_channel_count()
    );
    log::info!("[{}]   format={:?}", label, stream.get_format());
    log::info!(
        "[{}]   frames_per_burst={}",
        label,
        stream.get_frames_per_burst()
    );
    log::info!(
        "[{}]   buffer_capacity={}",
        label,
        stream.get_buffer_capacity_in_frames()
    );
    log::info!(
        "[{}]   buffer_size={}",
        label,
        stream.get_buffer_size_in_frames()
    );
    log::info!(
        "[{}]   performance_mode={:?}",
        label,
        stream.get_performance_mode()
    );
    log::info!("[{}]   sharing_mode={:?}", label, stream.get_sharing_mode());
    log::info!(
        "[{}]   bytes_per_frame={}",
        label,
        stream.get_bytes_per_frame()
    );
    log::info!(
        "[{}]   bytes_per_sample={}",
        label,
        stream.get_bytes_per_sample()
    );
    log::info!("[{}]   uses_aaudio={}", label, stream.uses_aaudio());
    log::info!("[{}]   channel_mask={:?}", label, stream.get_channel_mask());
    log::info!(
        "[{}]   hw_channel_count={}",
        label,
        stream.get_hardware_channel_count()
    );
    log::info!(
        "[{}]   hw_sample_rate={}",
        label,
        stream.get_hardware_sample_rate()
    );
    log::info!("[{}]   hw_format={:?}", label, stream.get_hardware_format());
    log::info!(
        "[{}]   allowed_capture={:?}",
        label,
        stream.get_allowed_capture_policy()
    );
    log::info!(
        "[{}]   privacy_mode={:?}",
        label,
        stream.get_privacy_sensitive_mode()
    );
    log::info!(
        "[{}]   is_spatialized={}",
        label,
        stream.is_content_spatialized()
    );
    log::info!(
        "[{}]   spatialization={:?}",
        label,
        stream.get_spatialization_behavior()
    );
    log::info!(
        "[{}]   delay_before_close={}ms",
        label,
        stream.get_delay_before_close_millis()
    );
    log::info!(
        "[{}]   perf_hint_enabled={}",
        label,
        stream.is_performance_hint_enabled()
    );
    log::info!(
        "[{}]   xrun_supported={}",
        label,
        stream.is_xrun_count_supported()
    );
}

fn play_and_verify<S: AudioStreamSafe + AudioStreamBase + AudioStream>(
    mut stream: S,
    label: &str,
    duration: Duration,
) -> Result<(), Error> {
    log_stream_info(&mut stream, label);
    stream.start()?;
    log::info!("[{}] playing {:.1}s", label, duration.as_secs_f64());
    std::thread::sleep(duration);
    let xrun = stream.get_xrun_count();
    log::info!("[{}] stopped. xrun_count={:?}", label, xrun);
    stream.stop()?;
    stream.close()?;
    log::info!("[{}] closed", label);
    Ok(())
}

fn run_test(label: &str, f: fn() -> Result<(), Error>) {
    log::info!("========== TEST: {} ==========", label);
    match f() {
        Ok(()) => log::info!("========== PASS: {} ==========", label),
        Err(e) => log::error!("========== FAIL: {} => {:?} ==========", label, e),
    }
}

struct SineF32Mono {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: f32,
    initialized: bool,
}
impl SineF32Mono {
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
impl AudioOutputCallback for SineF32Mono {
    type FrameType = (f32, Mono);
    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [f32],
    ) -> DataCallbackResult {
        if !self.initialized {
            self.delta = self.frequency * 2.0 * PI / stream.get_sample_rate() as f32;
            self.initialized = true;
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

struct SineF32Stereo {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: f32,
    initialized: bool,
}
impl SineF32Stereo {
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
impl AudioOutputCallback for SineF32Stereo {
    type FrameType = (f32, Stereo);
    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [(f32, f32)],
    ) -> DataCallbackResult {
        if !self.initialized {
            self.delta = self.frequency * 2.0 * PI / stream.get_sample_rate() as f32;
            self.initialized = true;
        }
        for (l, r) in frames {
            {
                let v = self.gain * self.phase.sin();
                *l = v;
                *r = v;
            }
            self.phase += self.delta;
            while self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }
        DataCallbackResult::Continue
    }
}

struct SineI16Mono {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: f32,
    initialized: bool,
}
impl SineI16Mono {
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
impl AudioOutputCallback for SineI16Mono {
    type FrameType = (i16, Mono);
    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [i16],
    ) -> DataCallbackResult {
        if !self.initialized {
            self.delta = self.frequency * 2.0 * PI / stream.get_sample_rate() as f32;
            self.initialized = true;
        }
        for frame in frames {
            *frame = (self.gain * self.phase.sin() * i16::MAX as f32) as i16;
            self.phase += self.delta;
            while self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }
        DataCallbackResult::Continue
    }
}

struct SineI16Stereo {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: f32,
    initialized: bool,
}
impl SineI16Stereo {
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
impl AudioOutputCallback for SineI16Stereo {
    type FrameType = (i16, Stereo);
    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [(i16, i16)],
    ) -> DataCallbackResult {
        if !self.initialized {
            self.delta = self.frequency * 2.0 * PI / stream.get_sample_rate() as f32;
            self.initialized = true;
        }
        for (l, r) in frames {
            {
                let v = (self.gain * self.phase.sin() * i16::MAX as f32) as i16;
                *l = v;
                *r = v;
            }
            self.phase += self.delta;
            while self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }
        DataCallbackResult::Continue
    }
}

struct SineI32Mono {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: f32,
    initialized: bool,
}
impl SineI32Mono {
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
impl AudioOutputCallback for SineI32Mono {
    type FrameType = (i32, Mono);
    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [i32],
    ) -> DataCallbackResult {
        if !self.initialized {
            self.delta = self.frequency * 2.0 * PI / stream.get_sample_rate() as f32;
            self.initialized = true;
        }
        for frame in frames {
            *frame = (self.gain * self.phase.sin() * i32::MAX as f32) as i32;
            self.phase += self.delta;
            while self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }
        DataCallbackResult::Continue
    }
}

struct VoiceRecorder {
    total_frames: Arc<AtomicI32>,
    callback_count: Arc<AtomicUsize>,
}
impl VoiceRecorder {
    fn new(total_frames: Arc<AtomicI32>, callback_count: Arc<AtomicUsize>) -> Self {
        Self {
            total_frames,
            callback_count,
        }
    }
}
impl AudioInputCallback for VoiceRecorder {
    type FrameType = (i16, Mono);
    fn on_audio_ready(
        &mut self,
        _s: &mut dyn AudioInputStreamSafe,
        d: &[i16],
    ) -> DataCallbackResult {
        self.total_frames
            .fetch_add(d.len() as i32, Ordering::Relaxed);
        self.callback_count.fetch_add(1, Ordering::Relaxed);
        DataCallbackResult::Continue
    }
    fn on_error_before_close(&mut self, _s: &mut dyn AudioInputStreamSafe, e: Error) {
        log::error!("[voice-rec] err before close: {:?}", e);
    }
    fn on_error_after_close(&mut self, _s: &mut dyn AudioInputStreamSafe, e: Error) {
        log::error!("[voice-rec] err after close: {:?}", e);
    }
}

fn test_f32_mono_48k() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<f32>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_callback(SineF32Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "F32-Mono-48k", Duration::from_secs(3))
}

fn test_f32_stereo_48k() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<f32>()
        .set_channel_count::<Stereo>()
        .set_sample_rate(48000)
        .set_callback(SineF32Stereo::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "F32-Stereo-48k", Duration::from_secs(3))
}

fn test_i16_mono_48k() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<i16>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_callback(SineI16Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "I16-Mono-48k", Duration::from_secs(3))
}

fn test_i16_stereo_48k() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<i16>()
        .set_channel_count::<Stereo>()
        .set_sample_rate(48000)
        .set_callback(SineI16Stereo::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "I16-Stereo-48k", Duration::from_secs(3))
}

fn test_i32_mono_48k() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<i32>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_callback(SineI32Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "I32-Mono-48k", Duration::from_secs(3))
}

fn test_f32_mono_16k() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<f32>()
        .set_channel_count::<Mono>()
        .set_sample_rate(16000)
        .set_callback(SineF32Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "F32-Mono-16k", Duration::from_secs(3))
}

fn test_i16_mono_16k() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<i16>()
        .set_channel_count::<Mono>()
        .set_sample_rate(16000)
        .set_callback(SineI16Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "I16-Mono-16k", Duration::from_secs(3))
}

fn test_f32_mono_44100() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<f32>()
        .set_channel_count::<Mono>()
        .set_sample_rate(44100)
        .set_callback(SineF32Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "F32-Mono-44100", Duration::from_secs(3))
}

fn test_voice_comm_output() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<i16>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_usage(Usage::VoiceCommunication)
        .set_content_type(ContentType::Speech)
        .set_privacy_sensitive_mode(PrivacySensitiveMode::Enabled)
        .set_spatialization_behavior(SpatializationBehavior::Never)
        .set_callback(SineI16Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "VoiceComm-Out", Duration::from_secs(3))
}

fn test_voice_comm_input() -> Result<(), Error> {
    let tf = Arc::new(AtomicI32::new(0));
    let cc = Arc::new(AtomicUsize::new(0));
    let mut s = AudioStreamBuilder::default()
        .set_input()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<i16>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_input_preset(InputPreset::VoiceCommunication)
        .set_callback(VoiceRecorder::new(Arc::clone(&tf), Arc::clone(&cc)))
        .open_stream()?;
    log_stream_info(&mut s, "VoiceComm-In");
    s.start()?;
    log::info!("[VoiceComm-In] recording 3s");
    std::thread::sleep(Duration::from_secs(3));
    let xrun = s.get_xrun_count();
    let frames = tf.load(Ordering::Relaxed);
    let callbacks = cc.load(Ordering::Relaxed);
    log::info!(
        "[VoiceComm-In] stopped. xrun={:?} frames={} callbacks={}",
        xrun,
        frames,
        callbacks
    );
    s.stop()?;
    s.close()?;
    log::info!("[VoiceComm-In] closed");
    Ok(())
}

fn test_voice_comm_f32_output() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<f32>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_usage(Usage::VoiceCommunication)
        .set_content_type(ContentType::Speech)
        .set_spatialization_behavior(SpatializationBehavior::Never)
        .set_callback(SineF32Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "VoiceComm-F32-Out", Duration::from_secs(3))
}

fn test_exclusive_low_latency() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Exclusive)
        .set_format::<f32>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_callback(SineF32Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "Exclusive-LL", Duration::from_secs(3))
}

fn test_power_saving() -> Result<(), Error> {
    let s = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::PowerSaving)
        .set_sharing_mode(SharingMode::Shared)
        .set_format::<f32>()
        .set_channel_count::<Mono>()
        .set_sample_rate(48000)
        .set_callback(SineF32Mono::new(440.0, 0.5))
        .open_stream()?;
    play_and_verify(s, "PowerSaving", Duration::from_secs(3))
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
            .with_tag("oboe-format-test"),
    );
    log::info!("oboe-format-test starting");

    // Format tests
    run_test("F32-Mono-48k", test_f32_mono_48k);
    run_test("F32-Stereo-48k", test_f32_stereo_48k);
    run_test("I16-Mono-48k", test_i16_mono_48k);
    run_test("I16-Stereo-48k", test_i16_stereo_48k);
    run_test("I32-Mono-48k", test_i32_mono_48k);
    run_test("F32-Mono-16k", test_f32_mono_16k);
    run_test("I16-Mono-16k", test_i16_mono_16k);
    run_test("F32-Mono-44100", test_f32_mono_44100);

    // Voice chat tests
    run_test("VoiceComm-Output", test_voice_comm_output);
    run_test("VoiceComm-Input", test_voice_comm_input);
    run_test("VoiceComm-F32-Output", test_voice_comm_f32_output);
    run_test("Exclusive-LowLatency", test_exclusive_low_latency);
    run_test("PowerSaving", test_power_saving);

    log::info!("oboe-format-test finished");
}

#[cfg(not(target_os = "android"))]
fn main() {
    eprintln!("This example only runs on Android.");
}
