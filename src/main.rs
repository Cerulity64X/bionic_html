use std::{env::args, fs, error::Error};

use rt_format::{ParsedFormat, FormatArgument, Format, NoNamedArguments};

fn fmt_args(spec: &str, args: &[FormatString]) -> String {
    format!("{}", ParsedFormat::parse(spec, args, &NoNamedArguments).unwrap())
}

struct FormatString {
    st: String
}
impl FormatArgument for FormatString {
    fn supports_format(&self, specifier: &rt_format::Specifier) -> bool { true }
    fn fmt_display(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", self.st) }
    fn fmt_debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.fmt_display(f) }
    fn fmt_octal(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.fmt_display(f) }
    fn fmt_lower_hex(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.fmt_display(f) }
    fn fmt_upper_hex(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.fmt_display(f) }
    fn fmt_binary(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.fmt_display(f) }
    fn fmt_lower_exp(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.fmt_display(f) }
    fn fmt_upper_exp(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.fmt_display(f) }
}

fn tofmt(st: &str) -> FormatString { FormatString { st: st.to_string() } }

fn main() -> Result<(), Box<dyn Error>> {
    let mut argv = args();
    argv.next();
    let template = fs::read_to_string("template.html")?;
    if let Some(path) = argv.next() {
        let txt = fs::read_to_string(&path)?;
        let mut out: String = String::new();
        for line in txt.split('\n') {
            let mut words = vec![];
            for word in line.split(' ') {
                if word.len() > 1 {
                    let chrs = word.chars().collect::<Vec<char>>();
                    let (chl, chr) = chrs.split_at(chrs.len() / 2);
                    let (l, r) = (chl.iter().collect::<String>().trim().to_string(), chr.iter().collect::<String>().trim().to_string());
                    words.push(format!("<p class=\"bolded\">{l}</p>{r}"));
                } else {
                    words.push(format!("{}", word.trim().to_string()));
                }
            }
            out.push_str(&format!("{}<br>", words.join(" ")));
        }
        fs::write(format!("{path}.bionic.html"), fmt_args(&template, &[tofmt(&out)]))?;
    } else {
        Err(String::from("No file name provided!"))?;
    }
    Ok(())
}
