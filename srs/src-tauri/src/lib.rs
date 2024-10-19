use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, SizedSample};
use minimp3::{Decoder, Error as MP3Error, Frame};
use std::io::Cursor;

#[tauri::command]
fn play_audio(mp3_data: Vec<u8>) -> Result<(), String> {
    // let pcm_data = decode_mp3(&mp3_data).map_err(|e| e.to_string()).unwrap();

    // Set up CPAL
    let host = cpal::default_host();
    let device = host.default_output_device().expect("failed to find output device");
    println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    // // Create and play the stream
    // let stream = match supported_config.sample_format() {
    //     SampleFormat::F32 => build_stream::<f32>(&device, &supported_config.into(), &pcm_data),
    //     SampleFormat::I16 => build_stream::<i16>(&device, &supported_config.into(), &pcm_data),
    //     SampleFormat::U8 => build_stream::<u8>(&device, &supported_config.into(), &pcm_data),
    //     _ => panic!("Unsupported sample format"),
    // }.map_err(|e| e.to_string()).unwrap();

    // stream.play().map_err(|e| e.to_string()).unwrap();

    Ok(())
}

fn decode_mp3(mp3_data: &[u8]) -> Result<Vec<f32>, MP3Error> {
    let mut decoder = Decoder::new(Cursor::new(mp3_data));
    let mut pcm_data = Vec::new();

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, .. }) => pcm_data.extend(data.iter().map(|&s| s as f32 / 32768.0)),
            Err(MP3Error::Eof) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(pcm_data)
}

// fn build_stream<T: SizedSample>(
//     device: &cpal::Device,
//     config: &cpal::StreamConfig,
//     audio_data: &[f32],
// ) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
//     let mut sample_index = 0;
//     let sample_count = audio_data.len();

//     let stream = device.build_output_stream(
//         config,
//         move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
//             for frame in data.chunks_mut(config.channels as usize) {
//                 if sample_index >= sample_count {
//                     sample_index = 0; // Loop the audio
//                 }

//                 let value = audio_data[sample_index];

//                 for sample in frame.iter_mut() {
//                     *sample = value;
//                 }

//                 sample_index += 1;
//             }
//         },
//         |err| eprintln!("An error occurred on the output audio stream: {}", err),
//         None
//     )?;

//     Ok(stream)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
