# 2.idioms

[Idioms - Rust Design Patterns](https://rust-unofficial.github.io/patterns/idioms/index.html)

[『Rust Design Patterns』を翻訳してみました（Idiom 編） - Qiita](https://qiita.com/Yappii_111/items/4ccc3a8461cdd4035651#memtake_-replace_-%E3%81%A7%E5%A4%89%E6%9B%B4%E3%81%95%E3%82%8C%E3%81%9F-enum-%E3%81%A7%E6%89%80%E6%9C%89%E3%81%97%E3%81%9F%E5%80%A4%E3%82%92%E4%BF%9D%E6%8C%81%E3%81%99%E3%82%8B)

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

借用チェッカーを満足させるためクローンするアンチパターへの解決方法

- クローンするとメモリ消費が大きくなる
- 借用チェックを行うRust特有の状態
- GCがある言語はデフォルトで参照で受け取る
- Cのような低レベルな言語ではポインタでか行けるする

mem::takeは要素（今回の場合はnameのString）をデフォルトで生成してあたいを書き込む

Defaultトレイトが実装していないオブジェクトを書き換えるには``men::replace(before,Object::new())``を使う

## 2.8. On-Stack Dynamic Dispatch

[rust 動的ディスパッチと静的ディスパッチの調査 - Qiita](https://qiita.com/Kashiwara/items/6b895ffdd50d7a8dee82)

動的ディスパッチ：実行時に型が決定されるコード

```rust
use std::io;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "-";

    // `readable` よりも長く存在しないといけないため、最初に宣言する :
    let (mut stdin_read, mut file_read);

    // 動的ディスパッチを得るために型を割り当てる必要がある。
    let readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };

    // ここで `readable` から読み込む

    Ok(())
}
```

stdin は Stdin 型であり、file は File 型であり、readable は &mut dyn Read 型

⇒ Stdin も File もReadトレイトを持っている。

readableより先にstdinやfileがドロップされては困るので先に宣言している

Boxベースで記述したほうがシンプルでありデメリットもないと感じる。

ヒープ領域に展開する量がちがう？動的ディスパッチだから変わらん気がするが

```rust
use std::io;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "-";

    // 動的ディスパッチのために依然として型を記述する必要がある。
    let readable: Box<dyn io::Read> = if arg == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(arg)?)
    };
    // ここで `readable` から読み込む。

    Ok(())
}
```

## 2.9. Foreign function interface (FFI)

### Idiomatic Errors

Cなどの言語はエラーをリターンコードで返す。変換して伝播させる必要がる

RustのAPIの機能を損ないうことなくエラーにアクセスしたい

ただし記述量が多く、一部の型は簡単に変換できない

FFIからのデータRustで処理した時に発生したエラーをFFIに伝播させる方法？

```rust
// Flat Enums
enum DatabaseError {
    IsReadOnly = 1, // user attempted a write operation
    IOError = 2, // user should read the C errno() for what it was
    FileCorrupted = 3, // user should run a repair tool to recover it
}

impl From<DatabaseError> for libc::c_int {
    // cのintからエラーコードを取り出す
    fn from(e: DatabaseError) -> libc::c_int {
        (e as i8).into()
        // Intoトレイトのinto()メソッド
        // Fromトレイトが実装されてるオブジェクト（この場合はlibc::c_int）に
        // DatabaseErrorをi8として代入スしている
    }
}

// Structured Enums
pub mod errors {
    enum DatabaseError {
        IsReadOnly,
        IOError(std::io::Error),
        FileCorrupted(String), // message describing the issue
    }

    impl From<DatabaseError> for libc::c_int {
        fn from(e: DatabaseError) -> libc::c_int {
            match e {
                DatabaseError::IsReadOnly => 1,
                DatabaseError::IOError(_) => 2,
                DatabaseError::FileCorrupted(_) => 3,
            }
        }
    }
}

pub mod c_api {
    use super::errors::DatabaseError;

    #[no_mangle]
    pub extern "C" fn db_error_description(
        e: *const DatabaseError
        ) -> *mut libc::c_char {

        let error: &DatabaseError = unsafe {
            // SAFETY: pointer lifetime is greater than the current stack frame
            &*e
        };

        let error_str: String = match error {
            DatabaseError::IsReadOnly => {
                format!("cannot write to read-only database");
            }
            DatabaseError::IOError(e) => {
                format!("I/O Error: {}", e);
            }
            DatabaseError::FileCorrupted(s) => {
                format!("File corrupted, run repair: {}", &s);
            }
        };

        let c_error = unsafe {
            // SAFETY: copying error_str to an allocated buffer with a NUL
            // character at the end
            let mut malloc: *mut u8 = libc::malloc(error_str.len() + 1) as *mut _;

            if malloc.is_null() {
                return std::ptr::null_mut();
            }

            let src = error_str.as_bytes().as_ptr();

            std::ptr::copy_nonoverlapping(src, malloc, error_str.len());

            std::ptr::write(malloc.add(error_str.len()), 0);

            malloc as *mut libc::c_char
        };

        c_error
    }
}

// Custom Error Types

struct ParseError {
    expected: char,
    line: u32,
    ch: u16
}

impl ParseError { /* ... */ }

/* Create a second version which is exposed as a C structure */
#[repr(C)]
pub struct parse_error {
    pub expected: libc::c_char,
    pub line: u32,
    pub ch: u16
}

impl From<ParseError> for parse_error {
    fn from(e: ParseError) -> parse_error {
        let ParseError { expected, line, ch } = e;
        parse_error { expected, line, ch }
    }
}
```

### Accepting Strings

ポインタを介してCの文字列を受け入れる場合、２つの原則に従う必要がある

1. Cの文字列を直接コピーするのではなく「借用」したままにする
2. Cの文字列からRust文字列への変換は、複雑さとunsafeコードを最小限にする

Cの文字列とRustの文字列は挙動が異なる

- Rustの文字列は長さのデータを持っているが、Cはnull-terminated('\0')している
- Rustの文字列はUTF-8である必要があるが、Cは non-zero byte 以外(null以外?)のデータを含むことができる
- Cの文字列はunsafeなポインター操作でアクセス・操作される

Rustの標準ライブラリにはCStringおよび＆CStrが付属ししている（RustのString、&strに相当）

これらを使うことでunsafeを回避することができる

&CStr 型を使用すると、借用したデータを操作可能。⇒Rust と C の間で文字列を渡す操作はゼロコスト

```rust
pub mod unsafe_module {
    #[no_mangle]
    pub unsafe extern "C" fn mylib_log(
        msg: *const libc::c_char,// 文字列の先頭ポインタ？
        level: libc::c_int
    ) {
        let level: crate::LogLevel = match level { /* ... */ };

        // 追跡されてない生ポインタが追跡されている参照になる
        let msg_str: &str = match std::ffi::CStr::from_ptr(msg).to_str() {
            Ok(s) => s,
            Err(e) => {
                crate::log_error("FFI string conversion failed");
                return;
            }
        };

        crate::log(msg_str, level);
    }
}
```

追跡の問題とは別に文字列がUTF-8でないことによるパニック・文字列末尾の問題や変換時のクラッシュの問題はある

### Passing Strings

文字列をFFIに渡す場合の4つの原則

1. 文字列のライフタイムを可能な限り長くする
2. unsafeブロックを最小限にする
3. Cコード内で文字列データを変更する場合は、CString の代わりに Vec を使用
4. 外部関数APIで必要とされない限り、文字列の所有権は呼び出し先に譲渡しない


ベスト プラクティスは安全でないコードを最小限に抑えるような方法で CString を使用

ただし、二次的な注意点は、ライフタイムを最大化する必要がある

```rust
pub mod unsafe_module {

    extern "C" {
        fn seterr(message: *const libc::c_char);
        // VecのポインタとサイズをFFIに渡して記入してもらう
        fn geterr(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    }

    fn report_error_to_ffi<S: Into<String>>(
        err: S
    ) -> Result<(), std::ffi::NulError>{
        let c_err = std::ffi::CString::new(err.into())?;

        unsafe {
            // SAFETY: calling an FFI whose documentation says the pointer is
            // const, so no modification should occur
            seterr(c_err.as_ptr());
        }

        Ok(())
        // The lifetime of c_err continues until here
    }

    fn get_error_from_ffi() -> Result<String, std::ffi::IntoStringError> {
        // からのVecを作成
        let mut buffer = vec![0u8; 1024];
        unsafe {
            // SAFETY: calling an FFI whose documentation implies
            // that the input need only live as long as the call
            let written: usize = geterr(buffer.as_mut_ptr(), 1023).into();
            // FFI関数を読んでいる間、bufferは操作される
            buffer.truncate(written + 1);
        }
        // buffurをCStringに変換することでRustコードないで扱えるようにする
        std::ffi::CString::new(buffer).unwrap().into_string()
    }
}

```

よくある間違い

```rust
pub mod unsafe_module {
    fn report_error_NG<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
        unsafe {
            seterr(std::ffi::CString::new(err.into())?.as_ptr());
        }
        Ok(())
    }

    fn report_error_OK<S: Into<String>>(
        err: S
    ) -> Result<(), std::ffi::NulError>{
        let c_err = std::ffi::CString::new(err.into())?;
        unsafe {
            seterr(c_err.as_ptr());
        }
        Ok(())
    }
}
```

as_ptr()メソッド（ポインタの作製）では CString のライフタイムの延長が行われないので、setter()処理内でだリングポインタになってしまう。

CStringのライフタイム終了後、setter()に値が渡される

ゼロの1kVecの初期化(``let mut buffer = vec![0u8; 1024]``)が遅いという疑問

マクロの最適化でOSの機能並に高速に（これ以上早い代替案は困難）

## 2.10. Iterating over an Option

Option型はIteratorトレイトを実装している

### extend()メソッドの引数として使用できる

```rust
let turing = Some("Turing");
let mut logicians = vec!["Curry", "Kleene", "Markov"];

logicians.extend(turing);

// 以下と等しい
if let Some(turing_inner) = turing {
    logicians.push(turing_inner);
}
```

### chain()メソッド

```rust
let turing = Some("Turing");
let logicians = vec!["Curry", "Kleene", "Markov"];

for logician in logicians.iter().chain(turing.iter()) {
    println!("{} is a logician", logician);
}
```

永続的に追加するか一時的に追加するかの違い

## 2.11. Pass Variables to Closure

クロージャーはデフォルトで与えられた変数を

- 借用によってキャプチャする
- move クロージャーを使って環境全体をmoveする

しかし、いくつかの変数だけをクロージャに移動させたり、いくつかのデータのコピーを与えたり、参照で渡したり、他の変換を行いたい場合がよくあります

⇒　別スコープで変数のリバイディングする

```rust
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);

let num2_cloned = num2.clone();
let num3_borrowed = num3.as_ref();
let closure = move || {
    *num1 + *num2_cloned + *num3_borrowed;
};
```

上を下にする

```rust
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);
let closure = {
    // `num1` is moved
    let num2 = num2.clone();  // `num2` is cloned
    let num3 = num3.as_ref();  // `num3` is borrowed
    move || {
        *num1 + *num2 + *num3;
    }
};
```

### 利点

コピーされたデータはクロージャの定義と一緒にグループ化されているので、目的がより明確になり、クロージャで消費されなくてもすぐにdropされている

クロージャーでは周囲のコードと同じ変数名を使用している

### 欠点

ネストが一段深くなる

## 2.12. Privacy For Extensibility

後方互換性を損なうことなく public フィールドを public 構造体に追加したり、新しいバリアントを列挙型に追加する手段

- #[non_exhaustive] 属性をstructにつける
- プライベートフィールドを使う

プライベートフィールドでよさげ

### 議論

クライアントがパターンを使用して構造体を作成している場合、ライブラリ側でフィールドを追加してしまうとクライアントのプログラムが動かなくなる

⇒　クライアント側でパターン記述ないで..を使用する

```rust
mod a {
    // public な構造体
    pub struct S {
        pub foo: i32,
        // プライベートフィールド
        bar: i32,
    }
}

fn main(s: a::S) {
    // S::bar はプライベートなので、ここでは名前をつけることができずパターン内で `..` を使わなければならない
    let a::S { foo: _, ..} = s;
}

```

欠点として、不要なフィールドを構造体内に追加する必要があかもしれないこと

実行時にオーバーヘッドが内容に()型を使用し、フィールド名の前に_を追加することで未使用のフィールドの警告回避できる

もし Rust が列挙型のプライベート変数を許可しているならば、列挙型にバリアントを追加して下位互換性を持たせるために同じトリックを使うことができます。ここで問題となるのは、網羅的なマッチ式です。プライベート変数はクライアントに _ ワイルドカードパターンを強制します。代わりにこれを実装する一般的な方法は #[non_exhaustive] 属性を使用することです。

⇒よく意味が分からん。パターンを強制とは

## 2.13. Easy doc initialization

ドキュメントのルールが分からないので何を解決したいのか分からない

ドキュメント中のコードをテストする方法がある？

## 2.14. Temporary mutability

変数の処理を行うために一時的にmutableでかこう、以降immutableにしたい場合

```rust
// ネストで実現
let data = {
    let mut data = get_vec();
    data.sort();
    data
};

// ここでは `data` はイミュータブル

// リバイディングで実現
let mut data = get_vec();
data.sort();
let data = data;

// ここでは `data` はイミュータブル
```

