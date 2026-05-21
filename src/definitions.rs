use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use oboe_sys as ffi;
use std::{error, fmt, result};

/**
 * The number of nanoseconds in a microsecond. 1,000.
 */
pub const NANOS_PER_MICROSECOND: i64 = 1000;

/**
 * The number of nanoseconds in a millisecond. 1,000,000.
 */
pub const NANOS_PER_MILLISECOND: i64 = NANOS_PER_MICROSECOND * 1000;

/**
 * The number of milliseconds in a second. 1,000.
 */
pub const MILLIS_PER_SECOND: i64 = 1000;

/**
 * The number of nanoseconds in a second. 1,000,000,000.
 */
pub const NANOS_PER_SECOND: i64 = NANOS_PER_MILLISECOND * MILLIS_PER_SECOND;

/**
 * The state of the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum StreamState {
    #[default]
    Uninitialized = ffi::oboe_StreamState_Uninitialized,
    Unknown = ffi::oboe_StreamState_Unknown,
    Open = ffi::oboe_StreamState_Open,
    Starting = ffi::oboe_StreamState_Starting,
    Started = ffi::oboe_StreamState_Started,
    Pausing = ffi::oboe_StreamState_Pausing,
    Paused = ffi::oboe_StreamState_Paused,
    Flushing = ffi::oboe_StreamState_Flushing,
    Flushed = ffi::oboe_StreamState_Flushed,
    Stopping = ffi::oboe_StreamState_Stopping,
    Stopped = ffi::oboe_StreamState_Stopped,
    Closing = ffi::oboe_StreamState_Closing,
    Closed = ffi::oboe_StreamState_Closed,
    Disconnected = ffi::oboe_StreamState_Disconnected,
}

/**
 * The direction of the stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum Direction {
    #[default]
    Output = ffi::oboe_Direction_Output,

    /**
     * Used for recording.
     */
    Input = ffi::oboe_Direction_Input,
}

/**
 * The format of audio samples.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum AudioFormat {
    Invalid = ffi::oboe_AudioFormat_Invalid,

    #[default]
    Unspecified = ffi::oboe_AudioFormat_Unspecified,

    I16 = ffi::oboe_AudioFormat_I16,

    F32 = ffi::oboe_AudioFormat_Float,

    I24 = ffi::oboe_AudioFormat_I24,

    I32 = ffi::oboe_AudioFormat_I32,

    IEC61937 = ffi::oboe_AudioFormat_IEC61937,

    MP3 = ffi::oboe_AudioFormat_MP3,

    AAC_LC = ffi::oboe_AudioFormat_AAC_LC,

    AAC_HE_V1 = ffi::oboe_AudioFormat_AAC_HE_V1,

    AAC_HE_V2 = ffi::oboe_AudioFormat_AAC_HE_V2,

    AAC_ELD = ffi::oboe_AudioFormat_AAC_ELD,

    AAC_XHE = ffi::oboe_AudioFormat_AAC_XHE,

    OPUS = ffi::oboe_AudioFormat_OPUS,
}

/**
 * The result of an audio callback.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum DataCallbackResult {
    /**
     * Indicates to the caller that the callbacks should continue.
     */
    Continue = ffi::oboe_DataCallbackResult_Continue,

    /**
     * Indicates to the caller that the callbacks should stop immediately.
     */
    Stop = ffi::oboe_DataCallbackResult_Stop,
}

/**
 * The result of an operation with value
 */
pub type Result<T> = result::Result<T, Error>;

/**
 * The result of operation without value
 */
pub type Status = Result<()>;

pub(crate) fn wrap_status(result: i32) -> Status {
    if result == ffi::oboe_Result_OK {
        Ok(())
    } else {
        Err(FromPrimitive::from_i32(result).unwrap_or(Error::Internal))
    }
}

pub(crate) fn wrap_result<T>(result: ffi::oboe_ResultWithValue<T>) -> Result<T> {
    if result.mError == ffi::oboe_Result_OK {
        Ok(result.mValue)
    } else {
        Err(FromPrimitive::from_i32(result.mError).unwrap_or(Error::Internal))
    }
}

