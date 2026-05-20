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

impl<T: RawAudioStreamBase> AudioStreamBase for T {
    fn get_channel_count(&self) -> ChannelCount {
        FromPrimitive::from_i32(self._raw_base().mChannelCount).unwrap_or_default()
    }

    fn get_channel_mask(&self) -> ChannelMask {
        FromPrimitive::from_u32(self._raw_base().mChannelMask).unwrap_or_default()
    }

    fn get_direction(&self) -> Direction {
        FromPrimitive::from_i32(self._raw_base().mDirection).unwrap_or_default()
    }

    fn get_sample_rate(&self) -> i32 {
        self._raw_base().mSampleRate
    }

    fn get_frames_per_callback(&self) -> i32 {
        self._raw_base().mFramesPerCallback
    }

    fn get_format(&self) -> AudioFormat {
        FromPrimitive::from_i32(self._raw_base().mFormat).unwrap_or_default()
    }

    fn get_hardware_format(&self) -> AudioFormat {
        FromPrimitive::from_i32(self._raw_base().mHardwareFormat).unwrap_or_default()
    }

    fn get_buffer_size_in_frames(&self) -> i32 {
        self._raw_base().mBufferSizeInFrames
    }

    fn get_buffer_capacity_in_frames(&self) -> i32 {
        self._raw_base().mBufferCapacityInFrames
    }

    fn get_sharing_mode(&self) -> SharingMode {
        FromPrimitive::from_i32(self._raw_base().mSharingMode).unwrap_or_default()
    }

    fn get_performance_mode(&self) -> PerformanceMode {
        FromPrimitive::from_i32(self._raw_base().mPerformanceMode).unwrap_or_default()
    }

    fn get_usage(&self) -> Usage {
        FromPrimitive::from_i32(self._raw_base().mUsage).unwrap_or_default()
    }

    fn get_allowed_capture_policy(&self) -> AllowedCapturePolicy {
        FromPrimitive::from_i32(self._raw_base().mAllowedCapturePolicy).unwrap_or_default()
    }

    fn get_privacy_sensitive_mode(&self) -> PrivacySensitiveMode {
        FromPrimitive::from_i32(self._raw_base().mPrivacySensitiveMode).unwrap_or_default()
    }

    fn is_content_spatialized(&self) -> bool {
        self._raw_base().mIsContentSpatialized
    }

    fn get_spatialization_behavior(&self) -> SpatializationBehavior {
        FromPrimitive::from_i32(self._raw_base().mSpatializationBehavior).unwrap_or_default()
    }

    fn get_hardware_channel_count(&self) -> i32 {
        self._raw_base().mHardwareChannelCount
    }

    fn get_hardware_sample_rate(&self) -> i32 {
        self._raw_base().mHardwareSampleRate
    }

    fn get_content_type(&self) -> ContentType {
        FromPrimitive::from_i32(self._raw_base().mContentType).unwrap_or_default()
    }

    fn get_input_preset(&self) -> InputPreset {
        FromPrimitive::from_i32(self._raw_base().mInputPreset).unwrap_or_default()
    }

    fn get_session_id(&self) -> SessionId {
        FromPrimitive::from_i32(self._raw_base().mSessionId).unwrap_or_default()
    }

    fn is_channel_conversion_allowed(&self) -> bool {
        self._raw_base().mChannelConversionAllowed
    }

    fn is_format_conversion_allowed(&self) -> bool {
        self._raw_base().mFormatConversionAllowed
    }

    fn get_sample_rate_conversion_quality(&self) -> SampleRateConversionQuality {
        FromPrimitive::from_i32(self._raw_base().mSampleRateConversionQuality).unwrap_or_default()
    }
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
