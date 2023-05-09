use std::default;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum Token {
    Name(String),
    String(String),
    Eos,
}

#[derive(Debug)]
pub struct Lex {
    input: File,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Lex {
            input: input,
        }
    }

    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0'
        }
    }
}

impl Iterator for Lex {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.read_char();
        match ch {
            ' ' | '\r' | '\t' | '\n' => self.next(),
            '\0' => Some(Token::Eos),
            '"' => {
                let mut s = String::new();
                loop {
                    match self.read_char() {
                        '\0' => panic!("unfinished literal string"),
                        '"' => break,
                        ch => s.push(ch),
                    }
                }
                Some(Token::String(s))
            },
            'A'..='Z' | 'a'..='z' | '_' => {
                let mut name = String::new();
                name.push(ch);
                loop {
                    match self.read_char() {
                        '\0' => break,
                        '_' => name.push('_'),
                        ch if ch.is_alphanumeric() => name.push(ch),
                        _ => {
                            self.input.seek(SeekFrom::Current(-1)).unwrap() ;
                            break;
                        }
                    }
                }
                Some(Token::Name(name))
            },
            _ => None
        }
    }
}


#[cfg(test)]
mod test {
    use std::{fs::File, thread::sleep, time::Duration};

    use super::Lex;

    #[test]
    fn it() {
        let file = File::open("D:/CodeDir/rust/luatalk/.gitignore").unwrap();
        let mut l = Lex::new(file);
        loop {
            println!("{:?}", l.next());
            sleep(Duration::from_millis(500));
        }
    }
}