/**
 * The error of an operation.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum Error {
    Disconnected = ffi::oboe_Result_ErrorDisconnected,
    IllegalArgument = ffi::oboe_Result_ErrorIllegalArgument,
    Internal = ffi::oboe_Result_ErrorInternal,
    InvalidState = ffi::oboe_Result_ErrorInvalidState,
    InvalidHandle = ffi::oboe_Result_ErrorInvalidHandle,
    Unimplemented = ffi::oboe_Result_ErrorUnimplemented,
    Unavailable = ffi::oboe_Result_ErrorUnavailable,
    NoFreeHandles = ffi::oboe_Result_ErrorNoFreeHandles,
    NoMemory = ffi::oboe_Result_ErrorNoMemory,
    Null = ffi::oboe_Result_ErrorNull,
    Timeout = ffi::oboe_Result_ErrorTimeout,
    WouldBlock = ffi::oboe_Result_ErrorWouldBlock,
    InvalidFormat = ffi::oboe_Result_ErrorInvalidFormat,
    OutOfRange = ffi::oboe_Result_ErrorOutOfRange,
    NoService = ffi::oboe_Result_ErrorNoService,
    InvalidRate = ffi::oboe_Result_ErrorInvalidRate,
    Closed = ffi::oboe_Result_ErrorClosed,
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/**
 * The sharing mode of the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum SharingMode {
    Exclusive = ffi::oboe_SharingMode_Exclusive,

    #[default]
    Shared = ffi::oboe_SharingMode_Shared,
}

/**
 * The performance mode of the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum PerformanceMode {
    #[default]
    None = ffi::oboe_PerformanceMode_None,

    PowerSaving = ffi::oboe_PerformanceMode_PowerSaving,

    LowLatency = ffi::oboe_PerformanceMode_LowLatency,

    PowerSavingOffloaded = ffi::oboe_PerformanceMode_PowerSavingOffloaded,
}

/**
 * The underlying audio API used by the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum AudioApi {
    #[default]
    Unspecified = ffi::oboe_AudioApi_Unspecified,

    /**
     * Use OpenSL ES.
     */
    OpenSLES = ffi::oboe_AudioApi_OpenSLES,

    /**
     * Try to use AAudio. Fail if unavailable.
     */
    AAudio = ffi::oboe_AudioApi_AAudio,
}

/**
 * Specifies the quality of the sample rate conversion performed by Oboe.
 * Higher quality will require more CPU load.
 * Higher quality conversion will probably be implemented using a sinc based resampler.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum SampleRateConversionQuality {
    #[default]
    None = ffi::oboe_SampleRateConversionQuality_None,

    /**
     * Fastest conversion but may not sound great.
     * This may be implemented using bilinear interpolation.
     */
    Fastest = ffi::oboe_SampleRateConversionQuality_Fastest,

    Low = ffi::oboe_SampleRateConversionQuality_Low,

    Medium = ffi::oboe_SampleRateConversionQuality_Medium,

    High = ffi::oboe_SampleRateConversionQuality_High,

    /**
     * Highest quality conversion, which may be expensive in terms of CPU.
     */
    Best = ffi::oboe_SampleRateConversionQuality_Best,
}

