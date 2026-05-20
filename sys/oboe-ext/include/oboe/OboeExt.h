#ifndef OBOE_EXT_H
#define OBOE_EXT_H

#include "oboe/Oboe.h"

namespace oboe {
  typedef std::shared_ptr<AudioStream> AudioStreamShared;

  typedef void (*DropContextHandler)(void *context);

  typedef DataCallbackResult (*AudioReadyHandler)(void *context,
                                                  AudioStream *oboeStream,
                                                  void *audioData,
                                                  int32_t numFrames);

  typedef void (*ErrorCloseHandler)(void *context,
                                    AudioStream *oboeStream,
                                    Result error);

  class AudioStreamCallbackWrapper
      : public AudioStreamDataCallback, public AudioStreamErrorCallback {
  public:
    AudioStreamCallbackWrapper(void *context,
                               const DropContextHandler drop_context,
                               const AudioReadyHandler audio_ready,
                               const ErrorCloseHandler before_close,
                               const ErrorCloseHandler after_close);

    ~AudioStreamCallbackWrapper();

    DataCallbackResult onAudioReady(AudioStream *oboeStream,
                                    void *audioData,
                                    int32_t numFrames);

    void onErrorBeforeClose(AudioStream *oboeStream,
                            Result error);

    void onErrorAfterClose(AudioStream *oboeStream,
                           Result error);

  private:
    void *_context;
    const DropContextHandler _drop_context;
    const AudioReadyHandler _audio_ready;
    const ErrorCloseHandler _before_close;
    const ErrorCloseHandler _after_close;
  };

  void AudioStreamBuilder_create(AudioStreamBuilder *builder);
  void AudioStreamBuilder_delete(AudioStreamBuilder *builder);
  void AudioStreamBuilder_setCallback(AudioStreamBuilder *builder,
                                      void *context,
                                      const DropContextHandler drop_context,
                                      const AudioReadyHandler audio_ready,
                                      const ErrorCloseHandler before_close,
                                      const ErrorCloseHandler after_close);

  AudioApi AudioStreamBuilder_getAudioApi(const AudioStreamBuilder *builder);
  void AudioStreamBuilder_setAudioApi(AudioStreamBuilder *builder, AudioApi api);
  void AudioStreamBuilder_setChannelMask(AudioStreamBuilder *builder,
                                         ChannelMask channelMask);
  void AudioStreamBuilder_setAllowedCapturePolicy(AudioStreamBuilder *builder,
                                                  AllowedCapturePolicy policy);
  void AudioStreamBuilder_setPrivacySensitiveMode(AudioStreamBuilder *builder,
                                                  PrivacySensitiveMode mode);
  void AudioStreamBuilder_setIsContentSpatialized(AudioStreamBuilder *builder,
                                                  bool isSpatialized);
  void AudioStreamBuilder_setSpatializationBehavior(AudioStreamBuilder *builder,
                                                    SpatializationBehavior behavior);
  AudioStreamBase* AudioStreamBuilder_getBase(AudioStreamBuilder *builder);

  Result AudioStreamBuilder_openStreamShared(AudioStreamBuilder *builder,
                                             AudioStreamShared *sharedStream);

  void AudioStreamShared_clone(const AudioStreamShared *sharedStream,
                               AudioStreamShared *newSharedStream);
  void AudioStreamShared_delete(AudioStreamShared *sharedStream);
  AudioStream *AudioStreamShared_deref(AudioStreamShared *sharedStream);
  Result AudioStream_open(AudioStream *oboeStream);
  Result AudioStream_close(AudioStream *oboeStream);
  Result AudioStream_release(AudioStream *oboeStream);
  Result AudioStream_requestStart(AudioStream *oboeStream);
  Result AudioStream_requestPause(AudioStream *oboeStream);
  Result AudioStream_requestFlush(AudioStream *oboeStream);
  Result AudioStream_requestStop(AudioStream *oboeStream);
  StreamState AudioStream_getState(AudioStream *oboeStream);
  Result AudioStream_waitForStateChange(AudioStream *oboeStream,
                                        StreamState inputState,
                                        StreamState *nextState,
                                        int64_t timeoutNanoseconds);
  ResultWithValue<int32_t>
  AudioStream_setBufferSizeInFrames(AudioStream *oboeStream,
                                    int32_t requestedFrames);
  ResultWithValue<int32_t>
  AudioStream_getXRunCount(AudioStream *oboeStream);
  bool AudioStream_isXRunCountSupported(const AudioStream *oboeStream);
  int32_t AudioStream_getFramesPerBurst(AudioStream *oboeStream);
  int32_t AudioStream_getBytesPerFrame(AudioStream *oboeStream);
  int32_t AudioStream_getBytesPerSample(AudioStream *oboeStream);
  int64_t AudioStream_getFramesWritten(AudioStream *oboeStream);
  int64_t AudioStream_getFramesRead(AudioStream *oboeStream);
  ResultWithValue<FrameTimestamp> AudioStream_getTimestamp(AudioStream *oboeStream,
                                                           clockid_t clockId);
  ResultWithValue<int32_t> AudioStream_getAvailableFrames(AudioStream *oboeStream);
  ResultWithValue<int32_t>
  AudioStream_waitForAvailableFrames(AudioStream *oboeStream,
                                     int32_t numFrames,
                                     int64_t timeoutNanoseconds);
  Result AudioStream_getLastErrorCallbackResult(AudioStream *oboeStream);
  int32_t AudioStream_getDelayBeforeCloseMillis(AudioStream *oboeStream);
  void AudioStream_setDelayBeforeCloseMillis(AudioStream *oboeStream,
                                             int32_t delay);
  void AudioStream_setPerformanceHintEnabled(AudioStream *oboeStream,
                                             bool enabled);
  bool AudioStream_isPerformanceHintEnabled(AudioStream *oboeStream);
  bool AudioStream_usesAAudio(AudioStream *oboeStream);
  ResultWithValue<double>
  AudioStream_calculateLatencyMillis(AudioStream *oboeStream);
  AudioApi AudioStream_getAudioApi(const AudioStream *oboeStream);
  ResultWithValue<int32_t> AudioStream_read(AudioStream *oboeStream,
                                            void* buffer,
                                            int32_t numFrames,
                                            int64_t timeoutNanoseconds);
  ResultWithValue<int32_t> AudioStream_write(AudioStream *oboeStream,
                                             const void* buffer,
                                             int32_t numFrames,
                                             int64_t timeoutNanoseconds);

  AudioStreamBase* AudioStream_getBase(AudioStream *oboeStream);
  ChannelMask AudioStreamBase_getChannelMask(AudioStreamBase *base);
  AllowedCapturePolicy AudioStreamBase_getAllowedCapturePolicy(AudioStreamBase *base);
  PrivacySensitiveMode AudioStreamBase_getPrivacySensitiveMode(AudioStreamBase *base);
  bool AudioStreamBase_isContentSpatialized(AudioStreamBase *base);
  SpatializationBehavior AudioStreamBase_getSpatializationBehavior(AudioStreamBase *base);
  int32_t AudioStreamBase_getHardwareChannelCount(AudioStreamBase *base);
  int32_t AudioStreamBase_getHardwareSampleRate(AudioStreamBase *base);
  AudioFormat AudioStreamBase_getHardwareFormat(AudioStreamBase *base);
}

#endif
