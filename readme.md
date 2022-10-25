# Rust検証用環境

## 実行について

testで動くようにしている

``cargo test -p メンバー名 -- --nocapture``

各メンバのmain.rsでテストするモジュールを取り込んでいる

## TODO

- [Rustのパターンっぽいやつメモ](https://gist.github.com/qnighy/be99c2ece6f3f4b1248608a04e104b38)
- [Arc<Mutex<T>>という形はデザインパターン - Rustコトハジメ](https://rustforbeginners.hatenablog.com/entry/arc-mutex-design-pattern)
- [Introduction - Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [『Rust Design Patterns』を翻訳してみました（Idiom 編） - Qiita](https://qiita.com/Yappii_111/items/4ccc3a8461cdd4035651)
- [『Rust Design Patterns』を翻訳してみました（デザインパターン・アンチパターン編） - Qiita](https://qiita.com/Yappii_111/items/654717e6a6a980722189)
- [VisualStudioCodeで使えるRust言語用スニペットを書きました。 - Qiita](https://qiita.com/s4i/items/bd29911487c0ee4b296d)