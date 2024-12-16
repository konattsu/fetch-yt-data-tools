# fetch-yt-data-tools

![Rust](https://img.shields.io/badge/-Rust-6e412b.svg?logo=rust&style=plastic)

> [!IMPORTANT]
> Created using translation. Inaccurate expressions may be used.
<!-- >> [!NOTE]
> このリポジトリを使用した[video_downloader]()では、動画自体のダウンロードも可能です。 -->

> [!NOTE]
> This program cannot download the video itself.

## Quick Start

```bash
cargo run
```

or

```bash
cargo build --release
./target/release/fetch-yt-data-tools
```

## What program?

A tool to fetch video information (title, video id, etc.) on YouTube using [YouTube Data Api](https://developers.google.com/youtube/v3).

You need `cargo` (RUST's build system and package manager) to compile and run it.

## Settings

Settings can be specified in three ways. The left index is the priority.

1. Command line arguments.
2. Environment variables.
3. A configuration file(toml).

### About the value of the setting

- `LOG_LEVEL`: Set the log level
    - Value: `TRACE`,`DEBUG`,`INFO`,`WARN`,`ERROR`.
    - If not specified, no log is output. No logging has no effect on the operation.
- `YOUTUBE_DATA_API_KEY`: API key. It is not output to the log for security reasons.

### Command line arguments

Details can be found in the following commands.

```bash
cargo run -- --help
```

Output contents

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

### Environment variables

All options that can be used in command line arguments can also be used in environment variables. In addition, the following variables can be used:

- `YOUTUBE_DATA_API_KEY`: Specifies a API key (cannot be used in command line arguments).

### A configuration file(toml)

The configuration file uses the TOML format. By default, `. /settings.toml` is specified, but can be changed in the following way:

- Command line arguments: use `--settings-path` option.
- Environment variables: `SETTINGA_PATH`.
- If you do not use configuration file: use `--no-use-settings-file`,`-n` as command line arguments.

Example of configuration file.

```toml
# ./settings.toml
[fetch_yt_data_tools]  # ! This statement is required.
youtube_data_api_key = "<key>"
stdout_log_level = "info"
file_log_level = "debug"
output_path_without_ext = "./out"
output_file_ext = "json"
```

>![INFO]
> The section name `fetch_yt_data_tools` is required, as in the example above.

It works fine with unspecified values.

## Enter settings

If sufficient settings were not given, you will be prompted for input at the prompt when the program is run. You can complete the configuration phase by following the instructions at the prompt.

## Enter URLs

Once the settings have been entered, the system moves to the URLs entry phase.

- Inappropriate: 123ABCabc12
- Appropriate: https://www.youtube.com/watch?v=123ABCabc12

> [!IMPORTANT]
> Please enter the entire URL, not just the video ID. You can also use the URL of a playlist.

## fetch data

Fetching data from YouTube based on the entered URL. This process consumes the `quota` of the YouTube API.

- For more information on `quota`: see [official documentation](https://developers.google.com/youtube/v3/determine_quota_cost).
- Designed to minimize quota consumption.

## License

See [this page](README.md#license).
