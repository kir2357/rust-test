#![warn(dead_code)]

use std::mem;

#[derive(Debug)]
enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match *e {
        // 所有権ルールは値から `name` を受け取ることを許可しない。
        // しかし、可変参照から値を取り出すことはできない。置き換えない限りは:
        A { ref mut name } => B { name: mem::take(name) },
        B { ref mut name } => A { name: mem::take(name) },
        C => D,
        D => C
    }
}

fn swizzle_2(e: &mut MultiVariateEnum) {
    // 感覚的に実装してコンパイラを黙らした場合
    println!("test");
    use MultiVariateEnum::*;
    *e = match e {
        A { name:t } => B { name: t.to_string() },
        // ここでAのnameを参照し、to_string()メソッドでCopy可能にしてコピー
        // Copyトレイトが実装されているオブジェクトしかできない
        B { name:t } => A { name: t.to_string() },
        C => D,
        D => C
    }
}
#[test]
fn main () {
    let mut my_enum = MultiVariateEnum::A{name:"test".to_string()};
    println!("before");
    println!("{:?}",my_enum);

    swizzle_2(&mut my_enum);
    println!("after");
    println!("{:?}",my_enum);

} 


