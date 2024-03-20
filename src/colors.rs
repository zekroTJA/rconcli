const MC_ESCAPE: char = '§';
const ANSI_ESCAPE: char = '\x1b';

#[derive(Default)]
pub struct Transformer {
    buf: String,
    disable_color: bool,
}

impl Transformer {
    pub fn new(disable_color: bool) -> Self {
        Self {
            disable_color,
            ..Default::default()
        }
    }

    pub fn transform(&mut self, v: &str) -> &str {
        let mut split = v.split(MC_ESCAPE);

        let Some(first) = split.next() else {
            return "";
        };

        self.buf.clear();

        self.buf.push_str(first);

        for e in split {
            let Some(clr) = e.chars().next() else {
                break;
            };

            if let Some(ansi_clr) = mc_to_ansi(clr) {
                if !self.disable_color {
                    self.buf.push(ANSI_ESCAPE);
                    self.buf.push_str(ansi_clr);
                }
            } else {
                self.buf.push(MC_ESCAPE);
            }

            self.buf.push_str(&e[1..]);
        }

        return self.buf.as_str();
    }
}

fn mc_to_ansi(clr: char) -> Option<&'static str> {
    match clr {
        '4' => Some("[38;5;160m"), // dark red
        'c' => Some("[38;5;203m"), // red
        '6' => Some("[38;5;208m"), // gold (orange)
        'e' => Some("[38;5;220m"), // yellow
        '2' => Some("[38;5;34m"),  // dark green
        'a' => Some("[38;5;82m"),  // green
        'b' => Some("[38;5;51m"),  // aqua (cyan)
        '3' => Some("[38;5;38m"),  // dark aqua (dark cyan)
        '1' => Some("[38;5;26m"),  // dark blue
        '9' => Some("[38;5;33m"),  // blue
        'd' => Some("[38;5;200m"), // light purple (pink)
        '5' => Some("[38;5;171m"), // dark purple
        'f' => Some("[0m"),        // white (reset)
        '7' => Some("[38;5;248m"), // gray
        '8' => Some("[38;5;242m"), // dark gray
        '0' => Some("[38;5;235m"), // black
        _ => None,
    }
}
