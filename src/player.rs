extern crate rodio;

use std::fs::File;
use std::path::Path;
use rodio::Decoder;
use rodio::OutputStreamHandle;
use rodio::Sink;
use std::io::Write;

use crate::models::MediaFile;

pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new(stream_handle: &OutputStreamHandle) -> Player {
        Player {
            sink: Sink::try_new(stream_handle).unwrap()
        }
    }

    pub fn play(&self, mediafile: &MediaFile) {
        let file = mediafile.path.as_str();
        // print the file name
        println!("Playing: {}", file);
        std::io::stdout().flush().unwrap();
        let file_path = Path::new(file);
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}
