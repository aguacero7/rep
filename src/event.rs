use crossterm::event::{self, Event as CEvent, KeyCode, KeyEventKind};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Tick,
    Key(KeyCode),
    Resize(u16, u16),
}

pub struct EventLoop {
    rx: mpsc::Receiver<Event>,
    _tx: mpsc::Sender<Event>,
    _t: thread::JoinHandle<()>,
}

impl EventLoop {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();

        let t = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                if event::poll(Duration::from_millis(10)).unwrap_or(false) {
                    match event::read() {
                        Ok(CEvent::Key(k)) if k.kind == KeyEventKind::Press => {
                            let _ = input_tx.send(Event::Key(k.code));
                        }
                        Ok(CEvent::Resize(w, h)) => {
                            let _ = input_tx.send(Event::Resize(w, h));
                        }
                        _ => {}
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    let _ = input_tx.send(Event::Tick);
                    last_tick = Instant::now();
                }
            }
        });
        Self { rx, _tx: tx, _t: t }
    }

    pub fn recv(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
