fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

// 参照で返すことでエラーになることは分かったが、
// 参照を返したい場面って存在するのかな
