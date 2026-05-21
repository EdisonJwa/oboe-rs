//use oboe_sys as ffi;
use num_traits::FromPrimitive;

use std::fmt::{self, Display};

use super::{
    AllowedCapturePolicy, AudioFormat, ChannelCount, ChannelMask, ContentType, Direction,
    InputPreset, PerformanceMode, PrivacySensitiveMode, RawAudioStreamBase,
    SampleRateConversionQuality, SessionId, SharingMode, SpatializationBehavior, Usage,
};

/**
 * Base trait containing parameters for audio streams and builders.
 */
pub trait AudioStreamBase {
    /**
     * Get actual number of channels
     */
    fn get_channel_count(&self) -> ChannelCount;

    fn get_channel_mask(&self) -> ChannelMask;

    /**
     * Get actual stream direction
     *
     * `Direction::Input` or `Direction::Output`.
     */
    fn get_direction(&self) -> Direction;

    /**
     * Get the actual sample rate for the stream
     */
    fn get_sample_rate(&self) -> i32;

    /**
     * Get the number of frames in each callback
     */
    fn get_frames_per_callback(&self) -> i32;

    /**
     * Get the audio sample format (e.g. F32 or I16)
     */
    fn get_format(&self) -> AudioFormat;

    fn get_hardware_format(&self) -> AudioFormat;

    /**
     * Query the maximum number of frames that can be filled without blocking.
     * If the stream has been closed the last known value will be returned.
     */
    fn get_buffer_size_in_frames(&self) -> i32;

    /**
     * Get the capacity in number of frames
     */
    fn get_buffer_capacity_in_frames(&self) -> i32;

    /**
     * Get the sharing mode of the stream
     */
    fn get_sharing_mode(&self) -> SharingMode;

    fn get_performance_mode(&self) -> PerformanceMode;

    fn get_usage(&self) -> Usage;

    fn get_allowed_capture_policy(&self) -> AllowedCapturePolicy;

    fn get_privacy_sensitive_mode(&self) -> PrivacySensitiveMode;
    fn is_content_spatialized(&self) -> bool;
    fn get_spatialization_behavior(&self) -> SpatializationBehavior;
    fn get_hardware_channel_count(&self) -> i32;
    fn get_hardware_sample_rate(&self) -> i32;

    /**
     * Get the stream's content type
     */
    fn get_content_type(&self) -> ContentType;

    /**
     * Get the stream's input preset
     */
    fn get_input_preset(&self) -> InputPreset;

    /**
     * Get the stream's session ID allocation strategy (None or Allocate)
     */
    fn get_session_id(&self) -> SessionId;

    /**
     * Get the raw system audio session ID.
     *
     * After the stream is opened with `SessionId::Allocate`, AAudio
     * assigns a positive system session ID. Returns that raw value
     * (`Some(id)` where id > 0), or `None` if no session was allocated.
     * `get_session_id()` only returns builder parameters because the
     * `SessionId` enum has no `Allocated(i32)` variant.
     */
    fn get_raw_session_id(&self) -> Option<i32>;

    /**
     * Return true if can convert channel counts to achieve optimal results.
     */
    fn is_channel_conversion_allowed(&self) -> bool;

    /**
     * Return true if  Oboe can convert data formats to achieve optimal results.
     */
    fn is_format_conversion_allowed(&self) -> bool;

    /**
     * Get whether and how Oboe can convert sample rates to achieve optimal results.
     */
    fn get_sample_rate_conversion_quality(&self) -> SampleRateConversionQuality;
}

macro_rules! raw_enum_getter {
    ($method:ident, $field:ident, $enum_type:ty) => {
        fn $method(&self) -> $enum_type {
            FromPrimitive::from_i32(self._raw_base().$field).unwrap_or_default()
        }
    };
}

macro_rules! raw_u32_enum_getter {
    ($method:ident, $field:ident, $enum_type:ty) => {
        fn $method(&self) -> $enum_type {
            FromPrimitive::from_u32(self._raw_base().$field).unwrap_or_default()
        }
    };
}

macro_rules! raw_field_getter {
    ($method:ident, $field:ident, $ret_type:ty) => {
        fn $method(&self) -> $ret_type {
            self._raw_base().$field
        }
    };
}

