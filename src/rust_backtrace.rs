use std::env::var_os;

pub struct RustBacktrace {
    is_set: bool,
}

impl RustBacktrace {
    pub fn read() -> Self {
        let var = var_os("RUST_BACKTRACE");
        if let Some(var) = var {
            let is_set = match var.to_str() {
                Some(str) => match str {
                    "true" | "True" | "TRUE" | "1" => true,
                    _ => false,
                },
                None => false,
            };

            return Self { is_set };
        }

        Self { is_set: false }
    }
}

impl Into<bool> for RustBacktrace {
    fn into(self) -> bool {
        self.is_set
    }
}
