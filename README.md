# fetch-yt-data-tools

![Rust](https://img.shields.io/badge/-Rust-6e412b.svg?logo=rust&style=plastic)

> [!NOTE]
> English version is [here](readme-en.md)!
<!-- >> [!NOTE]
> このリポジトリを使用した[video_downloader]()では、動画自体のダウンロードも可能です。 -->

> [!NOTE]
> このプログラムでは動画自体をダウンロードすることはできません。

## クイックスタート

```bash
cargo run
```

または、

```bash
cargo build --release
./target/release/fetch-yt-data-tools
```

## どんなプログラム?

[YouTube Data Api](https://developers.google.com/youtube/v3)を使用してYouTube上の動画情報(タイトルや動画idなど)を取得するツールです。

コンパイルや実行に`cargo`(rustのビルドシステム兼パッケージマネージャー)が必要です。

## 設定

設定は以下の3つの方法で指定できます。インデックスは優先順位です。

1. コマンドライン引数
2. 環境変数
3. 設定ファイル(toml)

### 設定の値について

- `LOG_LEVEL`: ログレベルを設定します
    - 値: `TRACE`,`DEBUG`,`INFO`,`WARN`,`ERROR`
    - 未指定の場合、ログは出力されません。ログ無しでも動作に影響はありません。
- `YOUTUBE_DATA_API_KEY`: APIキーです。セキュリティ上ログに出力されることはありません。

### コマンドライン引数

詳細は以下のコマンドで確認できます。

```bash
cargo run -- --help
```

出力内容

```txt
fetch video data using youtube api

Usage: fetch-yt-data-tools.exe [OPTIONS]

Options:
  -s, --settings-path <SETTINGS_PATH>
          path to configuration file [env: SETTINGS_PATH=] [default: ./settings.toml]
  -n, --no-use-settings-file
          no use settings file [env: NO_USE_SETTINGS_FILE=]
  -i, --input-api-key
          input api the in the program [env: INPUT_API_KEY=]
      --stdout-log-level <STDOUT_LOG_LEVEL>
          log level of standard output [env: STDOUT_LOG_LEVEL=] [possible values: trace, debug, info, warn, error]
      --file-log-level <FILE_LOG_LEVEL>
          log level of file output [env: FILE_LOG_LEVEL=] [possible values: trace, debug, info, warn, error]
  -o, --output-file-without-ext <OUTPUT_FILE_WITHOUT_EXT>
          path to output fetched data [env: OUTPUT_FILE_WITHOUT_EXT=]
      --output-file-ext <OUTPUT_FILE_EXT>
          output file extension [env: OUTPUT_FILE_EXT=] [possible values: json, yaml]
  -h, --help
          Print help
```

### 環境変数

コマンドライン引数で使用できるオプションは全て環境変数でも使用可能です。これに加え以下の変数も使用できます。

- `YOUTUBE_DATA_API_KEY`: APIキーを指定します(コマンドライン引数では使用不可)。

### 設定ファイル(toml)

設定ファイルはTOML形式を使用します。デフォルトでは`./settings.toml`が指定されていますが、以下の方法で変更可能です。

- コマンドライン引数: `--settings-path`オプションを使用
- 環境変数: `SETTINGA_PATH`を指定
- 設定ファイルを使用しない場合: `--no-use-settings-file`,`-n`を指定

設定ファイルの例

```toml
# ./settings.toml
[fetch_yt_data_tools]  # ! この記述は必須です。
youtube_data_api_key = "<key>"
stdout_log_level = "info"
file_log_level = "debug"
output_path_without_ext = "./out"
output_file_ext = "json"
```

>![INFO]
> 上記例のように`fetch_yt_data_tools`のセクション名は必須です。

指定されない値があっても問題なく動作します。

## 設定の入力

設定が十分に与えられなかった場合、プログラム実行時にプロンプトで入力を求められます。プロンプトの指示に従うことで設定フェーズを完了できます。

## URLの入力

設定の入力が完了するとURLの入力フェーズへ移行します。

- 不適切: 123ABCabc12
- 適切: https://www.youtube.com/watch?v=123ABCabc12

> [!IMPORTANT]
> 動画IDのみでなくURL全体を入力してください。再生リストのURLも使用可能です。

## 取得

入力されたURLを基にYouTubeの情報を取得します。この処理ではYouTube APIの`quota`が消費されます。

- quotaの詳細: [公式ドキュメント](https://developers.google.com/youtube/v3/determine_quota_cost)を参照してください。
- 消費するquotaを最小限に押さえるように設計しています。

## その他

アドバイスや修正案、問題点などございましたら、issue, PRにてご教授いただけますと幸いです。

また、実装等に関する質問も受け付けております。

## License

This project is licensed under the Apache License - see the [LICENSE](./LICENSE) file for details.
