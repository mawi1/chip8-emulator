use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct Beeper {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
}

impl Beeper {
    pub fn new() -> Self {
        let (_stream, _stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&_stream_handle).unwrap();

        let source = rodio::source::SineWave::new(680.0);
        sink.append(source);
        sink.pause();

        Self {
            sink,
            _stream,
            _stream_handle,
        }
    }

    pub fn start(&mut self) {
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.sink.pause();
    }
}
