use fetch_yt_data_tools::{
    application::YouTubeService,
    auxiliary::{input_urls, settings_cfg::Settings},
    infrastructure::{fetch::ApiClient, output_to_file},
    util::{convert::separate_vec_deque_result, tracing::apply_tracing_settings},
};

// このクレート単体で実行することないので多少汚くて大丈夫
#[tokio::main]
async fn main() {
    let settings = Settings::load();
    let _tracing_settings = apply_tracing_settings(
        settings.get_stdout_log_level(),
        settings.get_file_log_level(),
    );

    println!("Finish input settings!");

    let api_impl = ApiClient::new(settings.get_api_key().as_string().into());
    let service = YouTubeService::new(api_impl);
    let urls = input_urls::input();

    if urls.is_empty() {
        println!("No urls inputted, exit this process.");
        return;
    }

    println!("Start fetching data");
    let res = service.using_urls(urls).await;
    let res = match res {
        Ok(v) => v,
        Err(e) => {
            println!("{}, so exit the program.", e);
            return;
        }
    };
    let (basic_data, invalid_urls) = separate_vec_deque_result(res);
    println!("Finish fetching data");

    let output = output_to_file::output_to_file(
        &serde_json::to_value(basic_data).unwrap(),
        settings.get_output_path_without_ext().into(),
        Some(settings.get_output_file_ext()),
    );

    if let Err(e) = output {
        println!("Failed to write result in file: `{}`", e);
        return;
    }

    if !invalid_urls.is_empty() {
        println!("Url that could not be found is following:");
        for url in invalid_urls {
            println!("  > {}", url.build_url());
        }
    }
}
