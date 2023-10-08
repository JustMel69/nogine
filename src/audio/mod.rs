use std::{sync::Mutex, cell::RefCell};

use rodio::{OutputStream, Sink, OutputStreamHandle};

pub mod clip;

thread_local! {
    static AUDIO: RefCell<Mutex<Audio>> = const { RefCell::new(Mutex::new(Audio::new())) };
}


pub struct Audio {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
}

impl Audio {
    pub const fn new() -> Self {
        Self { _stream: None, stream_handle: None, sink: None }
    }
    
    pub fn init() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        AUDIO.with(|audio| {
            let audio = audio.borrow_mut();
            let mut audio = audio.lock().unwrap();

            audio._stream = Some(_stream);
            audio.stream_handle = Some(stream_handle);
            audio.sink = Some(sink);
            sink.append(source)
        });
    }

    pub fn play() {
        
    }
}