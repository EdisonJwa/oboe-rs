# oboe-rs Audit — 2026-05-20 (updated after upgrade to Oboe 1.10.0)

## Project Structure

```
oboe-rs/                    # Workspace root (safe Rust wrapper crate "oboe")
├── Cargo.toml              # Workspace: members=[sys, demo, examples/sine], edition 2021
├── src/                    # Safe wrapper (~1300 LOC)
│   ├── lib.rs              # Re-exports
│   ├── definitions.rs      # Enums, Error, StreamState, SharingMode, PerformanceMode, etc.
│   ├── audio_stream_builder.rs  # Type-state builder
│   ├── audio_stream.rs     # Stream traits (AudioStream, AudioStreamSafe, sync/async impls)
│   ├── audio_stream_base.rs     # Stream property getters
│   ├── audio_stream_callback.rs # Callback bridge (Rust traits → C function pointers)
│   ├── type_guide.rs       # Phantom markers (Mono/Stereo, f32/i16/i32/I24, Input/Output)
│   ├── private.rs          # Raw FFI marker traits
│   ├── version.rs          # Oboe version from FFI constants
│   └── java_interface/     # Optional: JNI device info, audio features
├── sys/                    # oboe-sys crate (unsafe FFI + C++ build)
│   ├── Cargo.toml          # Features: shared-link, generate-bindings, shared-stdcxx
│   ├── build.rs            # Compiles Oboe C++ + oboe-ext via `cc` crate
│   ├── src/
│   │   ├── lib.rs          # Conditional include of arch-specific bindings
│   │   ├── bindings_aarch64.rs   # Pregenerated bindgen (~1851 LOC)
│   │   ├── bindings_armv7.rs
│   │   ├── bindings_i686.rs
│   │   └── bindings_x86_64.rs
│   ├── oboe/               # Git submodule → google/oboe (currently 1.10.0)
│   └── oboe-ext/           # C++ shim layer
│       ├── CMakeLists.txt  # Source list (updated for 1.10.0)
│       ├── include/oboe/OboeExt.h       # C-linkable wrapper functions
│       └── src/
│           ├── AudioStreamWrapper.cpp
│           ├── AudioStreamBuilderWrapper.cpp
│           └── AudioStreamCallbackWrapper.cpp
├── examples/sine/          # Minimal sine output example (no egui)
│   ├── Cargo.toml
│   └── src/lib.rs          # 440Hz sine, 5s playback, logs stream params + xRun count
└── demo/                   # Android egui demo (not minimal)
    ├── Cargo.toml          # Uses egui/eframe, cargo-mobile2
    └── src/
        ├── lib.rs          # Android activity entry + egui UI
        └── audio.rs        # Sine generator + device info printer
```

## Current Oboe Version: 1.10.0

Submodule `sys/oboe` is pinned at commit `a81bb9f8` (tag `1.10.0`).

## Build System

- **Default path**: Compiles from source using `cc` crate (C++17, no RTTI/exceptions).
- Source file list in `build.rs` is hardcoded — must be updated when Oboe adds/renames .cpp files.
- `fetch-prebuilt` feature removed — always builds from source.
- `generate-bindings` feature runs bindgen against `OboeExt.h`.
- Pregenerated bindings in `sys/src/bindings_*.rs` are used by default.
- Makefile bindgen rule uses `cargo ndk --platform` (fixed from `--android-platform`) and `sed -E` (macOS-compatible).

## Build Status

- All 4 Android targets (aarch64, armv7, x86_64, i686) build successfully in release mode.
- Sine example builds successfully.
- LSP diagnostics: zero errors.

## FFI Architecture

Oboe is **C++ only** — no upstream C API. The `oboe-ext` C++ shim wraps C++ classes into
`extern` C-linkable functions. This is the correct approach. Google's own samples use the same
pattern for JNI (`extern "C"` wrappers around Oboe engine classes).

The shim provides:
- `AudioStreamCallbackWrapper` — bridges C++ virtual callbacks to C function pointers
- `AudioStreamBuilder_*` — placement new/delete, config via `AudioStreamBase` fields
- `AudioStream*` — open/close/start/stop/pause/flush + query methods
- `AudioStreamShared` — manages `std::shared_ptr<AudioStream>` lifetime

## API Surface (Already Exposed)

| Desired API | Current Status |
|-------------|---------------|
| Stream open/start/stop/close | ✅ AudioStream trait |
| Callback output | ✅ AudioOutputCallback trait |
| LowLatency/Exclusive/Shared | ✅ PerformanceMode/SharingMode enums |
| Buffer size/capacity | ✅ get/set buffer_size_in_frames, get_buffer_capacity_in_frames |
| Frames per burst | ✅ get_frames_per_burst |
| Actual stream params | ✅ AudioStreamBase trait |
| xRun count | ✅ get_xrun_count / is_xrun_count_supported |
| I24 format | ✅ I24 struct + IsFormat impl + set_i24() builder method |

## Completed Actions

1. ✅ Updated Oboe submodule from 1.8.1 to 1.10.0
2. ✅ Added missing source files to build.rs (OboeExtensions, Limiter, MonoBlend, MultiToManyConverter, SinkI8_24, SourceI8_24)
3. ✅ Removed deprecated `static_flag`/`shared_flag` calls from cc::Build
4. ✅ Removed `fetch-prebuilt` feature and `fetch_unroll` dependency (always build from source)
5. ✅ Regenerated bindings for all 4 Android architectures
6. ✅ Removed `get_device_id`/`set_device_id` (Oboe 1.10.0 replaced mDeviceId with mDeviceIds)
7. ✅ Added minimal sine output example (examples/sine/)
8. ✅ Clean build verified for all 4 Android targets (release mode)
9. ✅ Added I24 format type (struct I24 + IsFormat impl + set_i24() builder)
10. ✅ Updated README with current build instructions and API
11. ✅ Fixed Makefile bindgen rule (--android-platform → --platform, sed -r → sed -E)
12. ✅ Updated oboe-ext CMakeLists.txt with 10 missing Oboe 1.10.0 source files
13. ✅ All commits pushed to origin/master

## Runtime Verification Checklist

When running on a real Android device, verify:

- [ ] Stream opens without error
- [ ] Sample rate matches device (typically 48000 Hz)
- [ ] Channel count is Mono (1)
- [ ] Frames per burst is reported correctly (varies by device)
- [ ] Buffer capacity and size are reasonable
- [ ] Performance mode is LowLatency
- [ ] Sharing mode is Shared
- [ ] XRun count is 0 or very low after 5s of playback
- [ ] Increasing XRun count is treated as a bug (tune buffer size first)
- [ ] Stream stops and closes cleanly

Record these values from the sine example log output:

```
Sine callback initialized: rate=<SAMPLE_RATE>, delta=<DELTA>
Stream opened successfully
  Sample rate:      <VALUE>
  Channel count:    <VALUE>
  Format:           <VALUE>
  Frames per burst: <VALUE>
  Buffer capacity:  <VALUE>
  Buffer size:      <VALUE>
  Performance mode: <VALUE>
  Sharing mode:     <VALUE>
  XRun supported:   <VALUE>
Stream started — playing 440 Hz sine for 5 seconds
  XRun count:       <VALUE>
Stream stopped and closed
```
