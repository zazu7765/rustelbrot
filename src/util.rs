mod util {
    use num_complex::Complex64;

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
        use num_complex::Complex64;

        /// Given image bounds `(width, height)`, pixel `(column, row)` and upper left/lower right `Complex64`,
        /// will return the corresponding point on complex plane.
        fn map_px_to_pt(
            bounds: (usize, usize),
            pixel: (usize, usize),
            ul: Complex64,
            lr: Complex64,
        ) -> Complex64 {
            todo!()
        }

        #[test]
        fn test_map_px_to_pt() {}

        /// Plots Mandelbrot Set into arr by calling escape_time on each 
        pub fn render(pixels: &mut [u8], bounds: (usize, usize), ul: Complex64, lr: Complex64) {
            todo!();
        }

        /// writes a buffer of pixels to an image
        fn write(fname: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>{
            todo!();
        }
    }
}
