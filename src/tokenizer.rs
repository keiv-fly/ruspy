use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Tokens<'a> {
    Str(&'a str),
    Int32(i32),
    Float64(f64),
    Identifier(&'a str),
    EqSign,
    Error(String, usize, usize, usize, usize),
    Unimplemented,
    NotSet,
}

pub fn run(input_string: &str) -> Result<Vec<(Vec<(Tokens, usize)>, usize)>, Box<dyn Error>> {
    let mut res = vec![];
    let mut line_res: Vec<(Tokens, usize)> = vec![];
    let mut errs: Vec<Tokens> = vec![];
    let mut real_line = 0;
    let mut logical_line = 0;
    let mut column = 0;
    let mut it = input_string.char_indices();
    while let Some((i, ch)) = it.next() {
        if ch == '\n' {
            res.push((line_res, real_line));
            real_line += 1;
            logical_line += 1;
            column = 0;
            line_res = vec![];
            continue;
        } else {
            let token = match ch {
                // var or fn identifier
                'A'..='Z' | 'a'..='z' | '_' => get_identifier(i, ch, &mut it, input_string),
                // number
                '0'..='9' => get_number(i, ch, &mut it, input_string),
                // string
                '"' => Tokens::Unimplemented,
                // Equal sign
                '=' => Tokens::EqSign,
                ' ' => {
                    column += 1;
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
                        real_line,
                        column,
                        logical_line,
                    );
                    errs.push(e.clone());
                    e
                }
            };
            line_res.push((token, i));
        }
    }
    res.push((line_res, real_line));
    Ok(res)
}

fn get_identifier<'a>(
    i: usize,
    ch: char,
    it: &mut std::str::CharIndices,
    input_string: &'a str,
) -> Tokens<'a> {
    let i_start = i;
    let mut i_stop = 0;
    while let Some((i, ch)) = it.next() {
        match ch {
            'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => continue,
            _ => {
                i_stop = i;
                break;
            }
        }
    }
    if i_stop == 0 {
        i_stop = input_string.len();
    }
    Tokens::Identifier(&input_string[i_start..i_stop])
}

fn get_number<'a>(
    i: usize,
    ch: char,
    it: &mut std::str::CharIndices,
    input_string: &'a str,
) -> Tokens<'a> {
    let i_start = i;
    let mut i_stop = 0;
    while let Some((i, ch)) = it.next() {
        match ch {
            '0'..='9' => continue,
            _ => {
                i_stop = i;
                break;
            }
        }
    }
    if i_stop == 0 {
        i_stop = input_string.len();
    }
    Tokens::Int32(input_string[i_start..i_stop].parse::<i32>().unwrap())
}
