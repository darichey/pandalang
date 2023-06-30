use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Console {
    buf: Vec<u8>,
}

impl std::io::Write for Console {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        log(&String::from_utf8_lossy(&self.buf));
        self.buf.clear();
        Ok(())
    }
}

pub fn console() -> Console {
    Console { buf: Vec::new() }
}
