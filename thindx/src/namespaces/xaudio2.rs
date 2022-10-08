//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-apis-portal)\]
//! APIs for XAudio2 style audio output

// TODO: document to hell and back

#[doc(inline)] pub use thindx_xaudio2::xaudio2_9::xaudio2::*;

#[doc(inline)] pub use thindx_xaudio2::xaudio2_9::{
    // Real types
    IXAudio2SourceVoiceTyped,

    // Ext. traits
    IXAudio2Ext,
    IXAudio2ExtensionExt,
    IXAudio2MasteringVoiceExt,
    IXAudio2SourceVoiceExt,
    IXAudio2VoiceExt,
};

//  xxx     IXAudio2                                            = xaudio2::sys::IXAudio2
//  xxx     IXAudio2                                            = xaudio2::XAudio2
//  xxx     IXAudio2                                            = xaudio2::IXAudio2Ext
//#cpp2rust IXAudio2::CommitChanges                             = xaudio2::IXAudio2Ext::commit_changes
//#cpp2rust IXAudio2::CreateMasteringVoice                      = xaudio2::IXAudio2Ext::create_mastering_voice
//#cpp2rust IXAudio2::CreateSourceVoice                         = xaudio2::IXAudio2Ext::create_source_voice_typed_callback
//#cpp2rust IXAudio2::CreateSourceVoice                         = xaudio2::IXAudio2Ext::create_source_voice_unchecked
//#cpp2rust IXAudio2::CreateSubmixVoice                         = xaudio2::IXAudio2Ext::create_submix_voice
//#cpp2rust IXAudio2::GetPerformanceData                        = xaudio2::IXAudio2Ext::get_performance_data
//#cpp2rust IXAudio2::RegisterForCallbacks                      = xaudio2::IXAudio2Ext::register_for_callbacks
//#cpp2rust IXAudio2::RegisterForCallbacks                      = xaudio2::IXAudio2Ext::register_for_callbacks_leak
//#cpp2rust IXAudio2::SetDebugConfiguration                     = xaudio2::IXAudio2Ext::set_debug_configuration
//#cpp2rust IXAudio2::StartEngine                               = xaudio2::IXAudio2Ext::start_engine
//#cpp2rust IXAudio2::StopEngine                                = xaudio2::IXAudio2Ext::stop_engine
//#cpp2rust IXAudio2::UnregisterForCallbacks                    = xaudio2::IXAudio2Ext::unregister_for_callbacks
//
//#cpp2rust IXAudio2EngineCallback                              = xaudio2::sys::IXAudio2EngineCallback
//#cpp2rust IXAudio2EngineCallback                              = xaudio2::EngineCallbackWrapper
//#cpp2rust IXAudio2EngineCallback                              = xaudio2::EngineCallback
//#cpp2rust IXAudio2EngineCallback::OnCriticalError             = xaudio2::EngineCallback::on_critical_error
//#cpp2rust IXAudio2EngineCallback::OnProcessingPassEnd         = xaudio2::EngineCallback::on_processing_pass_end
//#cpp2rust IXAudio2EngineCallback::OnProcessingPassStart       = xaudio2::EngineCallback::on_processing_pass_start
//
//  xxx     IXAudio2Extension                                   = xaudio2::sys::IXAudio2Extension
//  xxx     IXAudio2Extension                                   = xaudio2::IXAudio2ExtensionExt
//#cpp2rust IXAudio2Extension::GetProcessingQuantum             = xaudio2::IXAudio2ExtensionExt::get_processing_quantum
//#cpp2rust IXAudio2Extension::GetProcessor                     = xaudio2::IXAudio2ExtensionExt::get_processor
//
//#cpp2rust IXAudio2MasteringVoice                              = xaudio2::sys::IXAudio2MasteringVoice
//#cpp2rust IXAudio2MasteringVoice                              = xaudio2::MasteringVoice
//#cpp2rust IXAudio2MasteringVoice                              = xaudio2::IXAudio2MasteringVoiceExt
//#cpp2rust IXAudio2MasteringVoice::GetChannelMask              = xaudio2::IXAudio2MasteringVoiceExt::get_channel_mask
//
//#cpp2rust IXAudio2SourceVoice                                 = xaudio2::sys::IXAudio2SourceVoice
//#cpp2rust IXAudio2SourceVoice                                 = xaudio2::SourceVoice
//#cpp2rust IXAudio2SourceVoice                                 = xaudio2::IXAudio2SourceVoiceTyped
//#cpp2rust IXAudio2SourceVoice                                 = xaudio2::IXAudio2SourceVoiceExt
//#cpp2rust IXAudio2SourceVoice::Discontinuity                  = xaudio2::IXAudio2SourceVoiceExt::discontinuity
//#cpp2rust IXAudio2SourceVoice::ExitLoop                       = xaudio2::IXAudio2SourceVoiceExt::exit_loop
//#cpp2rust IXAudio2SourceVoice::FlushSourceBuffers             = xaudio2::IXAudio2SourceVoiceExt::flush_source_buffers
//#cpp2rust IXAudio2SourceVoice::GetFrequencyRatio              = xaudio2::IXAudio2SourceVoiceExt::get_frequency_ratio
//#cpp2rust IXAudio2SourceVoice::GetState                       = xaudio2::IXAudio2SourceVoiceExt::get_state
//#cpp2rust IXAudio2SourceVoice::SetFrequencyRatio              = xaudio2::IXAudio2SourceVoiceExt::set_frequency_ratio
//#cpp2rust IXAudio2SourceVoice::SetSourceSampleRate            = xaudio2::IXAudio2SourceVoiceExt::set_source_sample_rate
//#cpp2rust IXAudio2SourceVoice::Start                          = xaudio2::IXAudio2SourceVoiceExt::start
//#cpp2rust IXAudio2SourceVoice::Stop                           = xaudio2::IXAudio2SourceVoiceExt::stop
//#cpp2rust IXAudio2SourceVoice::SubmitSourceBuffer             = xaudio2::IXAudio2SourceVoiceTyped::submit_source_buffer
//
//#cpp2rust IXAudio2SubmixVoice                                 = xaudio2::sys::IXAudio2SubmixVoice
//#cpp2rust IXAudio2SubmixVoice                                 = xaudio2::SubmixVoice
//  xxx     IXAudio2SubmixVoice                                 = xaudio2::IXAudio2SubmixVoiceExt
//
//#cpp2rust IXAudio2Voice                                       = xaudio2::sys::IXAudio2Voice
//#cpp2rust IXAudio2Voice                                       = xaudio2::Voice
//#cpp2rust IXAudio2Voice                                       = xaudio2::IXAudio2VoiceExt
//#cpp2rust IXAudio2Voice::DestroyVoice                         = xaudio2::Voice::destroy_voice
//#cpp2rust IXAudio2Voice::DisableEffect                        = xaudio2::IXAudio2VoiceExt::disable_effect
//#cpp2rust IXAudio2Voice::EnableEffect                         = xaudio2::IXAudio2VoiceExt::enable_effect
//#cpp2rust IXAudio2Voice::GetChannelVolumes                    = xaudio2::IXAudio2VoiceExt::get_channel_volumes
//#cpp2rust IXAudio2Voice::GetEffectParameters                  = xaudio2::IXAudio2VoiceExt::get_effect_parameters_raw
//#cpp2rust IXAudio2Voice::GetEffectState                       = xaudio2::IXAudio2VoiceExt::get_effect_state
//#cpp2rust IXAudio2Voice::GetFilterParameters                  = xaudio2::IXAudio2VoiceExt::get_filter_parameters
//#cpp2rust IXAudio2Voice::GetOutputFilterParameters            = xaudio2::IXAudio2VoiceExt::get_output_filter_parameters
//#cpp2rust IXAudio2Voice::GetOutputMatrix                      = xaudio2::IXAudio2VoiceExt::get_output_matrix
//#cpp2rust IXAudio2Voice::GetVoiceDetails                      = xaudio2::IXAudio2VoiceExt::get_voice_details
//#cpp2rust IXAudio2Voice::GetVolume                            = xaudio2::IXAudio2VoiceExt::get_volume
//#cpp2rust IXAudio2Voice::SetChannelVolumes                    = xaudio2::IXAudio2VoiceExt::set_channel_volumes
//#cpp2rust IXAudio2Voice::SetEffectChain                       = xaudio2::IXAudio2VoiceExt::set_effect_chain
//#cpp2rust IXAudio2Voice::SetEffectParameters                  = xaudio2::IXAudio2VoiceExt::set_effect_parameters_raw
//#cpp2rust IXAudio2Voice::SetFilterParameters                  = xaudio2::IXAudio2VoiceExt::set_filter_parameters
//#cpp2rust IXAudio2Voice::SetOutputFilterParameters            = xaudio2::IXAudio2VoiceExt::set_output_filter_parameters
//#cpp2rust IXAudio2Voice::SetOutputMatrix                      = xaudio2::IXAudio2VoiceExt::set_output_matrix
//#cpp2rust IXAudio2Voice::SetOutputVoices                      = xaudio2::IXAudio2VoiceExt::set_output_voices
//#cpp2rust IXAudio2Voice::SetVolume                            = xaudio2::IXAudio2VoiceExt::set_volume
//
//#cpp2rust IXAudio2VoiceCallback                               = xaudio2::sys::IXAudio2VoiceCallback
//#cpp2rust IXAudio2VoiceCallback                               = xaudio2::VoiceCallbackWrapper
//#cpp2rust IXAudio2VoiceCallback                               = xaudio2::VoiceCallback
//#cpp2rust IXAudio2VoiceCallback::OnBufferEnd                  = xaudio2::VoiceCallback::on_buffer_end
//#cpp2rust IXAudio2VoiceCallback::OnBufferStart                = xaudio2::VoiceCallback::on_buffer_start
//#cpp2rust IXAudio2VoiceCallback::OnLoopEnd                    = xaudio2::VoiceCallback::on_loop_end
//#cpp2rust IXAudio2VoiceCallback::OnStreamEnd                  = xaudio2::VoiceCallback::on_stream_end
//#cpp2rust IXAudio2VoiceCallback::OnVoiceError                 = xaudio2::VoiceCallback::on_voice_error
//#cpp2rust IXAudio2VoiceCallback::OnVoiceProcessingPassEnd     = xaudio2::VoiceCallback::on_voice_processing_pass_end
//#cpp2rust IXAudio2VoiceCallback::OnVoiceProcessingPassStart   = xaudio2::VoiceCallback::on_voice_processing_pass_start
//
//#cpp2rust XAUDIO2_BUFFER                                      =
//#cpp2rust XAUDIO2_BUFFER_WMA                                  =
//#cpp2rust XAUDIO2_DEBUG_CONFIGURATION                         = xaudio2::DebugConfiguration
//#cpp2rust XAUDIO2_EFFECT_CHAIN                                =
//#cpp2rust XAUDIO2_EFFECT_DESCRIPTOR                           = xaudio2::EffectDescriptor
//#cpp2rust XAUDIO2_FILTER_PARAMETERS                           = xaudio2::FilterParameters
//#cpp2rust XAUDIO2_PERFORMANCE_DATA                            = xaudio2::PerformanceData
//#cpp2rust XAUDIO2_SEND_DESCRIPTOR                             = xaudio2::SendDescriptor
//#cpp2rust XAUDIO2_VOICE_DETAILS                               = xaudio2::VoiceDetails
//#cpp2rust XAUDIO2_VOICE_SENDS                                 =
//#cpp2rust XAUDIO2_VOICE_STATE                                 = xaudio2::VoiceState
//
//#cpp2rust XAUDIO2_FILTER_TYPE                                 =
//
//#cpp2rust BandPassFilter                                      =
//#cpp2rust HighPassFilter                                      =
//#cpp2rust HighPassOnePoleFilter                               =
//#cpp2rust LowPassFilter                                       =
//#cpp2rust LowPassOnePoleFilter                                =
//#cpp2rust NotchFilter                                         =
//
//#cpp2rust FACILITY_XAUDIO2                                    = xaudio2::FACILITY
//
//#cpp2rust Processor1                                          = xaudio2::Processor1
//#cpp2rust Processor2                                          = xaudio2::Processor2
//#cpp2ignore Processor3
//#cpp2ignore Processor4
//#cpp2ignore Processor5
//#cpp2ignore Processor6
//#cpp2ignore Processor7
//#cpp2ignore Processor8
//#cpp2ignore Processor9
//#cpp2ignore Processor10
//#cpp2ignore Processor11
//#cpp2ignore Processor12
//#cpp2ignore Processor13
//#cpp2ignore Processor14
//#cpp2ignore Processor15
//#cpp2ignore Processor16
//#cpp2ignore Processor17
//#cpp2ignore Processor18
//#cpp2ignore Processor19
//#cpp2ignore Processor20
//#cpp2ignore Processor21
//#cpp2ignore Processor22
//#cpp2ignore Processor23
//#cpp2ignore Processor24
//#cpp2ignore Processor25
//#cpp2ignore Processor26
//#cpp2ignore Processor27
//#cpp2ignore Processor28
//#cpp2ignore Processor29
//#cpp2ignore Processor30
//#cpp2rust Processor31                                         = xaudio2::Processor31
//#cpp2rust Processor32                                         = xaudio2::Processor32
//
//#cpp2rust XAUDIO2D_DLL                                        = xaudio2::D_DLL
//#cpp2rust XAUDIO2D_DLL_A                                      = xaudio2::D_DLL_A
//#cpp2rust XAUDIO2D_DLL_W                                      = xaudio2::D_DLL_W
//#cpp2rust XAUDIO2_1024_QUANTUM                                =
//#cpp2rust XAUDIO2_ANY_PROCESSOR                               = xaudio2::ANY_PROCESSOR
//#cpp2rust XAUDIO2_COMMIT_ALL                                  = xaudio2::COMMIT_ALL
//#cpp2rust XAUDIO2_COMMIT_NOW                                  = xaudio2::COMMIT_NOW
//#cpp2rust XAUDIO2_DEBUG_ENGINE                                =
//#cpp2rust XAUDIO2_DEFAULT_CHANNELS                            = xaudio2::DEFAULT_CHANNELS
//#cpp2rust XAUDIO2_DEFAULT_FILTER_FREQUENCY                    = xaudio2::DEFAULT_FILTER_FREQUENCY
//#cpp2rust XAUDIO2_DEFAULT_FILTER_ONEOVERQ                     = xaudio2::DEFAULT_FILTER_ONEOVERQ
//#cpp2rust XAUDIO2_DEFAULT_FILTER_TYPE                         = xaudio2::DEFAULT_FILTER_TYPE
//#cpp2rust XAUDIO2_DEFAULT_FREQ_RATIO                          = xaudio2::DEFAULT_FREQ_RATIO
//#cpp2rust XAUDIO2_DEFAULT_PROCESSOR                           = xaudio2::DEFAULT_PROCESSOR
//#cpp2rust XAUDIO2_DEFAULT_SAMPLERATE                          = xaudio2::DEFAULT_SAMPLERATE
//#cpp2rust XAUDIO2_DLL                                         = xaudio2::DLL
//#cpp2rust XAUDIO2_DLL_A                                       = xaudio2::DLL_A
//#cpp2rust XAUDIO2_DLL_W                                       = xaudio2::DLL_W
//#cpp2rust XAUDIO2_END_OF_STREAM                               = xaudio2::END_OF_STREAM
//#cpp2rust XAUDIO2_E_DEVICE_INVALIDATED                        = xaudio2::E_DEVICE_INVALIDATED
//#cpp2rust XAUDIO2_E_INVALID_CALL                              = xaudio2::E_INVALID_CALL
//#cpp2rust XAUDIO2_E_XAPO_CREATION_FAILED                      = xaudio2::E_XAPO_CREATION_FAILED
//#cpp2rust XAUDIO2_E_XMA_DECODER_ERROR                         = xaudio2::E_XMA_DECODER_ERROR
//#cpp2rust XAUDIO2_INVALID_OPSET                               = xaudio2::INVALID_OPSET
//#cpp2rust XAUDIO2_LOG_API_CALLS                               =
//#cpp2rust XAUDIO2_LOG_DETAIL                                  =
//#cpp2rust XAUDIO2_LOG_ERRORS                                  =
//#cpp2rust XAUDIO2_LOG_FUNC_CALLS                              =
//#cpp2rust XAUDIO2_LOG_INFO                                    =
//#cpp2rust XAUDIO2_LOG_LOCKS                                   =
//#cpp2rust XAUDIO2_LOG_MEMORY                                  =
//#cpp2rust XAUDIO2_LOG_STREAMING                               =
//#cpp2rust XAUDIO2_LOG_TIMING                                  =
//#cpp2rust XAUDIO2_LOG_WARNINGS                                =
//#cpp2rust XAUDIO2_LOOP_INFINITE                               = xaudio2::LOOP_INFINITE
//#cpp2rust XAUDIO2_MAX_AUDIO_CHANNELS                          = xaudio2::MAX_AUDIO_CHANNELS
//#cpp2rust XAUDIO2_MAX_BUFFERS_SYSTEM                          = xaudio2::MAX_BUFFERS_SYSTEM
//#cpp2rust XAUDIO2_MAX_BUFFER_BYTES                            = xaudio2::MAX_BUFFER_BYTES
//#cpp2rust XAUDIO2_MAX_FILTER_FREQUENCY                        = xaudio2::MAX_FILTER_FREQUENCY
//#cpp2rust XAUDIO2_MAX_FILTER_ONEOVERQ                         = xaudio2::MAX_FILTER_ONEOVERQ
//#cpp2rust XAUDIO2_MAX_FREQ_RATIO                              = xaudio2::MAX_FREQ_RATIO
//#cpp2rust XAUDIO2_MAX_INSTANCES                               = xaudio2::MAX_INSTANCES
//#cpp2rust XAUDIO2_MAX_LOOP_COUNT                              = xaudio2::MAX_LOOP_COUNT
//#cpp2rust XAUDIO2_MAX_QUEUED_BUFFERS                          = xaudio2::MAX_QUEUED_BUFFERS
//#cpp2rust XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO               = xaudio2::MAX_RATIO_TIMES_RATE_XMA_MONO
//#cpp2rust XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL       = xaudio2::MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL
//#cpp2rust XAUDIO2_MAX_SAMPLE_RATE                             = xaudio2::MAX_SAMPLE_RATE
//#cpp2rust XAUDIO2_MAX_VOLUME_LEVEL                            = xaudio2::MAX_VOLUME_LEVEL
//#cpp2rust XAUDIO2_MIN_FREQ_RATIO                              = xaudio2::MIN_FREQ_RATIO
//#cpp2rust XAUDIO2_MIN_SAMPLE_RATE                             = xaudio2::MIN_SAMPLE_RATE
//#cpp2rust XAUDIO2_NO_LOOP_REGION                              = xaudio2::NO_LOOP_REGION
//#cpp2rust XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT                     =
//#cpp2rust XAUDIO2_PLAY_TAILS                                  = xaudio2::PLAY_TAILS
//#cpp2rust XAUDIO2_QUANTUM_DENOMINATOR                         = xaudio2::QUANTUM_DENOMINATOR
//#cpp2rust XAUDIO2_QUANTUM_MS                                  = xaudio2::QUANTUM_MS
//#cpp2rust XAUDIO2_QUANTUM_NUMERATOR                           = xaudio2::QUANTUM_NUMERATOR
//#cpp2rust XAUDIO2_SEND_USEFILTER                              = xaudio2::SEND_USEFILTER
//#cpp2rust XAUDIO2_STDAPI                                      = extern "system"
//#cpp2rust XAUDIO2_STOP_ENGINE_WHEN_IDLE                       =
//#cpp2rust XAUDIO2_USE_DEFAULT_PROCESSOR                       = xaudio2::USE_DEFAULT_PROCESSOR
//#cpp2rust XAUDIO2_VOICE_NOPITCH                               = xaudio2::VOICE_NOPITCH
//#cpp2rust XAUDIO2_VOICE_NOSAMPLESPLAYED                       = xaudio2::VOICE_NOSAMPLESPLAYED
//#cpp2rust XAUDIO2_VOICE_NOSRC                                 = xaudio2::VOICE_NOSRC
//#cpp2rust XAUDIO2_VOICE_USEFILTER                             = xaudio2::VOICE_USEFILTER
//
//#cpp2ignore _USE_MATH_DEFINES
//#cpp2ignore FWD_DECLARE
//#cpp2ignore X2DEFAULT
