use js_sys::{ArrayBuffer, Promise, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, AudioBuffer, AudioContext, AudioNode};

pub struct MyWebAudio {
    // audio_context: web_sys::HtmlAudioElement
    // context: AudioContext,
    // buffers: Vec<Option<AudioBuffer>>,
    // audio: web_sys::HtmlAudioElement,
}

impl MyWebAudio {
    pub fn new() -> Self {
        Self {
            // audio_context: Self::load_audio_from_u8array(include_bytes!("../../game_audios/0.wav")).unwrap()
            // load_audio_from_u8array(include_bytes!("../../game_audios/0.wav")).unwrap()
        }
    }

    // pub fn play_audio(&self) {
    //     // self.audio_context.play();
    // }
}

pub fn load_audio_from_u8array(u8array: &[u8]) -> Result<web_sys::HtmlAudioElement, JsValue> {
    /* Method 1*/
    // let result = web_sys::HtmlAudioElement::new_with_src("../game_audios/0.wav").unwrap();
    // result.play();

    /* Method 2*/
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &Uint8Array::from(u8array),
        &web_sys::BlobPropertyBag::new().type_("audio/mp3"),
    )?;

    let audio_src = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let result = web_sys::HtmlAudioElement::new_with_src(audio_src.as_str()).unwrap();

    result.play();

    Ok(result)
}
