use js_sys::{ArrayBuffer, Promise, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, AudioBuffer, AudioContext, AudioNode};

#[derive(Clone, Copy)]
pub enum SoundType {
    UniqueSound,
    LoopSound,
    VariableLengthSound,
}

impl PartialEq for SoundType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SoundType::UniqueSound, SoundType::UniqueSound) | (SoundType::LoopSound, SoundType::LoopSound) => true,
            _ => false,
        }
    }
}

pub struct MyWebAudio {
    sounds: Vec<web_sys::HtmlAudioElement>,
    sounds_types: Vec<SoundType>,
    last_sounds_states: Vec<bool>,
}

impl MyWebAudio {
    pub fn new(sounds_bytes: Vec<(&[u8], SoundType)>) -> Self {
        let mut sounds = Vec::new();
        let mut sounds_types = Vec::new();
        let len = sounds_bytes.len();
        for sound_bytes in sounds_bytes {
            let audio_element = load_audio_from_u8array(sound_bytes.0).unwrap();
            audio_element.set_loop(sound_bytes.1 == SoundType::LoopSound);
            sounds_types.push(sound_bytes.1);
            sounds.push(audio_element);
        }

        Self {
            sounds,
            sounds_types,
            last_sounds_states: vec![false; len],
        }
    }

    pub fn play_sounds(&mut self, sounds_states: &[bool]) {
        for (i, sound) in self.sounds.iter().enumerate() {
            match self.sounds_types[i] {
                SoundType::UniqueSound => {
                    // If is unique sound, play only on mounting state
                    if sounds_states[i] && !self.last_sounds_states[i] {
                        sound.play().unwrap();
                    }
                }
                SoundType::LoopSound => {
                    // If is loop sound, play only on mounting state ans stop on unmounting state
                    if sounds_states[i] && !self.last_sounds_states[i] {
                        sound.play().unwrap();
                        web_sys::console::log_1(&"play".into());
                    } else if !sounds_states[i] && self.last_sounds_states[i] {
                        sound.pause();
                        sound.set_current_time(0.0);
                    }
                }
                SoundType::VariableLengthSound => {
                    // If is variable length sound, play only on mounting state and stop on unmounting state
                    if sounds_states[i] && !self.last_sounds_states[i] {
                        sound.play().unwrap();
                    } else if !sounds_states[i] && self.last_sounds_states[i] {
                        sound.pause();
                        sound.set_current_time(0.0);
                    }
                }
            }
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
