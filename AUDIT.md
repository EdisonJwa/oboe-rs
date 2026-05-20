# oboe-rs Audit — 2026-05-20

## Project Structure

```
oboe-rs/                    # Workspace root (safe Rust wrapper crate "oboe")
├── Cargo.toml              # Workspace: members=[sys, demo], edition 2021
├── src/                    # Safe wrapper (~1300 LOC)
│   ├── lib.rs              # Re-exports
│   ├── definitions.rs      # Enums, Error, StreamState, SharingMode, PerformanceMode, etc.
│   ├── audio_stream_builder.rs  # Type-state builder
│   ├── audio_stream.rs     # Stream traits (AudioStream, AudioStreamSafe, sync/async impls)
│   ├── audio_stream_base.rs     # Stream property getters
│   ├── audio_stream_callback.rs # Callback bridge (Rust traits → C function pointers)
│   ├── type_guide.rs       # Phantom markers (Mono/Stereo, f32/i16, Input/Output)
│   ├── private.rs          # Raw FFI marker traits
│   ├── version.rs          # Oboe version from FFI constants
│   └── java_interface/     # Optional: JNI device info, audio features
├── sys/                    # oboe-sys crate (unsafe FFI + C++ build)
│   ├── Cargo.toml          # Features: shared-link, generate-bindings, fetch-prebuilt, test
│   ├── build.rs            # Compiles Oboe C++ + oboe-ext via `cc` crate
│   ├── src/
│   │   ├── lib.rs          # Conditional include of arch-specific bindings
│   │   ├── bindings_aarch64.rs   # Pregenerated bindgen (~1696 LOC)
│   │   ├── bindings_armv7.rs
│   │   ├── bindings_i686.rs
│   │   └── bindings_x86_64.rs
│   ├── oboe/               # Git submodule → google/oboe (currently 1.8.1)
│   └── oboe-ext/           # C++ shim layer
│       ├── include/oboe/OboeExt.h       # C-linkable wrapper functions
│       └── src/
│           ├── AudioStreamWrapper.cpp
│           ├── AudioStreamBuilderWrapper.cpp
│           └── AudioStreamCallbackWrapper.cpp
└── demo/                   # Android egui demo (not minimal)
    ├── Cargo.toml          # Uses egui/eframe, cargo-mobile2
    └── src/
        ├── lib.rs          # Android activity entry + egui UI
        └── audio.rs        # Sine generator + device info printer
```

## Current Oboe Version: 1.8.1

Submodule `sys/oboe` is pinned at commit `86165b8249bc22b9ef70b69e20323244b6f08d88` (tag `1.8.1`).
Latest Google Oboe stable release: **1.10.0**.

## Build System

- **Default path**: Compiles from source using `cc` crate (C++17, no RTTI/exceptions).
- Source file list in `build.rs` is **hardcoded** — must be updated when Oboe adds/renames .cpp files.
- `fetch-prebuilt` feature downloads prebuilt `.a` from original author's GitHub releases — **stale, should be removed**.
- `generate-bindings` feature runs bindgen against `OboeExt.h`.
- Pregenerated bindings in `sys/src/bindings_*.rs` are used by default.

## Build Status

- `cargo ndk --platform 21 --target aarch64-linux-android -- build --release` → **PASSES** for `oboe` and `oboe-sys`.
- `cc` crate deprecation warnings: `static_flag()`/`shared_flag()` are deprecated.

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

## Issues Found

1. **Oboe submodule outdated** (1.8.1 → 1.10.0)
2. **No minimal sine example** — demo requires egui/eframe/cargo-mobile2
3. **`fetch-prebuilt` feature** depends on stale prebuilt binaries
4. **`cc` deprecated warnings** — `static_flag`/`shared_flag`
5. **CI uses NDK 25.2** — should update to NDK 27+
6. **README mentions `compile-library` feature** which doesn't exist in Cargo.toml
7. **Source file list** in `build.rs` may need new .cpp files from Oboe 1.9.x/1.10.x
8. **`AudioFormat::I24`** in bindings but no `IsFormat for` 24-bit type in safe wrapper

## Oboe Version Changes (1.8.1 → 1.10.0)

### 1.9.0
- IEC61937 compressed audio format
- Improved resampler
- USB device attach/detach events

### 1.9.3
- FullDuplexStream moved to std::shared_ptr (ownership change)
- AudioClock utility
- Static OpenSL ES linking support
- 16KB page size support

### 1.10.0
- MMAP policy query (AAudio)
- PCM offload, compressed format support
- flushFromFrame, partial data callback
- OBOE_DISABLE_CONVERSION compile flag
- getDeviceIds (replaces getDeviceId)

## Completed Actions

1. ✅ Updated Oboe submodule from 1.8.1 to 1.10.0
2. ✅ Added 6 missing source files to build.rs (OboeExtensions, Limiter, MonoBlend, MultiToManyConverter, SinkI8_24, SourceI8_24)
3. ✅ Removed deprecated `static_flag`/`shared_flag` calls from cc::Build
4. ✅ Removed `fetch-prebuilt` feature and `fetch_unroll` dependency (always build from source)
5. ✅ Regenerated bindings for all 4 Android architectures
6. ✅ Removed `get_device_id`/`set_device_id` (Oboe 1.10.0 replaced mDeviceId with mDeviceIds)
7. ✅ Added minimal sine output example (examples/sine/)
8. ✅ Clean build verified for aarch64-linux-android (release + debug)
9. ✅ All 4 Android targets build successfully
10. ✅ Updated README with current build instructions and API

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
