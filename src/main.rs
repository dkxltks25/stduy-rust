#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// impl -> implementation 구현 블록
impl Rectangle {
    fn area(&self) -> u32 { // &self는 Reactangle의 구현체 내부에 있기에 Ractangle임을 알 수 있다.
        // 만약 변경하기를 원한다면 &self가 아닌 &mut self가 되될 것이다
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 45, width: 10 };
    let rect3 = Rectangle { length: 40, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect2));
    println!("rect is{:?}", Rectangle::square(4));
    println!("rect1 is{:?}", rect1);
}