/**
 * The Usage attribute expresses *why* you are playing a sound, what is this sound used for.
 * This information is used by certain platforms or routing policies
 * to make more refined volume or routing decisions.
 *
 * Note that these match the equivalent values in AudioAttributes in the Android Java API.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum Usage {
    #[default]
    Media = ffi::oboe_Usage_Media,

    /**
     * Use this for voice over IP, telephony, etcetera.
     */
    VoiceCommunication = ffi::oboe_Usage_VoiceCommunication,

    /**
     * Use this for sounds associated with telephony such as busy tones, DTMF, etcetera.
     */
    VoiceCommunicationSignalling = ffi::oboe_Usage_VoiceCommunicationSignalling,

    /**
     * Use this to demand the users attention.
     */
    Alarm = ffi::oboe_Usage_Alarm,

    /**
     * Use this for notifying the user when a message has arrived or some
     * other background event has occured.
     */
    Notification = ffi::oboe_Usage_Notification,

    /**
     * Use this when the phone rings.
     */
    NotificationRingtone = ffi::oboe_Usage_NotificationRingtone,

    /**
     * Use this to attract the users attention when, for example, the battery is low.
     */
    NotificationEvent = ffi::oboe_Usage_NotificationEvent,

    /**
     * Use this for screen readers, etcetera.
     */
    AssistanceAccessibility = ffi::oboe_Usage_AssistanceAccessibility,

    /**
     * Use this for driving or navigation directions.
     */
    AssistanceNavigationGuidance = ffi::oboe_Usage_AssistanceNavigationGuidance,

    /**
     * Use this for user interface sounds, beeps, etcetera.
     */
    AssistanceSonification = ffi::oboe_Usage_AssistanceSonification,

    /**
     * Use this for game audio and sound effects.
     */
    Game = ffi::oboe_Usage_Game,

    /**
     * Use this for audio responses to user queries, audio instructions or help utterances.
     */
    Assistant = ffi::oboe_Usage_Assistant,
}

/**
 * The ContentType attribute describes *what* you are playing.
 * It expresses the general category of the content. This information is optional.
 * But in case it is known (for instance {@link Movie} for a
 * movie streaming service or {@link Speech} for
 * an audio book application) this information might be used by the audio framework to
 * enforce audio focus.
 *
 * Note that these match the equivalent values in AudioAttributes in the Android Java API.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum ContentType {
    Speech = ffi::oboe_ContentType_Speech,

    #[default]
    Music = ffi::oboe_ContentType_Music,

    /**
     * Use this for a movie or video soundtrack.
     */
    Movie = ffi::oboe_ContentType_Movie,

    /**
     * Use this for sound is designed to accompany a user action,
     * such as a click or beep sound made when the user presses a button.
     */
    Sonification = ffi::oboe_ContentType_Sonification,
}

/**
 * Defines the audio source.
 * An audio source defines both a default physical source of audio signal, and a recording
 * configuration.
 *
 * Note that these match the equivalent values in MediaRecorder.AudioSource in the Android Java API.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum InputPreset {
    Generic = ffi::oboe_InputPreset_Generic,

    Camcorder = ffi::oboe_InputPreset_Camcorder,

    #[default]
    VoiceRecognition = ffi::oboe_InputPreset_VoiceRecognition,

    /**
     * Use this preset when doing telephony or voice messaging.
     */
    VoiceCommunication = ffi::oboe_InputPreset_VoiceCommunication,

    /**
     * Use this preset to obtain an input with no effects.
     * Note that this input will not have automatic gain control
     * so the recorded volume may be very low.
     */
    Unprocessed = ffi::oboe_InputPreset_Unprocessed,

    /**
     * Use this preset for capturing audio meant to be processed in real time
     * and played back for live performance (e.g karaoke).
     * The capture path will minimize latency and coupling with playback path.
     */
    VoicePerformance = ffi::oboe_InputPreset_VoicePerformance,
}

/**
 * This attribute can be used to allocate a session ID to the audio stream.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum SessionId {
    #[default]
    None = ffi::oboe_SessionId_None,

    /**
     * Allocate a session ID that can be used to attach and control
     * effects using the Java AudioEffects API.
     * Note that the use of this flag may result in higher latency.
     *
     * Note that this matches the value of `AudioManager.AUDIO_SESSION_ID_GENERATE`.
     */
    Allocate = ffi::oboe_SessionId_Allocate,
}

