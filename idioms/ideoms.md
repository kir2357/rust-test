# 2.idioms

[Idioms - Rust Design Patterns](https://rust-unofficial.github.io/patterns/idioms/index.html)

イディオムはコーディングするときに従うガイドラインです。

それらはコミュニティの社会的規範です。
 
それに従わないこともできますが、その場合はそうする理由があるべきです。

## 2.1 Use borrowed types for arguments

引数には所有型の借用よりも借用型を常に使うべき

- &String < &str
- &Vec<T> < &[T]
- &Box<T> < &T 


## 2.2 Concatenating strings with format!

format!マクロを使うと可読性が良くなる

```rust
fn say_hello(name: &str) -> String {
    // result 文字列を地道に作成することもできる。
    // let mut result = "Hello ".to_owned();//リテラルと
    // result.push_str(name);//非リテラルが混ざると特に
    // result.push('!');
    // result

    // しかし format! を使う方がよりよい。
    format!("Hello {}!", name)
}
```

可読性が良くなるがメモリアロケート的にはpushのほうが良くなるらしい？

## 2.3. Constructor

Rustには言語機能としてのコントラクタを持っていないのでnewメソッドを作製する慣例がある

## 2.4. The Default Trait

コントラクタは自作するので「すべてがnew()メソッドを持っている」ということを抽象化できていない

構造体フィールドの型にもdefaultで初期化できる。newの場合new()メソッド内でnewを呼び出す必要がある

```rust
#[derive(Default)]
pub struct Second {
    value: u64 // u64のdefault は0
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

fn main () {
    println!("second:{:?}",Second::default().value() );
}
```

## 2.5. Collections Are Smart Pointers

Deref トレイトを使って、データの所有や借用を提供するスマートポインタのようなコレクションを扱います。

Derefトレイトが実装されていると``*Vec<T>`` ⇒``*(Vec<T>.deref())`⇒``までコンパイラで変換する

処理時に``*(&[T])``⇒``T``となっている

```rust
use std::ops::Deref;

struct Vec<T> {
    // スマートポインタ
    data: RawVec<T>,
    //..
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    // コレクションにしている
    fn deref(&self) -> &[T] {
        //..
    }
}
```

### 動機

データを所持するデータ構造を実装するとき、データの参照を提供するとより柔軟な API にする

### 利点

ほとんどのメソッドは参照に対してのみ実装されているはずで、暗にそれは所有した値にも実装することができます。

クライアントにデータを借用するか所有権をとるかの選択肢を与えることができる

### 欠点

間接参照を使って利用できるメソッドやトレイトは境界チェックのとき考慮されない ⇒ コードの複雑化

## 2.6. Finalisation in Destructors

Rustはfinallyブロックに相当する機能がない

必要であればDropトレイトを実装することでデストラクタを作る

### 動機

関数が複数のreturnポイントを持っている場合、終了処理の実装が難しい

?等で関数がパニックした場合も終了処理が行われる

### 利点

デストラクタのコードはパニックやアーリーリターンなどに対応していて（ほぼ）常に実行されます。

### 欠点

- デストラクタが実行されることは保証されません。
  
  無限ループやクラッシュ

- ファイナライズ処理のためにオブジェクトと Drop の実装を必要とすることはボイラープレートとしては重い


## 2.7. mem::{take(_), replace(_)}