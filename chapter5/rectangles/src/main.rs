
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    
    fn area(rectangle: &Rectangle)->u32{
        rectangle.width*rectangle.height
    }

    fn can_hold(&self, rect: &Rectangle)->bool{
        if self.width < rect.width {
            return false;
        }
        if self.height < rect.height {
            return false;
        }
        return true;
    }
}

// fn main() {
//     //println!("Hello, world!");
//     //let width = 30;
//     //let height = 50;
//     //let rectangle = (30,50);
//     let rectangle = Rectangle{
//         width: 30,
//         height: 50,
//     };

//     println!("The area of the rectangle is {} square pixels.", area(&rectangle));
//     println!("rectangle fields: {:?}",rectangle);
// }

// fn area(rectangle:(i32,i32))->i32{
//     rectangle.0*rectangle.1
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
