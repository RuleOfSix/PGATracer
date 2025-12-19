pub use colors::*;
use std::fs::write;
use std::iter::IntoIterator;

mod colors;

pub struct Canvas {
    grid: Vec<Vec<Color>>,
}

impl IntoIterator for Canvas {
    type Item = Color;
    type IntoIter = std::iter::Flatten<std::vec::IntoIter<Vec<Self::Item>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter().flatten()
    }
}

impl Canvas {
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            grid: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height],
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        self.grid.get(y).and_then(|r| r.get(x))
    }

    #[inline]
    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &Color)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, r): (usize, &Vec<Color>)| {
                r.iter().enumerate().map(move |(x, c)| (x, y, c))
            })
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), &'static str> {
        *self
            .grid
            .get_mut(y)
            .and_then(|r| r.get_mut(x))
            .ok_or("Pixel doesn't exist")? = color;
        Ok(())
    }

    pub fn fill(&mut self, color: Color) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                let _ = self.write_pixel(x, y, color);
            }
        }
    }

    fn to_ppm(&self) -> String {
        let ppm_header = format!("P3\n{} {}\n255\n", self.width(), self.height());
        let ppm_body = self
            .grid
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| {
                        format!(
                            "{} {} {}",
                            Self::to_255(c.red),
                            Self::to_255(c.green),
                            Self::to_255(c.blue)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .flat_map(|s: String| {
                if s.len() < 70 {
                    return vec![s];
                }
                match s.rmatch_indices(" ").find(|&(i, _)| i < 70) {
                    None => vec![s],
                    Some((i, _)) => {
                        let (s1, s2) = s.split_at(i);
                        vec![s1.into(), s2.trim().into()]
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
            + "\n";
        format!("{ppm_header}{ppm_body}")
    }

    #[inline]
    pub fn write_file(&self, filepath: &str) -> std::io::Result<()> {
        write(filepath, self.to_ppm())
    }

    fn to_255(cval: f32) -> u8 {
        let cval = cval.clamp(0.0, 1.0);
        (cval * 255.0).round() as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn canvas_creation() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width(), 10);
        assert_eq!(canvas.height(), 20);

        for color in canvas {
            assert_eq!(color, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn write_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red).unwrap();
        assert_eq!(c.pixel_at(2, 3).copied(), Some(red));
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        println!("{ppm:?}");
        assert!(ppm.starts_with("P3\n5 3\n255\n"));
    }

    #[test]
    fn ppm_values() {
        let mut canv = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canv.write_pixel(0, 0, c1).unwrap();
        canv.write_pixel(2, 1, c2).unwrap();
        canv.write_pixel(4, 2, c3).unwrap();
        let ppm = canv.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn ppm_row_char_limit_70() {
        let mut canv = Canvas::new(10, 2);
        canv.fill(Color::new(1.0, 0.8, 0.6));
        let ppm = canv.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ppm_ends_with_newline() {
        let canvas = Canvas::new(5, 3);
        assert!(canvas.to_ppm().ends_with("\n"));
    }
}
