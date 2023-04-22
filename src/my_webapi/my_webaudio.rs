use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioBuffer, AudioContext, AudioNode};

// struct PreloadedAudio {
//     // buffer: AudioBuffer,
//     // source_node: AudioNode,
// }
//
// impl PreloadedAudio {
//     fn new(context: &AudioContext, data: &[u8]) -> Result<Self, JsValue> {
//         // // Create a buffer from the ArrayBuffer
//         // let array_buffer = ArrayBuffer::new(data.len() as u32);
//         // let mut uint8_array = Uint8Array::new(&array_buffer);
//         // uint8_array.set(&Uint8Array::from(&data[..]).unchecked_ref(), 0);
//         // let buffer = context.decode_audio_data(&array_buffer)?;
//         //
//         // let source = context.create_buffer_source()?;
//         //
//         // source.set_buffer(Some(&buffer?));
//
//         // source.connect_with_audio_node(context.destination())?;
//         //
//         // source.start()?;
//
//         Ok(Self {
//             // buffer,
//             // source_node: source_node.into(),
//         })
//     }
//
//     fn play(&self) {
//         // self.source_node.start(0.0)?;
//     }
// }

pub struct MyWebAudio {
    context: AudioContext,
}

impl MyWebAudio {
    pub async fn new() -> Result<Self, JsValue> {
        let context = AudioContext::new()?;

        // Load the audio file into memory
        let data = include_bytes!("../../game_audios/0.wav");
        let array_buffer = ArrayBuffer::new(data.len() as u32);

        let mut uint8_array = Uint8Array::new(&array_buffer);
        uint8_array.set(&Uint8Array::from(&data[..]).unchecked_ref(), 0);

        // let buffer = context.decode_audio_data(&array_buffer).await?;
        let buffer = context.decode_audio_data(&array_buffer)?;

        let source = context.create_buffer_source()?;

        // source.set_buffer(Some(&buffer?));
        source.connect_with_audio_node(&context.destination())?;
        // source.start()?;

        Ok(Self { context })
    }

    pub fn play(&self, index: usize) {
        // self.audios[index].play();
    }
}
