mod util{
    use std::str::FromStr;

    /// generic parse from str into type, with separator
    /// 
    /// example: 32x32 with separator `x` will return a pair of `(32, 32)`
    fn parse_pair<T: FromStr>(str: &str, sep: char) -> Option<(T, T)>{
        let idx = str.find(sep)?;

        if let (Ok(l), Ok(r)) = (T::from_str(&str[..idx]), T::from_str(&str[idx+1..])) {
            return Some((l, r));
        }
        None
    }
    #[test]
    fn test_parse_pair(){
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("32", ','), None);
        assert_eq!(parse_pair::<i32>(",32", ','), None);
        assert_eq!(parse_pair::<i32>("32,32", ','), Some((32, 32)));
        assert_eq!(parse_pair::<i32>("32,32abcd", ','), None);
        assert_eq!(parse_pair::<f32>("32.0x0.32abcd", 'x'), None);
        assert_eq!(parse_pair::<f32>("32.0x0.32", 'x'), Some((32.0, 0.32)));
    }

}