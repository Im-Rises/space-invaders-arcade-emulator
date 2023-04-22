use js_sys::{ArrayBuffer, Promise, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, AudioBuffer, AudioContext, AudioNode};

pub struct MyWebAudio {
    sdsqdqs: i32,
    // context: AudioContext,
    // buffers: Vec<Option<AudioBuffer>>,
}

impl MyWebAudio {
    pub fn new() -> Self {
        let result = web_sys::HtmlAudioElement::new_with_src("../game_audios/0.wav").unwrap();
        result.play();

        Self { sdsqdqs: 0 }
    }

    pub fn load_audio_from_u8array(u8array: &[u8]) -> Result<web_sys::HtmlAudioElement, JsValue> {
        // Create a blob from the u8 array
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
            u8array,
            web_sys::BlobPropertyBag::new().type_("audio/mp3"),
        )?;

        // Create a new HtmlAudioElement and set the src attribute to the blob URL
        let audio = web_sys::HtmlAudioElement::new_with_src(blob.as_ref().as_blob_url().as_str())?;

        Ok(audio)
    }

    // pub async fn new() -> Result<Self, JsValue> {
    //     let context = AudioContext::new().map_err(|e| e.into())?;
    //
    //     // Load the audio files into memory
    //     // let filenames = ["audio1.mp3", "audio2.mp3", "audio3.mp3"];
    //     let mut buffers = Vec::with_capacity(9);
    //     for _ in 0..9 {
    //         // let data = include_bytes!(filename);
    //         let data = include_bytes!("../../game_audios/0.wav");
    //         let array_buffer = Uint8Array::new_with_length(data.len() as u32);
    //         array_buffer.copy_from(data);
    //
    //         let buffer = context
    //             .decode_audio_data(&array_buffer.buffer())
    //             // .await
    //             .map_err(|e| e.into())?;
    //         buffers.push(Some(buffer));
    //     }
    //
    //     // let buffers_result = wasm_bindgen_futures::JsFuture::from(buffers).await?;
    //
    //     Ok(Self { context, buffers })
    // }

    // async fn my_async_function(promise: Promise) -> Result<JsValue, JsValue> {
    //     let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    //     Ok(result)
    // }

    // pub fn play(&self, index: usize) {
    //     let buffer = self.buffers[index].as_ref().unwrap();
    //     let source = self.context.create_buffer_source().unwrap();
    //     source.set_buffer(Some(buffer));
    //     source.connect_with_audio_node(&self.context.destination()).unwrap();
    //     source.start_with_when(0.0).unwrap();
    // }
}
