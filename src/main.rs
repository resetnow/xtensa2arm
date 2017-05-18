#![allow(dead_code)]

extern crate r2pipe;
extern crate rustc_serialize;
extern crate clap;

mod object_storage;

use r2pipe::R2Pipe;
use object_storage::{Object, ObjectKind, ObjectStorage};
use clap::{Arg, SubCommand};
use std::option::Option;

#[derive(Default)]
struct App {
    objects: ObjectStorage,
    pipe: Box<Option<R2Pipe>>,
}

impl App {
    pub fn new() -> App {
        Default::default()
    }

    fn symbols_read(&mut self) {
        let symbols = self.pipe_get().cmdj("isj").unwrap();
        self.objects.from_json(symbols);

        println!("Read symbols: {:?}", self.objects.len());
    }

    fn pipe_create(&mut self, input: &str) {
        println!("Opening r2pipe");

        let spawn_input = input.to_string();
        self.pipe = Box::new(Some(R2Pipe::spawn(spawn_input, None).unwrap()));
    }

    fn pipe_get<'a>(&'a mut self) -> &'a mut R2Pipe {
        self.pipe.as_mut().as_mut().unwrap()
    }

    fn pipe_close(&mut self) {
        println!("Closing r2pipe");
        self.pipe_get().close();
    }

    pub fn run(&mut self) {
        let args = clap::App::new("xtensa2arm")
            .version("0.1")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .required(true))
            .get_matches();

        let input = args.value_of("input").unwrap().to_string();

        self.pipe_create(&input);
        self.symbols_read();
        self.pipe_close();
    }
}

fn main() {
    let mut app: App = App::new();
    app.run(); 
}
