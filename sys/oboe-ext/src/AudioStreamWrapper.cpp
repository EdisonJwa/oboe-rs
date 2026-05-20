#include "oboe/OboeExt.h"
#include <new>

namespace oboe {
  void AudioStreamShared_clone(const AudioStreamShared *sharedStream,
                               AudioStreamShared *newSharedStream) {
    new (newSharedStream) AudioStreamShared(*sharedStream);
  }

  void AudioStreamShared_delete(AudioStreamShared *sharedStream) {
    sharedStream->~shared_ptr(); // call destructor directly
  }

  AudioStream *AudioStreamShared_deref(AudioStreamShared *sharedStream) {
    return sharedStream->get();
  }

  Result AudioStream_open(AudioStream *oboeStream) {
    return oboeStream->open();
  }

  Result AudioStream_close(AudioStream *oboeStream) {
    return oboeStream->close();
  }

  Result AudioStream_release(AudioStream *oboeStream) {
    return oboeStream->release();
  }

  Result AudioStream_requestStart(AudioStream *oboeStream) {
    return oboeStream->requestStart();
  }

  Result AudioStream_requestPause(AudioStream *oboeStream) {
    return oboeStream->requestPause();
  }

  Result AudioStream_requestFlush(AudioStream *oboeStream) {
    return oboeStream->requestFlush();
  }

  Result AudioStream_requestStop(AudioStream *oboeStream) {
    return oboeStream->requestStop();
  }

  StreamState AudioStream_getState(AudioStream *oboeStream) {
    return oboeStream->getState();
  }

  Result AudioStream_waitForStateChange(AudioStream *oboeStream,
                                        StreamState inputState,
                                        StreamState *nextState,
                                        int64_t timeoutNanoseconds) {
    return oboeStream->waitForStateChange(inputState,
                                          nextState,
                                          timeoutNanoseconds);
  }

  ResultWithValue<int32_t>
  AudioStream_setBufferSizeInFrames(AudioStream *oboeStream,
                                    int32_t requestedFrames) {
    return oboeStream->setBufferSizeInFrames(requestedFrames);
  }

  ResultWithValue<int32_t>
  AudioStream_getXRunCount(AudioStream *oboeStream) {
    return oboeStream->getXRunCount();
  }

  bool AudioStream_isXRunCountSupported(const AudioStream *oboeStream) {
    return oboeStream->isXRunCountSupported();
  }

  int32_t AudioStream_getFramesPerBurst(AudioStream *oboeStream) {
    return oboeStream->getFramesPerBurst();
  }

  int32_t AudioStream_getBytesPerFrame(AudioStream *oboeStream) {
    return oboeStream->getBytesPerFrame();
  }

  int32_t AudioStream_getBytesPerSample(AudioStream *oboeStream) {
    return oboeStream->getBytesPerSample();
  }

  int64_t AudioStream_getFramesWritten(AudioStream *oboeStream) {
    return oboeStream->getFramesWritten();
  }

  int64_t AudioStream_getFramesRead(AudioStream *oboeStream) {
    return oboeStream->getFramesRead();
  }

  ResultWithValue<FrameTimestamp> AudioStream_getTimestamp(AudioStream *oboeStream,
                                                           clockid_t clockId) {
    return oboeStream->getTimestamp(clockId);
  }

  ResultWithValue<int32_t> AudioStream_getAvailableFrames(AudioStream *oboeStream) {
    return oboeStream->getAvailableFrames();
  }

  ResultWithValue<int32_t>
  AudioStream_waitForAvailableFrames(AudioStream *oboeStream,
                                     int32_t numFrames,
                                     int64_t timeoutNanoseconds) {
    return oboeStream->waitForAvailableFrames(numFrames, timeoutNanoseconds);
  }

  Result AudioStream_getLastErrorCallbackResult(AudioStream *oboeStream) {
    return oboeStream->getLastErrorCallbackResult();
  }

  int32_t AudioStream_getDelayBeforeCloseMillis(AudioStream *oboeStream) {
    return oboeStream->getDelayBeforeCloseMillis();
  }

  void AudioStream_setDelayBeforeCloseMillis(AudioStream *oboeStream,
                                             int32_t delay) {
    oboeStream->setDelayBeforeCloseMillis(delay);
  }

  void AudioStream_setPerformanceHintEnabled(AudioStream *oboeStream,
                                             bool enabled) {
    oboeStream->setPerformanceHintEnabled(enabled);
  }

  bool AudioStream_isPerformanceHintEnabled(AudioStream *oboeStream) {
    return oboeStream->isPerformanceHintEnabled();
  }

  bool AudioStream_usesAAudio(AudioStream *oboeStream) {
    return oboeStream->usesAAudio();
  }

  ResultWithValue<double>
  AudioStream_calculateLatencyMillis(AudioStream *oboeStream) {
    return oboeStream->calculateLatencyMillis();
  }

  AudioApi AudioStream_getAudioApi(const AudioStream *oboeStream) {
    return oboeStream->getAudioApi();
  }

  ResultWithValue<int32_t> AudioStream_read(AudioStream *oboeStream,
                                            void* buffer,
                                            int32_t numFrames,
                                            int64_t timeoutNanoseconds) {
    return oboeStream->read(buffer, numFrames, timeoutNanoseconds);
  }

  ResultWithValue<int32_t> AudioStream_write(AudioStream *oboeStream,
                                             const void* buffer,
                                             int32_t numFrames,
                                             int64_t timeoutNanoseconds) {
    return oboeStream->write(buffer, numFrames, timeoutNanoseconds);
  }

  AudioStreamBase* AudioStream_getBase(AudioStream *oboeStream) {
    return static_cast<AudioStreamBase*>(oboeStream);
  }

  ChannelMask AudioStreamBase_getChannelMask(AudioStreamBase *base) {
    return base->getChannelMask();
  }

  AllowedCapturePolicy AudioStreamBase_getAllowedCapturePolicy(AudioStreamBase *base) {
    return base->getAllowedCapturePolicy();
  }

  PrivacySensitiveMode AudioStreamBase_getPrivacySensitiveMode(AudioStreamBase *base) {
    return base->getPrivacySensitiveMode();
  }

  bool AudioStreamBase_isContentSpatialized(AudioStreamBase *base) {
    return base->isContentSpatialized();
  }

  SpatializationBehavior AudioStreamBase_getSpatializationBehavior(AudioStreamBase *base) {
    return base->getSpatializationBehavior();
  }

  int32_t AudioStreamBase_getHardwareChannelCount(AudioStreamBase *base) {
    return base->getHardwareChannelCount();
  }

  int32_t AudioStreamBase_getHardwareSampleRate(AudioStreamBase *base) {
    return base->getHardwareSampleRate();
  }

  AudioFormat AudioStreamBase_getHardwareFormat(AudioStreamBase *base) {
    return base->getHardwareFormat();
  }
}
