fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("{:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangleをインスタンス化するときに一緒にareaを計算しちゃった方がいい気が
// するんだけど、どうやるんだろう
// Rectangleの定義の中に書けるんかな？
// -> 構造体にメソッドを持つ感じになりそう？
