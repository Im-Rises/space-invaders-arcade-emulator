use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioBuffer, AudioContext, AudioNode};

struct PreloadedAudio {
    buffer: AudioBuffer,
    source_node: AudioNode,
}

impl PreloadedAudio {
    async fn new(context: &AudioContext, data: &[u8]) -> Result<Self, JsValue> {
        // Decode the audio data
        let buffer = context.decode_audio_data(data)?;

        // Create a source node and connect it to the audio context
        let source_node = context.create_buffer_source();
        source_node.set_buffer(Some(&buffer));
        source_node.connect_with_audio_node(&context.destination())?;

        Ok(Self { buffer, source_node })
    }

    fn play(&self) {
        self.source_node.start().unwrap();
    }
}

pub struct MyWebAudio {
    context: AudioContext,
    audios: [PreloadedAudio; 0],
}

impl MyWebAudio {
    pub fn new() -> Result<MyWebAudio, JsValue> {
        let context = AudioContext::new()?;
        let audios = [
            // PreloadedAudio::new(&context, include_bytes!("../assets/1.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/2.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/3.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/4.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/5.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/6.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/7.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/8.mp3")).await?,
            // PreloadedAudio::new(&context, include_bytes!("../assets/9.mp3")).await?,
        ];
        Ok(MyWebAudio { context, audios })
    }

    pub fn play(&self, index: usize) {
        self.audios[index].play();
    }
}
