# youtube-data-api

![Rust](https://img.shields.io/badge/-Rust-6e412b.svg?logo=rust&style=plastic)

いつか書く

<!-- >[!NOTE]
> English version is [here]()!

>[!NOTE]
> このリポジトリは[video_downloader]()と一緒に使用することで効果を最大限に受けられます。

## クイックスタート

```rust
cargo run -n -o ./output.json
```

## どんなプログラム?

`YouTube Data Api`を使用して`Url`からYouTube上に投稿されている動画の情報を取得します。タイトルや動画idなどです。
`YouTube Data Api`の公式ドキュメントは[こちら](https://developers.google.com/youtube/v3?hl=ja)。

## 設定

設定は`コマンドライン引数`, `環境変数`, `設定ファイル(toml)`を用いて行います。
先に記述してある方が優先度が高いです。

- `LOG_LEVEL`: `TRACE(0)` から `ERROR(5)` までが使用可能です。文字列か数値で指定します。渡さない場合はログが出力されません。ログを出力しなくても動作には問題ありません。
- `output_file`: この値は上記のいずれかの方法で指定する必要があります。指定されないとプログラムは**終了**します。
- `YOUTUBE_DATA_API_KEY`: 値が指定されないとプログラム内で入力します。コンソールには出力されません。また、ログ出力されることもありません。

### コマンドライン引数

詳細は`-h` または `--help` を使用してください。

<details>

<summary>出力される内容</summary>

```txt
fetch `video_id` using youtube api

Usage: youtube_api.exe [OPTIONS] [YOUTUBE_DATA_API_KEY]

Arguments:
  [YOUTUBE_DATA_API_KEY]  the key on `YouTube data v3 api` [env: YOUTUBE_DATA_API_KEY=]

Options:
  -s, --settings-path <SETTINGS_PATH>
          path to configuration file [env: SETTINGS_PATH=] [default: ./settings.toml]
  -n, --no-use-settings-file
          no use settings file [env: NO_USE_SETTINGS_FILE=]
  -i, --input-api-key
          input api the in the program [env: INPUT_API_KEY=]
      --stdout-log-level <STDOUT_LOG_LEVEL>
          log level of standard output [env: STDOUT_LOG_LEVEL=]
      --file-log-level <FILE_LOG_LEVEL>
          log level of file output [env: FILE_LOG_LEVEL=]
  -o, --output-file <OUTPUT_FILE>
          path to output execution results [env: OUTPUT_FILE=]
  -h, --help
          Print help
```

</details>

### 環境変数

コマンドライン引数で使用できるオプションを環境変数で使用します。追加で`YOUTUBE_DATA_API_KEY`という環境変数も利用可能です。(このオプションはコマンドライン引数では指定できません。)

### 設定ファイル(toml)

設定ファイルは`toml`形式を使用します。パスはデフォルトで`./settings.toml`が指定されています。この値は上記の`コマンドライン引数`または`環境変数`で変更するか、`--no-use-settings-file (-n)`を使用して設定ファイルを使用しないように指定します。

例

```toml
# ./settings.toml
[youtube_api]  # ! この記述は必須です。
youtube_data_api_key = "apiのキー"
stdout_log_level = "<log_level>"
file_log_level = "<log_level>"
output_file = "./output.json"
```

>![INFO]
> 上記例のように[youtube_api]という記述は必須です。

これらの値は全てオプションです。一部の値が指定されなくても問題ありません。

## Url

プログラムを実行すると`Url`の入力フェーズへ移行します。

- 不適切: 123ABCabc12
- 適切: (前略).com/watch?v=123ABCabc12

このように`Id`のみの指定ではなく`Url`全体を指定してください。

また、再生リストの`Url`でも問題ありません。

## 出力形式
