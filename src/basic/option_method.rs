mod test {

    #[test]
    fn test_map_option() {
        {
            let number = Some(10);
            let result = number.map(|x| x * 2);
            assert_eq!(20, result.unwrap());
        }

        {
            let none: Option<i32> = None;
            let result = none.map(|x| x * 2);
            assert_eq!(None, result);
        }
    }

    #[test]
    fn test_map_trans_type() {
        let text = Some("123");
        let number = text.map(|s| s.parse::<i32>().unwrap());
        assert_eq!(number.unwrap(), 123);
    }

    #[test]
    fn test_and_then() {
        let closure = |x: &str| -> Option<i32> {
            match x.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => None,
            }
        };
        let text = Some("32");
        let result_map = text.map(closure);
        let result_and_then = text.and_then(closure);
        assert_eq!(result_map.unwrap(), Some(32));
        assert_eq!(result_and_then.unwrap(), 32);
    }

    #[test]
    fn test_map_or() {
        let mut text = Some("32");
        let mut number = text.map_or(0, |s| s.parse::<i32>().unwrap());
        assert_eq!(number, 32);

        text = None;
        number = text.map_or(0, |s| s.parse::<i32>().unwrap());
        assert_eq!(number, 0);
    }

    #[test]
    fn test_map_or_else() {
        let mut text = Some("32");
        let mut number = text.map_or_else(|| 0, |s| s.parse::<i32>().unwrap());
        assert_eq!(number, 32);

        text = None;
        number = text.map_or_else(|| 0, |s| s.parse::<i32>().unwrap());
        assert_eq!(number, 0);
    }
}