/**
 * The channel count of the audio stream.
 * Use of this enum is convenient to avoid "magic"
 * numbers when specifying the channel count.
 *
 * For example, you can write
 * `builder.set_channel_count(ChannelCount::Stereo)`
 * rather than `builder.set_channel_count(2).
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum ChannelCount {
    #[default]
    Unspecified = ffi::oboe_ChannelCount_Unspecified,

    /**
     * Use this for mono audio.
     */
    Mono = ffi::oboe_ChannelCount_Mono,

    /**
     * Use this for stereo audio.
     */
    Stereo = ffi::oboe_ChannelCount_Stereo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ChannelMask {
    #[default]
    Unspecified = ffi::oboe_ChannelMask_Unspecified,
    FrontLeft = ffi::oboe_ChannelMask_FrontLeft,
    FrontRight = ffi::oboe_ChannelMask_FrontRight,
    FrontCenter = ffi::oboe_ChannelMask_FrontCenter,
    LowFrequency = ffi::oboe_ChannelMask_LowFrequency,
    BackLeft = ffi::oboe_ChannelMask_BackLeft,
    BackRight = ffi::oboe_ChannelMask_BackRight,
    FrontLeftOfCenter = ffi::oboe_ChannelMask_FrontLeftOfCenter,
    FrontRightOfCenter = ffi::oboe_ChannelMask_FrontRightOfCenter,
    BackCenter = ffi::oboe_ChannelMask_BackCenter,
    SideLeft = ffi::oboe_ChannelMask_SideLeft,
    SideRight = ffi::oboe_ChannelMask_SideRight,
    TopCenter = ffi::oboe_ChannelMask_TopCenter,
    TopFrontLeft = ffi::oboe_ChannelMask_TopFrontLeft,
    TopFrontCenter = ffi::oboe_ChannelMask_TopFrontCenter,
    TopFrontRight = ffi::oboe_ChannelMask_TopFrontRight,
    TopBackLeft = ffi::oboe_ChannelMask_TopBackLeft,
    TopBackCenter = ffi::oboe_ChannelMask_TopBackCenter,
    TopBackRight = ffi::oboe_ChannelMask_TopBackRight,
    TopSideLeft = ffi::oboe_ChannelMask_TopSideLeft,
    TopSideRight = ffi::oboe_ChannelMask_TopSideRight,
    BottomFrontLeft = ffi::oboe_ChannelMask_BottomFrontLeft,
    BottomFrontCenter = ffi::oboe_ChannelMask_BottomFrontCenter,
    BottomFrontRight = ffi::oboe_ChannelMask_BottomFrontRight,
    LowFrequency2 = ffi::oboe_ChannelMask_LowFrequency2,
    FrontWideLeft = ffi::oboe_ChannelMask_FrontWideLeft,
    FrontWideRight = ffi::oboe_ChannelMask_FrontWideRight,
    Stereo = ffi::oboe_ChannelMask_Stereo,
    CM2Point1 = ffi::oboe_ChannelMask_CM2Point1,
    Tri = ffi::oboe_ChannelMask_Tri,
    TriBack = ffi::oboe_ChannelMask_TriBack,
    CM3Point1 = ffi::oboe_ChannelMask_CM3Point1,
    CM2Point0Point2 = ffi::oboe_ChannelMask_CM2Point0Point2,
    CM2Point1Point2 = ffi::oboe_ChannelMask_CM2Point1Point2,
    CM3Point0Point2 = ffi::oboe_ChannelMask_CM3Point0Point2,
    CM3Point1Point2 = ffi::oboe_ChannelMask_CM3Point1Point2,
    Quad = ffi::oboe_ChannelMask_Quad,
    QuadSide = ffi::oboe_ChannelMask_QuadSide,
    Surround = ffi::oboe_ChannelMask_Surround,
    Penta = ffi::oboe_ChannelMask_Penta,
    CM5Point1 = ffi::oboe_ChannelMask_CM5Point1,
    CM5Point1Side = ffi::oboe_ChannelMask_CM5Point1Side,
    CM6Point1 = ffi::oboe_ChannelMask_CM6Point1,
    CM7Point1 = ffi::oboe_ChannelMask_CM7Point1,
    CM5Point1Point2 = ffi::oboe_ChannelMask_CM5Point1Point2,
    CM5Point1Point4 = ffi::oboe_ChannelMask_CM5Point1Point4,
    CM7Point1Point2 = ffi::oboe_ChannelMask_CM7Point1Point2,
    CM7Point1Point4 = ffi::oboe_ChannelMask_CM7Point1Point4,
    CM9Point1Point4 = ffi::oboe_ChannelMask_CM9Point1Point4,
    CM9Point1Point6 = ffi::oboe_ChannelMask_CM9Point1Point6,
    FrontBack = ffi::oboe_ChannelMask_FrontBack,
}

