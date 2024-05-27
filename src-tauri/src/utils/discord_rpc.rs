use declarative_discord_rich_presence::activity::Activity;
use declarative_discord_rich_presence::activity::Assets;
use declarative_discord_rich_presence::activity::Timestamps;
use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
use tauri::State;

#[tauri::command]
pub fn set_rpc(
    client: State<'_, DeclarativeDiscordIpcClient>,
    details: &str,
    large_text: &str,
    small_text: &str,
    timestamp: i64,
) {
    if let Err(e) = client.set_activity(Activity::new()
        .details(details)
        .assets(Assets::new()
            .large_image("msm")
            .large_text(large_text)
            .small_text(small_text)
        )
        .timestamps(Timestamps::new().start(timestamp)),
    ) {
        eprintln!("Error setting activity: {:?}", e);
    }
}