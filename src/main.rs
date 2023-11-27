// 너비와 높이가 정수임을 표시
struct Object {
    width: u32,
    height: u32,
}

// 
impl Object{
    fn area(&self) -> u32{
        self.width *self.height
    }

    fn new(width: u32, height: u32) -> Object{
        Object{
            width,
            height,
        }    
    }
}
impl Object{
    fn show(&self){
        println!("{}X{} with area: {}", self.width, self.height, self.area());

    }
}

fn main() {
    let o = Object{
        width : 10,
        height : 15,
    };

    let j = Object{
        width: 13,
        height: 11,
    };

    o.show();
    j.show();
}
