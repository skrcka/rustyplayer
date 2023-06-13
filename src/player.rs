use rodio::Decoder;
use rodio::OutputStreamHandle;
use rodio::Sink;
use std::fs::File;
use std::path::Path;

use crate::models::MediaFile;

pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new(stream_handle: &OutputStreamHandle) -> Player {
        Player {
            sink: Sink::try_new(stream_handle).unwrap(),
        }
    }

    pub fn play(&self, mediafile: &MediaFile) {
        let file = mediafile.path.as_str();
        println!("Playing: {}", file);
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

    pub fn done(&self) -> bool {
        self.sink.empty()
    }

    pub fn skip(&self, count: Option<u32>) {
        for _ in 0..count.unwrap_or(1) {
            self.sink.skip_one();
        }
    }
}
