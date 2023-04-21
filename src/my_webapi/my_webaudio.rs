use wasm_bindgen::{JsCast, JsValue};

pub(crate) struct MyWebAudio {
    context: AudioContext,
    source: AudioBufferSourceNode,
    gain: GainNode,
    analyser: AnalyserNode,
    buffer: AudioBuffer,
}

impl MyWebAudio {
    pub fn new() -> Result<MyWebAudio, JsValue> {
        let context = AudioContext::new()?;
        let source = context.create_buffer_source()?;
        let gain = context.create_gain()?;
        let analyser = context.create_analyser()?;
        analyser.set_fft_size(2048);
        analyser.set_max_decibels(0.0);
        analyser.set_min_decibels(-100.0);
        analyser.set_smoothing_time_constant(0.8);
        source.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&analyser)?;
        analyser.connect_with_audio_node(&context.destination())?;
        Ok(MyWebAudio {
            context,
            source,
            gain,
            analyser,
            buffer: AudioBuffer::new(0, 0, 0.0).unwrap(),
        })
    }

    pub fn play_mp3(&mut self, mp3_data: &[u8]) -> Result<(), JsValue> {
        self.context.decode_audio_data(mp3_data)?.then(|result| {
            let buffer = result.unwrap();
            self.source.set_buffer(Some(&buffer));
            self.source.start()?;
            self.buffer = buffer;
            Ok(())
        })?;
        Ok(())
    }

    // pub fn get_frequency_data(&self) -> Vec<f32> {
    //     let mut data = vec![0.0; self.analyser.frequency_bin_count() as usize];
    //     self.analyser.get_float_frequency_data(&mut data);
    //     data
    // }
    //
    // pub fn get_time_domain_data(&self) -> Vec<f32> {
    //     let mut data = vec![0.0; self.analyser.frequency_bin_count() as usize];
    //     self.analyser.get_float_time_domain_data(&mut data);
    //     data
    // }
    //
    // pub fn get_duration(&self) -> f64 {
    //     self.buffer.duration()
    // }
    //
    // pub fn get_sample_rate(&self) -> f64 {
    //     self.buffer.sample_rate()
    // }
    //
    // pub fn get_channel_data(&self, channel: u32) -> Vec<f32> {
    //     let mut data = vec![0.0; self.buffer.length() as usize];
    //     self.buffer.copy_from_channel(&mut data, channel);
    //     data
    // }
    //
    // pub fn get_channel_count(&self) -> u32 {
    //     self.buffer.channel_count()
    // }
    //
    // pub fn get_length(&self) -> u32 {
    //     self.buffer.length()
    // }
}
