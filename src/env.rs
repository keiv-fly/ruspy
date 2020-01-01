use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Vi64T<'a> {
    Raw(Box<Vec<i64>>),
    SliceMut(Box<&'a mut [i64]>),
    Slice(Box<&'a [i64]>),
}

#[derive(Debug, PartialEq)]
pub enum Vu64T<'a> {
    Raw(Box<Vec<u64>>),
    SliceMut(Box<&'a mut [u64]>),
    Slice(Box<&'a [u64]>),
}

#[derive(Debug, PartialEq)]
pub enum Vf64T<'a> {
    Raw(Box<Vec<f64>>),
    SliceMut(Box<&'a mut [f64]>),
    Slice(Box<&'a [f64]>),
}

#[derive(Debug, PartialEq)]
pub enum Vbytes64T<'a> {
    Raw(Box<Vec<u8>>),
    SliceMut(Box<&'a mut [u8]>),
    Slice(Box<&'a [u8]>),
}

#[derive(Debug, PartialEq)]
pub enum StringT<'a> {
    Str(Box<String>),
    StrSliceMut(Box<&'a mut str>),
    StrSlice(Box<&'a str>),
}

#[derive(Debug, PartialEq)]
pub enum VecTypeEnum<'a> {
    Vi64(Vi64T<'a>),
    Vu64(Vu64T<'a>),
    Vf64(Vf64T<'a>),
    Vbytes(Vbytes64T<'a>),
}

#[derive(Debug, PartialEq)]
pub enum BaseTypeEnum<'a> {
    Int64(i64),
    UInt(u64),
    Float64(f64),
    Str(StringT<'a>),
    Vect(VecTypeEnum<'a>),
}

pub type RCBTE<'a> = RefCell<BaseTypeEnum<'a>>;

pub struct Env<'a> {
    pub globals: HashMap<String, RCBTE<'a>>,
}

impl Env<'_> {
    pub fn new<'a>() -> Env<'a> {
        Env {
            globals: HashMap::new(),
        }
    }
}
