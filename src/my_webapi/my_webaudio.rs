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
}

pub fn load_audio_from_u8array(u8array: &[u8]) -> Result<web_sys::HtmlAudioElement, JsValue> {
    /* Method 1*/
    // let result_method1 = web_sys::HtmlAudioElement::new_with_src("../game_audios/0.wav").unwrap();
    // result_method1.play();

    /* Method 2*/
    // let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
    //     &Uint8Array::from(u8array),
    //     &web_sys::BlobPropertyBag::new().type_("audio/mp3"),
    // )?;
    //
    // let audio_src = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    //
    // let result = web_sys::HtmlAudioElement::new_with_src(audio_src.as_str()).unwrap();

    // result.play();

    /* Method 3*/
    // let audio_file = web_sys::File::new_with_u8_array_sequence_and_options(
    //     &Uint8Array::from(u8array),
    //     "0qsdsqddqs.wav",
    //     &web_sys::FilePropertyBag::new().type_("audio/mp3"),
    // )?;
    //
    // let audio_src = web_sys::Url::create_object_url_with_blob(&audio_file).unwrap();
    //
    // let result = web_sys::HtmlAudioElement::new_with_src(audio_src.as_str()).unwrap();
    //
    // web_sys::console::log_1(&audio_src.into());
    //
    // result.play();

    /* Method 4*/
    // let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
    //     &Uint8Array::from(u8array),
    //     web_sys::BlobPropertyBag::new().type_("audio/mpeg"),
    // )?;
    //
    // let url = web_sys::Url::create_object_url_with_blob(&blob)?;
    // let audio = web_sys::HtmlAudioElement::new_with_src(&url)?;
    // // audio.play()?;

    // Thanks !!!! https://stackoverflow.com/questions/69556755/web-sysurlcreate-object-url-with-blobblob-not-formatting-binary-data-co
    // let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&u8array) }.into());
    let uint8arr = Uint8Array::from(u8array);
    let array = js_sys::Array::new();
    array.push(&uint8arr.buffer());
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &array,
        web_sys::BlobPropertyBag::new().type_("audio/mp3"),
    )
    .unwrap();

    let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let audio = web_sys::HtmlAudioElement::new_with_src(&download_url).unwrap();

    Ok(audio)
}
