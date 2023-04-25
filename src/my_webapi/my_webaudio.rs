use js_sys::{ArrayBuffer, Promise, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, AudioBuffer, AudioContext, AudioNode};

#[derive(Clone, Copy)]
pub enum SoundType {
    unique_sound,
    loop_sound,
}

impl PartialEq for SoundType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SoundType::unique_sound, SoundType::unique_sound) | (SoundType::loop_sound, SoundType::loop_sound) => true,
            _ => false,
        }
    }
}

pub struct MyWebAudio {
    sounds: Vec<web_sys::HtmlAudioElement>,
    last_sounds_states: Vec<bool>,
}

impl MyWebAudio {
    pub fn new(sounds_bytes: Vec<(&[u8], SoundType)>) -> Self {
        let mut sounds = Vec::new();
        let len = sounds_bytes.len();
        for sound_bytes in sounds_bytes {
            let audio_element = load_audio_from_u8array(sound_bytes.0).unwrap();
            audio_element.set_loop(sound_bytes.1 == SoundType::loop_sound);
            sounds.push(audio_element);
        }

        Self {
            sounds,
            last_sounds_states: vec![false; len],
        }
    }

    pub fn play_sounds(&mut self, sounds_states: &[bool]) {
        for (i, sound) in self.sounds.iter().enumerate() {
            if sounds_states[i] && !self.last_sounds_states[i] {
                sound.play().unwrap();
            }
            // else if !sounds_states[i] && self.last_sounds_states[i] {
            // sound.pause();
            // sound.set_current_time(0.0);
            // }
        }

        self.last_sounds_states = sounds_states.to_vec();
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
