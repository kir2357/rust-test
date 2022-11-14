use std::io::{BufReader, Read, BufWriter, Write};
use std::fs;
use std::fs::{File, DirBuilder};
use std::path::Path;
use std::env;
use toml::Value;
use once_cell::sync::Lazy;
// [lazy_static はもう古い！？ once_cell を使おう](https://zenn.dev/frozenlib/articles/lazy_static_to_once_cell)

pub static CONFIG: Lazy<Value> = Lazy::new(|| load_config());

const DEFAULT_CONFIG: &'static str = r#"
[csv.output]
file_path = "{CUR}\\data\\output_%Y%m%d_%H%M%S.csv"
char_code = "shift-jis"

[calc]
value1 = 10
value2 = 20
"#;

// toml形式の設定ファイルを読み込む
fn load_config() -> Value {

    // 当プログラムのディレクトリ
    let cur_path_str = env::current_exe().unwrap().clone();
    let cur_path = Path::new(&cur_path_str);
    let cur_dir = cur_path.parent().unwrap().display();

    // 当プログラムのディレクトリ配下に存在する該当拡張子のファイルパスを取得
    let file_paths = get_path(Vec::new(), &cur_dir.to_string(), "toml");

    // toml形式変換前の設定文字列
    let mut conf_toml_str = String::new();
    // 全ファイルをテキストで読み込み
    for path in file_paths.iter() {
        conf_toml_str = format!("{}{}", conf_toml_str, get_text_file(Path::new(&path), "toml"));
    }

    // 設定を1件も取得できていなければ
    if conf_toml_str.len() < 1 {
        // 設定ファイルを保存するディレクトリを生成
        let path_str = format!("{}\\config\\config.toml", &cur_dir);
        let path = Path::new(&path_str);
        let dir = path.parent().unwrap();
        DirBuilder::new().recursive(true).create(&dir).unwrap();

        // 設定ファイルのパスを書き込みモードで開く。これは`io::Result<File>`を返す。
        let file = match File::create(&path) {
            Err(e) => panic!("couldn't create {}: {}", path_str, &e.to_string()),
            Ok(file) => file,
        };

        // バッファリングされたストリーム出力とする
        let mut f = BufWriter::new(file);

        // 設定ファイルへ記述
        conf_toml_str = DEFAULT_CONFIG.to_string();
        match f.write_all(conf_toml_str.as_bytes()) {
            Err(e) => panic!("couldn't write {}: {}", path_str, &e.to_string()),
            Ok(_) => println!("{} writes :{}\n", path_str, conf_toml_str),
        }
    }

    // 文字列内に「{CUR}」が存在すれば、当プログラムが存在するディレクトリとみなして、カレントディレクトリに置換
    conf_toml_str = conf_toml_str.replace("{CUR}", &format!("{}", &cur_dir));
    // 設定をtoml形式に変換して返す
    conf_toml_str.parse::<Value>().expect(&format!("couldn't parse config file to toml format.{}", &conf_toml_str))
}

fn get_path (mut files: Vec<String>, target:&str, path_ext:&'static str) -> Vec<String> {
    for file_path in fs::read_dir(target).unwrap() {
        let file_path = file_path.unwrap().path();
        if file_path.is_dir() {
            files = get_path(files, &file_path.display().to_string(), path_ext);
        } else {
            match file_path.extension() {
                _path_ext => files.push(file_path.display().to_string()),
            }
        }
    }
    files
}

fn get_text_file (path: &Path, extension: &'static str) -> String {
    let display = path.display();
    match path.extension() {
        None => return "".to_string(),
        Some(ext) => {
            if ext != extension {
                return "".to_string();
            }
        },
    };

    // pathを読み込み専用モードで開く
    let f = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", display, &e.to_string()),
        Ok(f) => f,
    };

    //  バッファリングされたストリーム入力とする
    let mut br = BufReader::new(f);

    // ファイルの中身を文字列に読み込み
    let mut conf_toml_str = String::new();
    match br.read_to_string(&mut conf_toml_str) {
        Err(e) => panic!("couldn't read {}: {}", display, &e.to_string()),
        // Ok(_) => println!("{} contains:\n{}", display, conf_toml_str),
        Ok(_) => {}
    }
    conf_toml_str
}