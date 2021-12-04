use colored::Colorize;
use std::fmt;

pub struct InfoBox {
    pub pushed_lines: Vec<String>,
    pub pushed_len: Vec<usize>,
    pub longest_line: usize,
    pub border: bool,
}

impl fmt::Display for InfoBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.border {
            let corners = [
                "┌".bright_black(),
                "┐".bright_black(),
                "└".bright_black(),
                "┘".bright_black(),
            ];
            let dash = "─";
            let bar = "│";
            let top = format!(
                "{}{}{} ",
                corners[0],
                dash.repeat(self.longest_line).bright_black(),
                corners[1]
            );
            let bottom = format!(
                "{}{}{} ",
                corners[2],
                dash.repeat(self.longest_line).bright_black(),
                corners[3]
            );

            writeln!(f, "{}", top)?;

            for (i, line) in self.pushed_lines.iter().enumerate() {
                writeln!(
                    f,
                    "{}{}{}{} ",
                    bar.bright_black(),
                    line,
                    " ".repeat(self.longest_line - self.pushed_len[i]),
                    bar.bright_black()
                )?;
            }

            writeln!(f, "{}", bottom)?;

            writeln!(f)
        } else {
            for line in self.pushed_lines.iter() {
                writeln!(f, "{}{}", line, "     ")?; // some padding so it replace the text behind
            }

            writeln!(f)
        }
    }
}

impl InfoBox {
    pub fn push(&mut self, line: &String, len: usize) {
        self.pushed_lines.push(line.to_string());
        self.pushed_len.push(len);
        if len > self.longest_line {
            self.longest_line = len;
        }
    }
}
