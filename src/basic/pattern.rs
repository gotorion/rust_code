#![allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

mod test {
    #![allow(unused_imports)]
    use super::Direction;
    #[test]
    fn test_match_direction() {
        let d = Direction::Up;
        match d {
            Direction::Up => println!("Up"),
            Direction::Down => println!("Down"),
            Direction::Left => println!("Left"),
            Direction::Right => println!("Right"),
        }
    }

    #[test]
    fn test_if_let() {
        let _v = Some(3u8);
        if let Some(3) = _v {
            assert!(matches!(3, _v));
        }
    }
}
