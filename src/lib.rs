#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "doc-cfg", feature(doc_cfg))]

mod audio_stream;
mod audio_stream_base;
mod audio_stream_builder;
mod audio_stream_callback;
mod definitions;
mod type_guide;

#[cfg(feature = "java-interface")]
mod java_interface;

pub use self::audio_stream::*;
pub use self::audio_stream_base::*;
pub use self::audio_stream_builder::*;
pub use self::audio_stream_callback::*;
pub use self::definitions::*;
pub use self::type_guide::*;

#[cfg(feature = "java-interface")]
pub use self::java_interface::*;

use oboe_sys as ffi;

pub(crate) trait RawAudioStreamBase {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase;
    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase;
}

pub(crate) trait RawAudioStream {
    fn _raw_stream(&self) -> &ffi::oboe_AudioStream;
    fn _raw_stream_mut(&mut self) -> &mut ffi::oboe_AudioStream;
}

pub(crate) trait RawAudioInputStream {}

pub(crate) trait RawAudioOutputStream {}
