# rust-todoapp

## 概要

Rust 練習用の コマンドライン TODO アプリです

## 参考にしたサイト

[Microsoft の教材](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-command-line-program/)

## 動かし方

### タスク追加

```
$ cargo run add "study rust"
```

作成されたデータは、ルートディレクトリに `.rusty-journal.json` というファイル名で保存されます  
また、 `-j test.json` のように、保存するファイルを指定することもできます

### タスク確認

```
$ cargo run list
```

表示例
```
1: study rust                                         [2021-07-15 08:10]
2: buy milk and eggs                                  [2021-07-15 08:11]
```

### タスク削除

```
$ cargo run done 2
```
※ done の後に指定する番号は、タスク確認コマンドで表示された番号

