use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioBuffer, AudioContext, AudioNode, BaseAudioContext};

struct PreloadedAudio {
    // buffer: AudioBuffer,
    // source_node: AudioNode,
}

impl PreloadedAudio {
    fn new(context: AudioContext, data: &[u8]) -> Result<Self, JsValue> {
        // Create a buffer from the ArrayBuffer
        let array_buffer = Uint8Array::from(data).buffer();

        // Decode the audio data
        let buffer = context.decode_audio_data(&array_buffer)?;

        // Create a source node and connect it to the audio context
        let source_node = context.create_buffer()?;

        // // Create a source node and connect it to the audio context
        // let source_node = context.create_buffer()?;
        // source_node.set_buffer(Some(&buffer));

        // source_node.set_buffer(Some(&buffer));
        // source_node.connect_with_audio_node(&context.destination())?;

        Ok(Self { /*buffer, source_node*/ })
    }

    fn play(&self) {
        // self.source_node.start().unwrap();
    }
}

pub struct MyWebAudio {
    context: AudioContext,
    audios: [PreloadedAudio; 0],
}

impl MyWebAudio {
    pub fn new() -> Result<MyWebAudio, JsValue> {
        let context = AudioContext::new()?;

        // let buffer = context.create_buffer(9, 44100, 44100)?;

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
