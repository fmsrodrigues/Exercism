use core::fmt;
use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::cmp::Ordering;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => color.to_string(),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vec: Vec<ResistorColor> = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    vec.sort_by(|a, b| {
        let color_a_value = color_to_value(*a);
        let color_b_value = color_to_value(*b);

        if color_a_value < color_b_value {
            Ordering::Less
        } else if color_a_value == color_b_value {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    vec
}
