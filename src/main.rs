use std::io;
// 너비와 높이가 정수임을 표시, 여러가지 경우의 넓이를 구할 때 하나하나 다 구할 수 없으므로 모두에게 적용되는
// 높이와 너비를 struct 함수를 통해 받는다.
struct Object {
    width: u32,
    height: u32,
}

// Object에 포함되는 method들을 넣어주기 위해 적어준다. 
impl Object{
    
    //show함수에 있는 area를 지정해준 함수이다.
    fn area(&self) -> u32{
        self.width *self.height
    }
    //
    fn new(width: u32, height: u32) -> Object{
        Object{
            width,
            height,
        }
    }
    
    // show 함수를 통해 밑에 j의 넓이를 구할 때 더욱 편하게 구할 수 있게 되었다.
    fn show(&self){
        println!("{}X{} with area: {}", self.width, self.height, self.area());
    
    } 
}



fn main() {
    println!("Type two numbers: ");
    let mut w1 = String::new();
    io::stdin().read_line(&mut w1)
        .expect("Faild to read line");
    let w1:u32 = w1.trim().parse()
        .expect("Pleas write num");
    let mut h1 = String::new();
    io::stdin().read_line(&mut h1)
        .expect("Faild to read line");
    let h1:u32 = h1.trim().parse()
        .expect("Pleas write num");

    // struct를 활용해 width와 height를 선언해준다.
    let o = Object{
        width : 10,
        height : 15,
    };

    // 함수의 이름만 다르게 하여 width와 height을 간단하게 쓸 수 있다.
    let j = Object::new(30,12);

    let l = Object{
        width:w1,
        height:h1,
    };
    

    o.show();
    j.show();
    l.show();
}
