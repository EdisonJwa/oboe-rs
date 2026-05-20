# Rust bindings for Oboe library

[![github](https://img.shields.io/badge/github-katyo/oboe--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/oboe-rs)
[![Crates.io Package](https://img.shields.io/crates/v/oboe.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/oboe)
[![Docs.rs API Docs](https://img.shields.io/badge/docs.rs-oboe-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/oboe)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/Apache-2.0)
[![CI Status](https://img.shields.io/github/actions/workflow/status/katyo/oboe-rs/rust.yml?branch=master&style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/oboe-rs/actions?query=workflow%3ARust)

Safe Rust interface for [Oboe](https://github.com/google/oboe) High-Performance Audio library for Android.
Also it provides interface for some platform APIs significant to Audio IO.

__Oboe__ is a C++ library which makes it easy to build high-performance audio apps on Android. It was created primarily to allow developers to target a simplified API that works across multiple API levels back to API level 16 (Jelly Bean).

## Crate features

- __java-interface__ Add interface for some Android platform APIs.
- __generate-bindings__ Generate bindings at compile-time. By default the pregenerated bindings will be used.
- __shared-stdcxx__ Link against shared C++ standard library instead of static.
- __shared-link__ Use shared linking. By default the static Oboe library will be used.

The crate builds from source using the vendored Oboe C++ library (currently **1.10.0**).

Pregenerated bindings are available for:

- __armv7__
- __aarch64__
- __i686__
- __x86_64__

## Build prerequisites

- Rust stable (1.95+)
- `cargo-ndk` for cross-compilation
- Android NDK 27+ (tested with NDK 28)
- Android SDK with platform 34+
- `ANDROID_NDK_HOME` or `NDK_HOME` environment variable set

## Building

```bash
# Install cargo-ndk
cargo install cargo-ndk

# Add Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Build for arm64
cargo ndk --platform 21 --target aarch64-linux-android -- build --release

# Build the sine example
cargo ndk --platform 21 --target aarch64-linux-android -- build --release -p oboe-sine-example
```

## Regenerating bindings

When updating the vendored Oboe submodule or changing the C++ shim:

```bash
# Regenerate bindings for a specific architecture
cargo ndk --platform 21 --target aarch64-linux-android -- build --release --features generate-bindings -p oboe-sys

# Copy from build output
cp target/aarch64-linux-android/release/build/oboe-sys-*/out/bindings.rs sys/src/bindings_aarch64.rs
```

## Usage example

Playing sine wave in asynchronous (callback-driven) mode:

```rust
use oboe::{
    AudioOutputCallback,
    AudioOutputStreamSafe,
    AudioStreamBuilder,
    DataCallbackResult,
    PerformanceMode,
    SharingMode,
    Mono,
};

pub struct SineWave {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: Option<f32>,
}

impl Default for SineWave {
    fn default() -> Self {
        Self {
            frequency: 440.0,
            gain: 0.5,
            phase: 0.0,
            delta: None,
        }
    }
}

impl AudioOutputCallback for SineWave {
    type FrameType = (f32, Mono);

    fn on_audio_ready(&mut self, stream: &mut dyn AudioOutputStreamSafe, frames: &mut [f32]) -> DataCallbackResult {
        if self.delta.is_none() {
            let sample_rate = stream.get_sample_rate() as f32;
            self.delta = Some(self.frequency * 2.0 * std::f32::consts::PI / sample_rate);
        }

        let delta = self.delta.unwrap();

        for frame in frames {
            *frame = self.gain * self.phase.sin();
            self.phase += delta;
            while self.phase > 2.0 * std::f32::consts::PI {
                self.phase -= 2.0 * std::f32::consts::PI;
            }
        }

        DataCallbackResult::Continue
    }
}

let mut sine = AudioStreamBuilder::default()
    .set_performance_mode(PerformanceMode::LowLatency)
    .set_sharing_mode(SharingMode::Shared)
    .set_format::<f32>()
    .set_channel_count::<Mono>()
    .set_callback(SineWave::default())
    .open_stream()
    .unwrap();

sine.start().unwrap();
```
