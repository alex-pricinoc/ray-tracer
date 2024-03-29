use crate::{FuzzyEq, F};
use std::io::{Result as IoResult, Write};
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

pub fn color<R: Into<F>, G: Into<F>, B: Into<F>>(r: R, g: G, b: B) -> Color {
    Color::new(r.into(), g.into(), b.into())
}

pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: F,
    pub green: F,
    pub blue: F,
}

impl Color {
    pub fn new(red: F, green: F, blue: F) -> Self {
        Self { red, green, blue }
    }

    pub fn clip(self, lo: F, hi: F) -> Self {
        Self::new(
            self.red.max(lo).min(hi),
            self.green.max(lo).min(hi),
            self.blue.max(lo).min(hi),
        )
    }

    #[must_use]
    pub fn to_u8(self) -> (u8, u8, u8) {
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

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(BLACK, |acc, c| acc + c)
    }
}

impl FuzzyEq<Self> for Color {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.red.fuzzy_eq(&other.red)
            && self.green.fuzzy_eq(&other.green)
            && self.blue.fuzzy_eq(&other.blue)
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
        Self::new_with_color(width, height, BLACK)
    }

    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * self.width + x] = color;
    }

    fn rows(&self) -> impl Iterator<Item = &[Color]> {
        self.pixels.chunks_exact(self.width)
    }

    #[allow(dead_code)]
    fn rows_mut(&mut self) -> impl Iterator<Item = &mut [Color]> {
        self.pixels.chunks_exact_mut(self.width)
    }

    fn write_ppm_header(&self, writer: &mut impl Write) -> IoResult<()> {
        write!(writer, "P3\n{} {}\n255\n", self.width, self.height)
    }

    fn write_ppm_data(&self, writer: &mut impl Write) -> IoResult<()> {
        for row in self.rows() {
            for (i, color) in row.iter().enumerate() {
                if i > 0 {
                    write!(writer, " ")?;
                }

                let (r, g, b) = color.to_u8();

                write!(writer, "{r} {g} {b}")?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }

    pub fn write_ppm(&self, writer: &mut impl Write) -> IoResult<()> {
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

    fn flush_line(&mut self) -> IoResult<()> {
        if let Some(i) = self.line_buffer.iter().rposition(|&b| b == b' ') {
            self.line_buffer[i] = b'\n';

            return self.flush_partial(i);
        }

        Ok(())
    }

    fn flush_partial(&mut self, i: usize) -> IoResult<()> {
        self.writer.write_all(&self.line_buffer[..=i])?;
        self.line_buffer.drain(..=i);

        Ok(())
    }
}

impl<T: Write> Write for MaxWidthWriter<'_, T> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.line_buffer.extend_from_slice(buf);

        while let Some(i) = self.line_buffer.iter().position(|&b| b == b'\n') {
            self.flush_partial(i)?;
        }

        while self.line_buffer.len() > self.width {
            self.flush_line()?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
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

        assert_eq!(color.red, -0.5);
        assert_eq!(color.green, 0.4);
        assert_eq!(color.blue, 1.7);
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
                assert_fuzzy_eq!(c.pixel_at(x, y), BLACK);
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

        assert_eq!(&buf[11..], bytes.as_bytes());
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

        assert_eq!(buf.last(), Some(&b'\n'));
    }
}
