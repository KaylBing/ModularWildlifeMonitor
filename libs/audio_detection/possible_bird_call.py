import numpy as np
import pyaudio
import wave
import time
import datetime
import os
from scipy.signal import butter, lfilter


CHUNK = 1024               # Buffer size
FORMAT = pyaudio.paInt16   # 16-bit audio
CHANNELS = 1               # Mono audio
RATE = 44100               # Sampling rate
MIN_FREQUENCY = 1000.0     # Bird call frequency range (Hz)
MAX_FREQUENCY = 10000.0
AMPLITUDE_THRESHOLD = 1000  # RMS amplitude threshold
MAX_SILENCE = 30           # Max silence duration (seconds)
SILENCE_RMS_THRESHOLD = 0.01  # RMS threshold for detecting silence


last_bird_call_time = time.time()
is_recording = False
rolling_buffer = []


def bandpass_filter(data, lowcut, highcut, fs, order=5):
    nyquist = 0.5 * fs
    low = lowcut / nyquist
    high = highcut / nyquist
    b, a = butter(order, [low, high], btype='band')
    return lfilter(b, a, data)

def calculate_rms(data):
    """Calculate RMS amplitude from audio data."""
    if not data:
        return 0.0

    samples = np.frombuffer(data, dtype=np.int16).astype(np.float32)
    samples = np.nan_to_num(samples)  

    if len(samples) == 0:
        return 0.0

    rms = np.sqrt(np.mean(samples**2))

    if np.isnan(rms):
        print("NaN encountered in RMS calculation.")
        return 0.0
    return float(rms)

def calculate_dominant_frequency(data):
    """Calculate the dominant frequency using FFT."""
    samples = np.frombuffer(data, dtype=np.int16).astype(np.float32)
    if len(samples) == 0:
        return None

    filtered_samples = bandpass_filter(samples, MIN_FREQUENCY, MAX_FREQUENCY, RATE)

    
    fft_result = np.fft.rfft(filtered_samples)
    freqs = np.fft.rfftfreq(len(filtered_samples), 1.0 / RATE)
    magnitudes = np.abs(fft_result)

    if len(magnitudes) == 0:
        return None

    dominant_freq = freqs[np.argmax(magnitudes)]

    if MIN_FREQUENCY <= dominant_freq <= MAX_FREQUENCY:
        return dominant_freq
    else:
        return None

def save_snippet(snippet):
    save_directory = r"C:\Users\angel\Sandbox\calls"  # Correct directory path
    if not os.path.exists(save_directory):
        os.makedirs(save_directory)
        print(f"Created directory: {save_directory}")
    else:
        print("Directory exists")

    timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S_%f")
    filename = f"possible_bird_call_{timestamp}.wav"
    filepath = os.path.join(save_directory, filename)
    try:
        with wave.open(filepath, 'wb') as wf:
            wf.setnchannels(CHANNELS)
            wf.setsampwidth(2)  # 2 bytes for 16-bit audio
            wf.setframerate(RATE)
            wf.writeframes(b''.join(snippet))
        print(f"Saved bird call to {filepath}")
    except Exception as e:
        print(f"Error saving snippet: {e}")

p = pyaudio.PyAudio()


stream = p.open(format=FORMAT, channels=CHANNELS,
                rate=RATE, input=True,
                frames_per_buffer=CHUNK)

print("Recording...")

try:
    while True:
        data = stream.read(CHUNK)
        rms = calculate_rms(data)
        if rms < SILENCE_RMS_THRESHOLD:
            continue

        dominant_freq = calculate_dominant_frequency(data)

        if rms > AMPLITUDE_THRESHOLD and dominant_freq is not None and MIN_FREQUENCY <= dominant_freq <= MAX_FREQUENCY:
            current_time = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            print(f"Bird call detected at {current_time}! Frequency: {dominant_freq:.2f} Hz, Amplitude: {rms:.2f}")
            last_bird_call_time = time.time()
            is_recording = True
            rolling_buffer.append(data)
            print("Recording bird call...")

        elif is_recording:
            rolling_buffer.append(data)
            print("Appending data to rolling buffer...")
            if time.time() - last_bird_call_time > MAX_SILENCE:
                print("Max silence reached. Stopping recording...")
                save_snippet(rolling_buffer)
                rolling_buffer = []
                is_recording = False

except KeyboardInterrupt:
    print("Recording stopped by user")
finally:
    stream.stop_stream()
    stream.close()
    p.terminate()
  
    if is_recording and rolling_buffer:
        print("Saving final snippet...")
        save_snippet(rolling_buffer)
