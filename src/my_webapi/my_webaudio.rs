use js_sys::{ArrayBuffer, Promise, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, AudioBuffer, AudioContext, AudioNode};

pub struct MyWebAudio {
    sounds: Vec<web_sys::HtmlAudioElement>,
}

impl MyWebAudio {
    pub fn new(sounds_bytes: Vec<&[u8]>) -> Self {
        Self {
            sounds: sounds_bytes
                .iter()
                .map(|sound| load_audio_from_u8array(*sound).unwrap())
                .collect(),
        }
    }

    pub fn play_sound(&self, index: usize) {
        self.sounds[index].play().unwrap();
    }

    pub fn set_sound_loop(&self, index: usize, is_loop: bool) {
        self.sounds[index].set_loop(is_loop);
    }
}

pub fn load_audio_from_u8array(u8array: &[u8]) -> Result<web_sys::HtmlAudioElement, JsValue> {
    /* Method 1*/
    // let audio_element = web_sys::HtmlAudioElement::new_with_src("../game_audios/0.wav").unwrap();
    // audio_element.play();

    /* Method 2*/
    // Thanks !!! https://stackoverflow.com/questions/69556755/web-sysurlcreate-object-url-with-blobblob-not-formatting-binary-data-co
    let array = js_sys::Array::new();
    array.push(&Uint8Array::from(u8array).buffer());
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &array,
        web_sys::BlobPropertyBag::new().type_("audio/mp3"),
    )
    .unwrap();

    let audio_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let audio_element = web_sys::HtmlAudioElement::new_with_src(&audio_url).unwrap();

    Ok(audio_element)
}
