#![allow(dead_code)]

use crate::utils::FuzzyEq;
use crate::F;
use std::io;
use std::io::Write;
use std::ops::{Add, Mul, Sub};

pub fn color(r: impl Into<F>, g: impl Into<F>, b: impl Into<F>) -> Color {
    Color::new(r.into(), g.into(), b.into())
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: F,
    pub green: F,
    pub blue: F,
}

impl Color {
    fn new(red: F, green: F, blue: F) -> Self {
        Self { red, green, blue }
    }

    fn black() -> Self {
        color(0, 0, 0)
    }

    fn clip(self, lo: F, hi: F) -> Self {
        Self::new(
            self.red.max(lo).min(hi),
            self.green.max(lo).min(hi),
            self.blue.max(lo).min(hi),
        )
    }

    fn to_u8(self) -> (u8, u8, u8) {
        let c = self.clip(0.0, 1.0);
        (
            (c.red * 255.0).round() as _,
            (c.green * 255.0).round() as _,
            (c.blue * 255.0).round() as _,
        )
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        color(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        color(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Mul<F> for Color {
    type Output = Color;

    fn mul(self, other: F) -> Self::Output {
        color(self.red * other, self.green * other, self.blue * other)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        color(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl FuzzyEq<Color> for Color {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.red.fuzzy_eq(other.red)
            && self.green.fuzzy_eq(other.green)
            && self.blue.fuzzy_eq(other.blue)
    }
}

#[must_use]
pub struct Canvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_color(width, height, Color::black())
    }

    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.rows().nth(y).unwrap()[x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.rows_mut().nth(y).unwrap()[x] = color;
    }

    fn rows(&self) -> impl Iterator<Item = &[Color]> {
        self.pixels.chunks_exact(self.width)
    }

    fn rows_mut(&mut self) -> impl Iterator<Item = &mut [Color]> {
        self.pixels.chunks_exact_mut(self.width)
    }

    fn write_ppm_header(&self, writer: &mut impl Write) -> io::Result<()> {
        write!(writer, "P3\n{} {}\n255\n", self.width, self.height)
    }

    fn write_ppm_data(&self, writer: &mut impl Write) -> io::Result<()> {
        for row in self.rows() {
            for (i, pixel) in row.iter().enumerate() {
                if i > 0 {
                    write!(writer, " ")?;
                }

                let (r, g, b) = pixel.to_u8();

                write!(writer, "{r} {g} {b}")?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }

    pub fn write_ppm(&self, writer: &mut impl Write) -> io::Result<()> {
        let mut guard = MaxWidthWriter::new(70, writer);

        self.write_ppm_header(&mut guard)?;
        self.write_ppm_data(&mut guard)
    }
}

struct MaxWidthWriter<'a, T: Write> {
    writer: &'a mut T,
    width: usize,
    line_buffer: Vec<u8>,
}

impl<'a, T: Write> MaxWidthWriter<'a, T> {
    fn new(width: usize, writer: &'a mut T) -> Self {
        Self {
            writer,
            width,
            line_buffer: vec![],
        }
    }

    fn flush_line(&mut self) -> io::Result<()> {
        if let Some(i) = self.line_buffer.iter().rposition(|&b| b == b' ') {
            self.line_buffer[i] = b'\n';

            return self.flush_partial(i);
        }

        Ok(())
    }

    fn flush_partial(&mut self, i: usize) -> io::Result<()> {
        self.writer.write_all(&self.line_buffer[..=i])?;
        self.line_buffer.drain(..=i);

        Ok(())
    }
}

impl<T: Write> Write for MaxWidthWriter<'_, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.line_buffer.extend_from_slice(buf);

        while let Some(i) = self.line_buffer.iter().position(|&b| b == b'\n') {
            self.flush_partial(i)?;
        }

        while self.line_buffer.len() > self.width {
            self.flush_line()?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.write_all(&self.line_buffer)?;
        self.line_buffer.clear();
        self.writer.flush()
    }
}

impl<T: Write> Drop for MaxWidthWriter<'_, T> {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_tuples() {
        let color = color(-0.5, 0.4, 1.7);

        assert_fuzzy_eq!(color.red, -0.5);
        assert_fuzzy_eq!(color.green, 0.4);
        assert_fuzzy_eq!(color.blue, 1.7);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);

        let expected = color(0.9, 0.2, 0.04);

        assert_fuzzy_eq!(c1 * c2, expected);
    }

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(10, c.width);
        assert_eq!(20, c.height);

        for x in 0..c.width {
            for y in 0..c.height {
                assert_fuzzy_eq!(c.pixel_at(x, y), Color::black())
            }
        }
    }
    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);

        let red = color(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        let expected = color(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(expected, c.pixel_at(2, 3));
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);

        let mut buf = vec![];

        c.write_ppm(&mut buf).unwrap();

        let expected = "\
P3
5 3
255
";

        assert_eq!(&buf[..11], expected.as_bytes());
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);

        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let mut buf = vec![];

        c.write_ppm(&mut buf).unwrap();

        let bytes = "\
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
";

        assert_eq!(&buf[11..], bytes.as_bytes())
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let c = Canvas::new_with_color(10, 2, color(1, 0.8, 0.6));

        let mut buf = vec![];

        c.write_ppm(&mut buf).unwrap();

        let ppm = String::from_utf8(buf).unwrap();
        let lines = ppm.lines().skip(3).take(4).collect::<Vec<_>>().join("\n");

        let expected = "\
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153";

        assert_eq!(lines, expected);
    }

    #[test]
    fn ppm_end_newline() {
        let c = Canvas::new(10, 2);
        let mut buf = vec![];
        c.write_ppm(&mut buf).unwrap();

        assert_eq!(buf.last(), Some(&b'\n'))
    }
}
