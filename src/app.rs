use std::fmt::Display;

pub struct App {
    port: usize,
    host: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "localhost".to_string(),
        }
    }
}

trait Asd<T: Display> {
    fn asd(&self, param: T) {
        param.to_string();
    }
}

impl<T: Display> Asd<T> for App {}

trait WithPort {
    fn get_port(self) -> usize;
}
trait WithHost {
    fn get_host(self) -> String;
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn inject_dep(self) -> Self {
        todo!()
    }
    pub fn run(self) {
        todo!()
    }
}

fn yo(c: App) {}

fn asd(asd: Asd) {
    asd.asd(param)
}
