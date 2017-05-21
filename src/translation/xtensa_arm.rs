
use r2pipe::R2Pipe;

pub trait TranslationOutput {
    fn get_string() -> String;
}

pub struct Translator {

}

impl Translator {
    pub fn new() -> Translator {
        Translator {}
    }
}
