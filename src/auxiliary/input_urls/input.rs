use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::{self, Write},
};

use crate::url::Url;

/// urlを入力させる
///
/// 戻り値の長さが0の時もある
pub fn input() -> VecDeque<Url> {
    println!("Input urls. Type `help`, show help.");
    let mut urls: VecDeque<Url> = VecDeque::new();

    loop {
        let mut buffer = String::new();
        print!("  > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        if parse_input(buffer.trim(), &mut urls) {
            // parse_inputで重複削除する場合もあるが,必ず呼び出されるようここで呼ぶ
            remove_duplicates(&mut urls);
            break;
        }
    }
    urls
}

/// 戻り値:
/// - `true`: urlの入力を終了する
/// - `false`: urlの入力を終了しない
fn parse_input(s: &str, urls: &mut VecDeque<Url>) -> bool {
    let s_lowercase = s.to_lowercase();

    // Enterのみを押したとき
    if s.is_empty() {
        return false;

    // ヘルプメッセージを表示
    } else if s_lowercase == "help" {
        show_help_message();

    // ファイルからurlを取得
    } else if s.starts_with("f:") {
        read_from_file(s.trim_start_matches("f:"), urls);

    // ファイルからurlを取得
    } else if s.starts_with("F:") {
        read_from_file(s.trim_start_matches("F:"), urls);

    // 入力を終了
    } else if s_lowercase == "exit" {
        remove_duplicates(urls);
        return confirm_exit(urls);

    // 入力を強制的に終了
    } else if s_lowercase == "exit-f" {
        return true;

    // urlに入力をパース
    } else {
        match Url::new(s.to_string()) {
            Ok(url) => urls.push_back(url),
            Err(e) => println!("{}", e),
        }
    }
    false
}

fn show_help_message() {
    let message = r#"
# input urls
Please enter urls in stdin without abbreviation.
==== cmd ====
  - help            : show help message
  - exit            : exit url input(with confirm)
  - exit-f          : exit url input(without confirm)
  - f:<path/to/file>: read urls from file
    required csv file with url separated by commas
=============
"#;
    println!("{}", message.trim());
}

// ファイルからの読み込み関連 begin
fn read_from_file(path: &str, urls: &mut VecDeque<Url>) {
    match read_file_contents(path) {
        Ok(contents) => parse_read_file_contents(contents, urls),
        Err(e) => println!("{}", e),
    }
}

fn read_file_contents(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot open file:`{}`", e))
}

fn parse_read_file_contents(contents: String, urls: &mut VecDeque<Url>) {
    let mut added_urls_num: usize = 0;
    let mut invalid_urls_num: usize = 0;

    for c in contents
        .split_whitespace()
        .flat_map(|s| s.split(','))
        .skip_while(|s| s.is_empty())
    {
        match Url::new(c.to_string()) {
            Ok(url) => {
                urls.push_back(url);
                added_urls_num += 1;
            }
            Err(_) => invalid_urls_num += 1,
        }
    }

    if invalid_urls_num != 0 {
        println!(
            "{} url(s) are ignored due to invalid format.",
            invalid_urls_num
        );
    }
    println!("{} url(s) are added.", added_urls_num);
}
// ファイルからの読み込み関連 end

fn confirm_exit(urls: &mut VecDeque<Url>) -> bool {
    println!("Inputted urls are following:");
    for url in urls {
        println!(" > {}", url.build_url());
    }

    loop {
        print!("Exit input urls? [y/n] ");
        let mut buffer = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.to_lowercase().trim().to_string();

        if buffer == "y" {
            return true;
        } else if buffer == "n" {
            return false;
        } else {
            println!("invalid input:`{}`", buffer);
        }
    }
}

fn remove_duplicates(urls: &mut VecDeque<Url>) {
    let mut seen = HashSet::new();
    urls.retain(|url| seen.insert(url.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_urls_remove_duplicates() {
        let mut input_urls: VecDeque<Url> = vec![
            Url::v_inc_from_1(),
            Url::v_pl_inc_from_1(),
            Url::v_inc_from_1(),
            Url::v_inc_from_1(),
        ]
        .into();
        let expected_removed_urls: VecDeque<Url> =
            vec![Url::v_inc_from_1(), Url::v_pl_inc_from_1()].into();

        remove_duplicates(&mut input_urls);

        debug_assert_eq!(input_urls, expected_removed_urls);
    }

    #[test]
    fn test_input_urls_parse_read_file_contents_1() {
        let expected_urls: VecDeque<Url> =
            vec![Url::v_inc_from_1(), Url::v_pl_inc_from_1()].into();
        let contents = r#"
https://youtu.be/12345678901,https://www.youtube.com/watch?v=12345678901&list=1234567890123456789012345678901234
        "#
        .to_string();
        let mut urls: VecDeque<Url> = VecDeque::new();
        parse_read_file_contents(contents, &mut urls);
        assert_eq!(urls, expected_urls);
    }

    #[test]
    fn test_input_urls_parse_read_file_contents_2() {
        let expected_urls: VecDeque<Url> =
            vec![Url::v_inc_from_1(), Url::v_pl_inc_from_1()].into();
        let contents =
            format!(
                "{}\t {}\n\n\n\n\n",
                "https://youtu.be/12345678901",
                "https://www.youtube.com/watch?v=12345678901&list=1234567890123456789012345678901234"
            );

        let mut urls: VecDeque<Url> = VecDeque::new();
        parse_read_file_contents(contents, &mut urls);
        assert_eq!(urls, expected_urls);
    }
}