impl ChannelMask {
    #[allow(non_upper_case_globals)]
    pub const Mono: Self = Self::FrontLeft;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum AllowedCapturePolicy {
    #[default]
    Unspecified = ffi::oboe_AllowedCapturePolicy_Unspecified,
    All = ffi::oboe_AllowedCapturePolicy_All,
    System = ffi::oboe_AllowedCapturePolicy_System,
    None = ffi::oboe_AllowedCapturePolicy_None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum PrivacySensitiveMode {
    #[default]
    Unspecified = ffi::oboe_PrivacySensitiveMode_Unspecified,
    Disabled = ffi::oboe_PrivacySensitiveMode_Disabled,
    Enabled = ffi::oboe_PrivacySensitiveMode_Enabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum SpatializationBehavior {
    #[default]
    Unspecified = ffi::oboe_SpatializationBehavior_Unspecified,
    Auto = ffi::oboe_SpatializationBehavior_Auto,
    Never = ffi::oboe_SpatializationBehavior_Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum FlushFromAccuracy {
    #[default]
    Undefined = 0,
    Accurate = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive, Default)]
#[repr(i32)]
pub enum MMapPolicy {
    #[default]
    Unspecified = -1,
    Never = 1,
    Auto = 2,
    Always = 3,
}

/**
 * The default (optimal) audio streaming values.
 *
 * On API 16 to 26 OpenSL ES will be used.
 * When using OpenSL ES the optimal values for `sample_rate` and
 * `frames_per_burst` are not known by the native code.
 * On API 17+ these values should be obtained from the AudioManager using this code:
 *
 * ```java
 * // Note that this technique only works for built-in speakers and headphones.
 * AudioManager myAudioMgr = (AudioManager) getSystemService(Context.AUDIO_SERVICE);
 * String sampleRateStr = myAudioMgr.getProperty(AudioManager.PROPERTY_OUTPUT_SAMPLE_RATE);
 * int defaultSampleRate = Integer.parseInt(sampleRateStr);
 * String framesPerBurstStr = myAudioMgr.getProperty(AudioManager.PROPERTY_OUTPUT_FRAMES_PER_BUFFER);
 * int defaultFramesPerBurst = Integer.parseInt(framesPerBurstStr);
 * ```
 *
 * It can then be passed down to Oboe through JNI.
 *
 * AAudio will get the optimal `frames_per_burst` from the HAL and will ignore this value.
 */
pub struct DefaultStreamValues(());

macro_rules! default_stream_value {
    ($getter:ident, $setter:ident, $ffi_sym:ident) => {
        pub fn $getter() -> i32 {
            // SAFETY: Reading a static mut global. Safe as long as no concurrent
            // write occurs (see corresponding set_* SAFETY note).
            unsafe { ffi::$ffi_sym }
        }

        pub fn $setter(value: i32) {
            // SAFETY: Writing a static mut global. Caller must ensure no concurrent
            // read or write from another thread — typically called once at startup
            // before any streams are created.
            unsafe {
                ffi::$ffi_sym = value;
            }
        }
    };
}

impl DefaultStreamValues {
    default_stream_value!(
        get_sample_rate,
        set_sample_rate,
        oboe_DefaultStreamValues_SampleRate
    );
    default_stream_value!(
        get_frames_per_burst,
        set_frames_per_burst,
        oboe_DefaultStreamValues_FramesPerBurst
    );
    default_stream_value!(
        get_channel_count,
        set_channel_count,
        oboe_DefaultStreamValues_ChannelCount
    );
}

/**
 * The time at which the frame at `position` was presented
 */
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FrameTimestamp {
    /**
     * The position in number of frames
     */
    pub position: i64,

    /**
     * The timestamp in nanoseconds
     */
    pub timestamp: i64,
}
