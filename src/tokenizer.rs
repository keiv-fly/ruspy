use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Tokens<'a> {
    Str(&'a str),
    Int64(i64),
    Float64(f64),
    Identifier(&'a str),
    EqSign,
    Error(String, usize, usize, usize),
    // Unimplemented,
}

struct ParseIterator<'a> {
    input_string: &'a str,
    it: std::str::CharIndices<'a>,
    last_el: Option<(usize, char)>,
    real_line: usize,
    column: usize,
    // logical_line: usize,
}

impl ParseIterator<'_> {
    fn new<'a>(input_string: &'a str) -> ParseIterator<'a> {
        let mut it = input_string.char_indices();
        let last_el = it.next();
        ParseIterator {
            input_string,
            it,
            last_el,
            real_line: 0,
            column: 0,
            // logical_line: 0,
        }
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.last_el = self.it.next();
        if let Some((_, ch)) = self.last_el {
            match ch {
                '\n' => {
                    self.real_line += 1;
                    self.column = 0;
                }
                '\r' => {}
                _ => self.column += 1,
            }
        };
        self.last_el
    }
}

pub fn run(input_string: &str) -> Result<Vec<Vec<(Tokens, usize)>>, Box<dyn Error>> {
    let mut res = vec![];
    let mut line_res: Vec<(Tokens, usize)> = vec![];
    let mut errs: Vec<Tokens> = vec![];
    let mut pit = ParseIterator::new(input_string);
    while let Some((i, ch)) = pit.last_el {
        match ch {
            '\n' | ';' => {
                pit.next();
                res.push(line_res);
                line_res = vec![];
            }
            _ => {
                let token = match ch {
                    // var or fn identifier
                    'A'..='Z' | 'a'..='z' | '_' => get_identifier(&mut pit),
                    // number
                    '0'..='9' => get_number(&mut pit),
                    // string
                    '"' => get_string(&mut pit),
                    // Equal sign
                    '=' => {
                        pit.next();
                        Tokens::EqSign
                    }
                    ' ' | '\r' | '\t' => {
                        pit.next();
                        continue;
                    }
                    //All else should raise errors
                    _ => {
                        let e = Tokens::Error(
                            format!(
                                "Parsing error. Char {} cannot be parsed. Not implemented",
                                ch
                            ),
                            i,
                            pit.real_line,
                            pit.column,
                        );
                        pit.next();

                        errs.push(e.clone());
                        e
                    }
                };
                line_res.push((token, i));
            }
        }
    }
    res.push(line_res);
    Ok(res)
}

fn get_identifier<'a>(pit: &mut ParseIterator<'a>) -> Tokens<'a> {
    let (i, _) = pit.last_el.unwrap();
    let i_start = i;
    let mut i_stop = 0;
    while let Some((i, ch)) = pit.next() {
        match ch {
            'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => continue,
            _ => {
                i_stop = i;
                break;
            }
        }
    }
    if i_stop == 0 {
        i_stop = pit.input_string.len();
    }
    Tokens::Identifier(&pit.input_string[i_start..i_stop])
}

fn get_number<'a>(pit: &mut ParseIterator<'a>) -> Tokens<'a> {
    let (i, _) = pit.last_el.unwrap();
    let i_start = i;
    let mut i_stop = 0;
    let mut is_float = false;
    while let Some((i, ch)) = pit.next() {
        match ch {
            '0'..='9' => continue,
            '.' => match is_float {
                false => {
                    is_float = true;
                    continue;
                }
                true => {
                    i_stop = i;
                    break;
                }
            },
            _ => {
                i_stop = i;
                break;
            }
        }
    }
    if i_stop == 0 {
        i_stop = pit.input_string.len();
    }
    match is_float {
        true => Tokens::Float64(pit.input_string[i_start..i_stop].parse::<f64>().unwrap()),
        false => Tokens::Int64(pit.input_string[i_start..i_stop].parse::<i64>().unwrap()),
    }
}

fn get_string<'a>(pit: &mut ParseIterator<'a>) -> Tokens<'a> {
    let (i, _) = pit.last_el.unwrap();
    let i_start = i + 1;
    let real_line = pit.real_line;
    let column = pit.column;
    let mut i_stop = 0;
    while let Some((i, ch)) = pit.next() {
        match ch {
            '\\' => {
                pit.next();
            }
            '"' => {
                i_stop = i;
                pit.next();
                break;
            }
            _ => continue,
        }
    }

    match i_stop {
        0 => Tokens::Error(
            format!(
                "No matching end quote for quote at line {}, column {}",
                real_line, column
            ),
            pit.input_string.len(),
            pit.real_line,
            pit.column,
        ),
        _ => Tokens::Str(&pit.input_string[i_start..i_stop]),
    }
}
