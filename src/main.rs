#![allow(dead_code)]

extern crate r2pipe;
extern crate rustc_serialize;
extern crate clap;

mod object_storage;
mod function;
mod assembly;
mod translation;

use r2pipe::R2Pipe;

use function::{Function};
use object_storage::{Object, ObjectKind, ObjectStorage};
use translation::xtensa_arm::Translator;

use clap::{Arg, SubCommand};

use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::option::Option;
use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Default)]
struct App {
    objects: ObjectStorage,
    functions_in: Vec<Function>,
    functions_out: Vec<Function>,
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

    fn analyze(&mut self) {
        self.pipe_get().cmd("aa").unwrap();
    }

    fn functions_create(&mut self) {
        let names = vec!["sdk_rom_i2c_writeReg"];

        for function in &names {
            let mut f = Function::new();
            let command = format!("pdfj @ sym.{:}", function);
            let json = self.pipe_get().cmdj(&command).unwrap();

            f.from_json(json);
            f.name = function.to_string();

            self.functions_in.push(f);
        }
    }

    fn functions_translate(&mut self) {
        let mut translator = Translator::new();
        let mut pipe = self.pipe.as_mut().as_mut().unwrap();

        for function in &mut self.functions_in {
            let function = translator.translate(function, pipe);
            self.functions_out.push(function);
        }
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

    fn output_write(&self) {
        let mut file = match File::create("result.S") {
            Err(why) => panic!("Couldn't open output file: {}", why.description()),
            Ok(file) => file,
        };

        let includes = vec!["esp8266.S"];

        for include in &includes {
            let s = format!("#include \"{:}\"\n", include);
            file.write(s.as_bytes()).unwrap();
        }

        for function in &self.functions_out {
            let header = format!(
                "\n\n\
                .global {:};\n\
                {:}:\n",
                function.name, function.name
            );

            file.write(header.as_bytes()).unwrap();

            for instruction in &function.instructions {
                if instruction.referenced {
                    let reference = format!("loc_{:x}:\n", instruction.offset);
                    file.write(reference.as_bytes()).unwrap();
                }

                file.write("\t".as_bytes()).unwrap();
                file.write(instruction.opcode.as_bytes()).unwrap();
                file.write("\n".as_bytes()).unwrap();
            }
        }
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
        self.analyze();
        self.symbols_read();
        self.functions_create();
        self.functions_translate();
        self.output_write();
        self.pipe_close();
    }
}

fn main() {
    let mut app: App = App::new();
    app.run();
}
