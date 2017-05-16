#![allow(dead_code)]

extern crate r2pipe;
extern crate rustc_serialize;
extern crate clap;

mod object_storage;

use r2pipe::R2Pipe;
use object_storage::{Object, ObjectKind, ObjectStorage};
use std::option::Option;

#[derive(Default)]
struct App {
    objects: ObjectStorage,
    pipe: Box<Option<R2Pipe>>
}

impl App {
    fn get_pipe<'a>(&'a mut self) -> &'a R2Pipe {
        self.pipe.as_mut().as_mut().unwrap()
    }

    pub fn new() -> App {
        Default::default()
    }

    pub fn run(&mut self) {

    }
}

fn main() {
    let mut app: App = App::new();
    app.run(); 
}
