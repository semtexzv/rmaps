use ::prelude::*;
use std::cell::{
    RefCell, Cell,
};

use std::sync::mpsc::{
    Sender, Receiver, channel,
};

type Str<'a> = Cow<'a, str>;

pub enum Event {
    Start {
        name: Str<'static>,
        time: PreciseTime,
    },
    End {
        name: Str<'static>,
        time: PreciseTime,
    },
}

thread_local!(static PROF_RECORDER: RefCell<Recorder> = RefCell::new(Recorder::new()));

#[derive(Debug)]
pub struct Profiler {}

#[derive(Clone)]
pub struct Section {
    pub name: Str<'static>,
    pub start: PreciseTime,
    pub end: PreciseTime,
    pub frames: Vec<Section>,
}

impl Section {
    pub fn total(&self) -> Duration {
        return self.start.to(self.end);
    }
}

impl fmt::Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frame {{ {:?}, children : {:?} }}", self.name, self.frames)
    }
}

pub fn finish_frame() -> Option<Section> {
    let res = PROF_RECORDER.with(|p| {
        p.replace(Recorder::new())
    });

    fn process_frame(name: Str<'static>, time: PreciseTime, iterator: &mut impl Iterator<Item=Event>) -> Option<Section> {
        let mut children = vec![];
        loop {
            match iterator.next() {
                Some(Event::Start { name, time }) => {
                    if let Some(frame) = process_frame(name, time, iterator) {
                        children.push(frame);
                    }
                }

                Some(Event::End { name: end_name, time: end }) => {
                    return Some(Section {
                        name,
                        end: end,
                        start: time,
                        frames: children,
                    });
                }
                None => {
                    return None;
                }
            }
        }
        return None;
    }

    fn process_root(recorder: Recorder) -> Option<Section> {
        let mut iter = recorder.events.into_iter();
        if let Some(Event::Start { name, time }) = iter.next() {
            return process_frame(name, time, &mut iter);
        }
        return None;
    }
    return process_root(res);
}


pub struct Recorder {
    names: Vec<Str<'static>>,
    events: Vec<Event>,
}

impl Recorder {
    fn new() -> Self {
        Recorder {
            names: vec![],
            events: vec![],
        }
    }
    fn start(&mut self, name: impl Into<Str<'static>>) {
        let n = name.into();
        self.names.push(n.clone());

        self.events.push(Event::Start {
            time: PreciseTime::now(),
            name: n,
        });
    }
    fn end(&mut self) {
        if let Some(name) = self.names.pop() {
            self.events.push(Event::End {
                time: PreciseTime::now(),
                name: name,
            });
        } else {
            panic!("Too many ends")
        }
    }
}


pub fn frame<'a, R, F: FnOnce() -> R>(name: impl Into<Str<'static>>, f: F) -> R {
    let name = name.into();
    begin(name.clone());
    let res = f();
    end();
    res
}

pub fn begin(name: impl Into<Str<'static>>) {
    PROF_RECORDER.with(|p| {
        p.borrow_mut().start(name);
    });
}

pub fn end() {
    PROF_RECORDER.with(|p| {
        p.borrow_mut().end();
    });
}
