# Rust検証用環境

## 実行について

testで動くようにしている

``cargo test -p メンバー名 -- --nocapture``

各メンバのmain.rsでテストするモジュールを取り込んでいる

## メンバー

### idioms

### design_patterns

### axum_url_shortener

[URL短縮サービスを勢いだけで実装してみる（Rust, axum）](https://zenn.dev/kyoheiu/articles/e0ef8454e5600f)

[kyoheiu/url-shortener-axum-sled](https://github.com/kyoheiu/url-shortener-axum-sled)

上記サイトのクローンを実装

Rustでのバックエンドの実現性検討

開発中のでバックにはRest Clientを用いる

[VS Code上でHTTPリクエストを送信し、VS Code上でレスポンスを確認できる「REST Client」拡張の紹介 - Qiita](https://qiita.com/toshi0607/items/c4440d3fbfa72eac840c)

⇒ VSnote:Rust_axum検討

## config_val

設定値の外部ファイルからの取得検討 ⇒ tomlからの設定値取得はしんどい

## logging_example

ロギングについての検検証

## dotenv_example

dotenvクレートを用いた設定値取得。⇒設定値のない時の処理がめんどい

[Rustのdotenvの使い方まとめ - Qiita](https://qiita.com/Yukimura127/items/c3f199bbdfb0fee34015)

## confy_example

confyクレートを用いた設定値取得

[Rust製の設定管理ライブラリconfyを試す - Qiita](https://qiita.com/Tadahiro_Yamamura/items/7c5637371e83752cd13a)

デフォルトの出力場所はWindows: {FOLDERID_RoamingAppData}\{app_name}

設定ファイル内にキーが不足するとエラー

## axum_url_shortener

### 受信Jsonのパース

[struct <-> String <-> Json - Qiita](https://qiita.com/rejasupotaro/items/17328120decd3e4425d6)

## TODO

- [Rustのパターンっぽいやつメモ](https://gist.github.com/qnighy/be99c2ece6f3f4b1248608a04e104b38)
- [Arc<Mutex<T>>という形はデザインパターン - Rustコトハジメ](https://rustforbeginners.hatenablog.com/entry/arc-mutex-design-pattern)
- [Introduction - Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [『Rust Design Patterns』を翻訳してみました（Idiom 編） - Qiita](https://qiita.com/Yappii_111/items/4ccc3a8461cdd4035651)
- [『Rust Design Patterns』を翻訳してみました（デザインパターン・アンチパターン編） - Qiita](https://qiita.com/Yappii_111/items/654717e6a6a980722189)
- [VisualStudioCodeで使えるRust言語用スニペットを書きました。 - Qiita](https://qiita.com/s4i/items/bd29911487c0ee4b296d)