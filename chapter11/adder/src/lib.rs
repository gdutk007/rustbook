
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle)-> bool {
        self.width > other.width && self.height > other.height
    }
}




pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        //let result = add(2, 2);
        assert_eq!(2+2, 4);
    }
    // #[test]
    // fn another(){
    //     panic!("Make this test fail.")
    // }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 2,
            height: 5,
        };
        let smaller = Rectangle {
            width: 1,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {
            width: 2,
            height: 5,
        };
        let smaller = Rectangle {
            width: 1,
            height: 4,
        };
        assert!(!smaller.can_hold(&larger));
    }
}
