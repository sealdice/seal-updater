use once_cell::sync::Lazy;

static WIN_TERM: Lazy<bool> = Lazy::new(|| std::env::var("WT_SESSION").is_ok());

pub struct ColoredString {
    original: String,
    codes: Vec<i32>,
}

impl ColoredString {
    pub fn new_with(s: &str, code: i32) -> Self {
        ColoredString {
            original: s.to_owned(),
            codes: vec![code],
        }
    }

    fn push_color(&mut self, code: i32) {
        self.codes.push(code);
    }

    fn from_str(s: &str, code: i32) -> ColoredString {
        ColoredString::new_with(s, code)
    }
}

impl std::fmt::Display for ColoredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let supports_ansi = std::env::consts::OS != "windows" || *WIN_TERM;
        if !supports_ansi {
            write!(f, "{}", self.original)
        } else {
            let codes = self
                .codes
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(";");
            write!(f, "\x1b[{}m{}\x1b[0m", codes, self.original)
        }
    }
}

pub trait Colorize {
    fn black(self) -> ColoredString
    where
        Self: Sized;
    fn red(self) -> ColoredString
    where
        Self: Sized;
    fn green(self) -> ColoredString
    where
        Self: Sized;
    fn yellow(self) -> ColoredString
    where
        Self: Sized;
    fn blue(self) -> ColoredString
    where
        Self: Sized;
    fn magenta(self) -> ColoredString
    where
        Self: Sized;
    fn cyan(self) -> ColoredString
    where
        Self: Sized;
    fn white(self) -> ColoredString
    where
        Self: Sized;

    fn on_black(self) -> ColoredString
    where
        Self: Sized;
    fn on_red(self) -> ColoredString
    where
        Self: Sized;
    fn on_green(self) -> ColoredString
    where
        Self: Sized;
    fn on_yellow(self) -> ColoredString
    where
        Self: Sized;
    fn on_blue(self) -> ColoredString
    where
        Self: Sized;
    fn on_magenta(self) -> ColoredString
    where
        Self: Sized;
    fn on_cyan(self) -> ColoredString
    where
        Self: Sized;
    fn on_white(self) -> ColoredString
    where
        Self: Sized;
}

impl<'a> Colorize for &'a str {
    fn black(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 30)
    }

    fn red(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 31)
    }

    fn green(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 32)
    }

    fn yellow(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 33)
    }

    fn blue(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 34)
    }

    fn magenta(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 35)
    }

    fn cyan(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 36)
    }

    fn white(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 37)
    }

    fn on_black(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 40)
    }

    fn on_red(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 41)
    }

    fn on_green(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 42)
    }

    fn on_yellow(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 43)
    }

    fn on_blue(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 44)
    }

    fn on_magenta(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 45)
    }

    fn on_cyan(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 46)
    }

    fn on_white(self) -> ColoredString
    where
        Self: Sized,
    {
        ColoredString::from_str(self, 47)
    }
}

impl Colorize for ColoredString {
    fn black(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(30);
        self
    }

    fn red(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(31);
        self
    }

    fn green(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(32);
        self
    }

    fn yellow(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(33);
        self
    }

    fn blue(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(34);
        self
    }

    fn magenta(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(35);
        self
    }

    fn cyan(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(36);
        self
    }

    fn white(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(37);
        self
    }

    fn on_black(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(40);
        self
    }

    fn on_red(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(41);
        self
    }

    fn on_green(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(42);
        self
    }

    fn on_yellow(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(43);
        self
    }

    fn on_blue(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(44);
        self
    }

    fn on_magenta(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(45);
        self
    }

    fn on_cyan(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(46);
        self
    }

    fn on_white(mut self) -> ColoredString
    where
        Self: Sized,
    {
        self.push_color(47);
        self
    }
}