impl<T: RawAudioStreamBase> AudioStreamBase for T {
    raw_enum_getter!(get_channel_count, mChannelCount, ChannelCount);
    raw_u32_enum_getter!(get_channel_mask, mChannelMask, ChannelMask);
    raw_enum_getter!(get_direction, mDirection, Direction);
    raw_field_getter!(get_sample_rate, mSampleRate, i32);
    raw_field_getter!(get_frames_per_callback, mFramesPerCallback, i32);
    raw_enum_getter!(get_format, mFormat, AudioFormat);
    raw_enum_getter!(get_hardware_format, mHardwareFormat, AudioFormat);
    raw_field_getter!(get_buffer_size_in_frames, mBufferSizeInFrames, i32);
    raw_field_getter!(get_buffer_capacity_in_frames, mBufferCapacityInFrames, i32);
    raw_enum_getter!(get_sharing_mode, mSharingMode, SharingMode);
    raw_enum_getter!(get_performance_mode, mPerformanceMode, PerformanceMode);
    raw_enum_getter!(get_usage, mUsage, Usage);
    raw_enum_getter!(
        get_allowed_capture_policy,
        mAllowedCapturePolicy,
        AllowedCapturePolicy
    );
    raw_enum_getter!(
        get_privacy_sensitive_mode,
        mPrivacySensitiveMode,
        PrivacySensitiveMode
    );
    raw_field_getter!(is_content_spatialized, mIsContentSpatialized, bool);
    raw_enum_getter!(
        get_spatialization_behavior,
        mSpatializationBehavior,
        SpatializationBehavior
    );
    raw_field_getter!(get_hardware_channel_count, mHardwareChannelCount, i32);
    raw_field_getter!(get_hardware_sample_rate, mHardwareSampleRate, i32);
    raw_enum_getter!(get_content_type, mContentType, ContentType);
    raw_enum_getter!(get_input_preset, mInputPreset, InputPreset);
    raw_enum_getter!(get_session_id, mSessionId, SessionId);

    fn get_raw_session_id(&self) -> Option<i32> {
        let raw = self._raw_base().mSessionId;
        if raw > 0 { Some(raw) } else { None }
    }

    raw_field_getter!(
        is_channel_conversion_allowed,
        mChannelConversionAllowed,
        bool
    );
    raw_field_getter!(is_format_conversion_allowed, mFormatConversionAllowed, bool);
    raw_enum_getter!(
        get_sample_rate_conversion_quality,
        mSampleRateConversionQuality,
        SampleRateConversionQuality
    );
}

pub(crate) fn audio_stream_base_fmt<T: AudioStreamBase>(
    base: &T,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    "Direction: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_direction(), f)?;
    if base.get_direction() == Direction::Input {
        "\nInput preset: ".fmt(f)?;
        fmt::Debug::fmt(&base.get_input_preset(), f)?;
    }
    "\nBuffer capacity in frames: ".fmt(f)?;
    base.get_buffer_capacity_in_frames().fmt(f)?;
    "\nBuffer size in frames: ".fmt(f)?;
    base.get_buffer_size_in_frames().fmt(f)?;
    "\nFrames per callback: ".fmt(f)?;
    base.get_frames_per_callback().fmt(f)?;
    "\nSample rate: ".fmt(f)?;
    base.get_sample_rate().fmt(f)?;
    "\nSample rate conversion quality: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_sample_rate_conversion_quality(), f)?;
    "\nChannel count: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_channel_count(), f)?;
    if base.is_channel_conversion_allowed() {
        " (conversion allowed)".fmt(f)?;
    }
    "\nFormat: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_format(), f)?;
    if base.is_format_conversion_allowed() {
        " (conversion allowed)".fmt(f)?;
    }
    "\nSharing mode: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_sharing_mode(), f)?;
    "\nPerformance mode: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_performance_mode(), f)?;
    "\nUsage: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_usage(), f)?;
    "\nContent type: ".fmt(f)?;
    fmt::Debug::fmt(&base.get_content_type(), f)?;
    '\n'.fmt(f)
}
