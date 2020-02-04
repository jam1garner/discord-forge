import sys, soundfile, librosa

def resample(in_file):
    target_rate = 48000
    audio_data, audio_sample_rate = soundfile.read(in_file, dtype='float32')
    resampled_data = librosa.resample(librosa.to_mono(audio_data.T), audio_sample_rate, target_rate)
    audio_data = resampled_data.T
    soundfile.write(in_file, audio_data, target_rate, subtype='PCM_16')

if __name__ == '__main__':
    resample(sys.argv[1])

