#![allow(dead_code)]
fn create_tuple() -> (u32, u32, u32) {
    let t = (100, 200, 300);
    let (x, y, z) = t;
    (x, y, z)
}

fn visit_tuple_first_elem(t: (u32, u32, u32)) -> u32 {
    t.0
}

mod test {
    use super::*;
    #[test]
    fn test_visit_tuple_first_elem() {
        let t = (100, 200, 300);
        let x = visit_tuple_first_elem(t);
        assert_eq!(x, 100);
    }
    #[test]
    fn test_create_tuple() {
        let (x, y, z) = create_tuple();
        assert_eq!(x, 100);
        assert_eq!(y, 200);
        assert_eq!(z, 300);
    }
}
