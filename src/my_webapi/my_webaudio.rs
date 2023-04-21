// pub fn play_mp3(mp3_data: &[u8]) {
//     // create a new AudioContext
//     let context = AudioContext::new().unwrap();
//
//     // create an AudioBuffer from the MP3 data
//     context
//         .decode_audio_data(mp3_data)
//         .unwrap()
//         .then(|result| {
//             let buffer = result.unwrap();
//             let source = context.create_buffer_source().unwrap();
//             source.set_buffer(Some(&buffer));
//             source.connect_with_audio_node(&context.destination()).unwrap();
//             source.start().unwrap();
//             Ok(())
//         })
//         .unwrap();
// }
