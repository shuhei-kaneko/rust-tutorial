fn main(){
    print_label_measurement(5, 'h');
}

fn another_function(x: i32){
    println!("The value of x is: {}", x);
}

fn print_label_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

// NOTE: 文と式
// 文: なんらかの動作をして値を返さない命令(セミコロンを付ける)
// 式: 結果値に評価されます(セミコロンを付けない)
