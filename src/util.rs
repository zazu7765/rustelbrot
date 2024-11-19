mod util {
    use num_complex::{Complex, Complex64};

    /// Determine if C is in set, using `threshold` iterations to limit computation
/// 
/// If not a member, return number of iterations taken to leave circle (centered on origin).
/// 
/// Else, return None
fn iter_to_exit(c: Complex<f64>, threshold: usize) -> Option<usize>{
    assert!(threshold != 0);
    assert!(c != Complex{re:0.0, im:0.0});
    let mut z: Complex<f64> = Complex{im:0.0, re: 0.0};

    for i in 0..threshold{
        if z.norm_sqr() > 4_f64{
            return Some(i);
        }

        z = z * z + c;
    }
    None
}

    mod parse {
        use std::str::FromStr;

        use num_complex::{Complex, Complex64};

        /// generic parse from str into type, with separator
        ///
        /// example: 32x32 with separator `x` will return a pair of `(32, 32)`
        fn parse_pair<T: FromStr>(str: &str, sep: char) -> Option<(T, T)> {
            let idx = str.find(sep)?;

            if let (Ok(l), Ok(r)) = (T::from_str(&str[..idx]), T::from_str(&str[idx + 1..])) {
                return Some((l, r));
            }
            None
        }
        #[test]
        fn test_parse_pair() {
            assert_eq!(parse_pair::<i32>("", ','), None);
            assert_eq!(parse_pair::<i32>("32", ','), None);
            assert_eq!(parse_pair::<i32>(",32", ','), None);
            assert_eq!(parse_pair::<i32>("32,32", ','), Some((32, 32)));
            assert_eq!(parse_pair::<i32>("32,32abcd", ','), None);
            assert_eq!(parse_pair::<f32>("32.0x0.32abcd", 'x'), None);
            assert_eq!(parse_pair::<f32>("32.0x0.32", 'x'), Some((32.0, 0.32)));
        }

        /// parse pair implementation for our Complex64 type with a comma `,` delimiter
        pub fn parse_complex(s: &str) -> Option<Complex64> {
            if let Some((re, im)) = parse_pair(s, ',') {
                return Some(Complex64 { re, im });
            }
            None
        }

        #[test]
        fn test_parse_complex() {
            assert_eq!(parse_complex("1.75x,-0.075x"), None);
            assert_eq!(
                parse_complex("1.75,-0.075"),
                Some(Complex {
                    re: 1.75,
                    im: -0.075
                })
            );
        }
    }

    mod img {
        use std::{fs::File, io::BufWriter, iter};

        use num_complex::{Complex, Complex64};

        use crate::util::util::iter_to_exit;

        /// Given image bounds `(width, height)`, pixel `(column, row)` and upper left/lower right `Complex64`,
        /// will return the corresponding point on complex plane.
        fn map_px_to_pt(
            bounds: (usize, usize),
            pixel: (usize, usize),
            ul: Complex64,
            lr: Complex64,
        ) -> Complex64 {
            let (w, h) = (lr.re - ul.re, ul.im - lr.im);

            Complex {
                re: ul.re + pixel.0 as f64 * w / bounds.0 as f64,
                im: ul.im - pixel.1 as f64 * h / bounds.1 as f64,
            }
        }

        #[test]
        fn test_map_px_to_pt() {
            assert_eq!(
                map_px_to_pt(
                    (100, 200),
                    (25, 175),
                    Complex { re: -1.0, im: 1.0 },
                    Complex { re: 1.0, im: -1.0 }
                ),
                Complex {
                    re: -0.5,
                    im: -0.75
                }
            );
        }

        /// Plots Mandelbrot Set into arr by calling escape_time on each
        pub fn render(pixels: &mut [u8], bounds: (usize, usize), ul: Complex64, lr: Complex64) {
            assert!(pixels.len() == bounds.0 * bounds.1);

            (0..bounds.1).for_each(|row|
            (0..bounds.0).for_each(|col|
            {
                let point = map_px_to_pt(bounds, (col, row), ul, lr);

                pixels[row * bounds.0 + col] = match iter_to_exit(point, 255){
                    Some(x) => 255 - x as u8,
                    None => 0,
                };
            }
            ));
        }

        /// writes a buffer of pixels to an image
        pub fn write(fname: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
            let outFile = File::create(fname)?;

            let ref mut w = BufWriter::new(outFile);

            let mut encoder = png::Encoder::new(w, bounds.0 as u32, bounds.1 as u32);

            encoder.set_color(png::ColorType::Grayscale);

            let mut imageWriter = encoder.write_header().unwrap();

            imageWriter.write_image_data(&pixels).unwrap();


            Ok(())
        }
    }
}
