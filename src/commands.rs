use poise::serenity_prelude as serenity;

use crate::{
    mappings::{ENG_TO_MIL, MIL_TO_ENG},
    Context, Error,
};
use log::info;

/// Generates a log of the translation.
async fn log_translation(text: &str, words: &str, ctx: Context<'_>) -> String {
    // Log the translation itself alongside the author.
    let mut log = format!(
        "Translation of '{}' to '{}'. Requested by '{}' with ID '{}'.",
        &text,
        &words,
        ctx.author().name,
        ctx.author().id.0
    );
    // Log the guild that the translation occurred in, if possible.
    if let Some(guild) = ctx.partial_guild().await {
        let name = guild.name;
        let id = guild.id.0;
        let addendum = format!(" In server '{}' with ID '{}'.", name, id);
        log += &addendum;
    }
    log
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            // extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    crate::teardown(
        ctx.framework().shard_manager,
        format!(
            "User '{}' with ID '{}' used the '{}' command.",
            ctx.author().name,
            ctx.author().id.0,
            ctx.command().name
        )
        .as_str(),
    )
    .await;
    Ok(())
}

/// Function that actually translates from NATO military phonetic alphabet to english
async fn translate_inner(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        .filter_map(|mil| MIL_TO_ENG.get(mil))
        .map(|eng| eng.to_owned())
        .collect::<String>()
}

/// Function that actually translates from English to NATO military phonetic alphabet
async fn reverse_translate_inner(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        // get first letter in word and unwrap
        .filter_map(|eng| ENG_TO_MIL.get(eng.get(0..1).unwrap()))
        .map(|mil| mil.to_owned())
        .map(|mil| format!("{} ", mil))
        .collect::<String>()
        .trim_end()
        .to_owned()
}

/// Translate NATO Military Phonetic Alphabet to English.
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn translate(
    ctx: Context<'_>,
    #[rest]
    #[description = "Text to translate"]
    text: String,
) -> Result<(), Error> {
    // Do the translation
    let words = translate_inner(&text).await;
    // Send the response
    ctx.say(&words).await?;
    // Log the translation to the translation logger
    let log = log_translation(&text, &words, ctx).await;
    info!(target: "translation-logger", "{}", &log);
    Ok(())
}

/// Translate English to NATO Military Phonetic Alphabet.
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn translate_english(
    ctx: Context<'_>,
    #[rest]
    #[description = "Text to translate from NATO military phonetic alphabet to english."]
    text: String,
) -> Result<(), Error> {
    // Do the translation
    let words = reverse_translate_inner(&text).await;
    // Send the response
    ctx.say(&words).await?;
    // Log the translation to the translation logger
    let log = log_translation(&text, &words, ctx).await;
    info!(target: "translation-logger", "{}", &log);
    Ok(())
}

/// Translate NATO Military Phonetic Alphabet to English.
#[poise::command(
    context_menu_command = "Translate NATO MPA to English",
    rename = "Translate NATO MPA to English"
)]
pub async fn translate_context_menu(
    ctx: Context<'_>,
    #[description = "Message to translate"] msg: serenity::Message,
) -> Result<(), Error> {
    // Do the translation
    let words = translate_inner(&msg.content).await;
    // Send the response
    ctx.say(&words).await?;
    // Log the translation to the translation logger
    let log = log_translation(&msg.content, &words, ctx).await;
    info!(target: "translation-logger", "{}", &log);
    Ok(())
}

/// Translate English to NATO Military Phonetic Alphabet.
#[poise::command(
    context_menu_command = "Translate English to NATO MPA",
    rename = "Translate English to NATO MPA"
)]
pub async fn translate_english_context_menu(
    ctx: Context<'_>,
    #[description = "Message to translate"] msg: serenity::Message,
) -> Result<(), Error> {
    // Do the translation
    let words = reverse_translate_inner(&msg.content).await;
    // Send the response
    ctx.say(&words).await?;
    // Log the translation to the translation logger
    let log = log_translation(&msg.content, &words, ctx).await;
    info!(target: "translation-logger", "{}", &log);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_translate() {
        let test_input = "India  Mike  Oscar Victor Echo Romeo  Hotel Echo Romeo Echo  Sierra Tango Romeo Oscar Kilo India November Golf Mike Yankee Delta India Charlie Kilo  India  Golf Oscar Tango Lima Oscar Tango India Oscar November Oscar November Mike Yankee Delta India Charlie Kilo Romeo India Golf Hotel Tango November Oscar Whiskey  India Mike  Juliett Uniform Sierra Tango  Sierra Tango Romeo Oscar Kilo India November Golf Mike Yankee Sierra Hotel India Tango India  Mike Hotel Oscar Romeo November Yankee Alpha Sierra Foxtrot Uniform Charlie Kilo Mike Alpha November India Mike Alpha  Foxtrot Romeo Echo Alpha Kilo Mike Alpha November Lima India Kilo Echo";
        let correct_output = "IMOVERHERESTROKINGMYDICKIGOTLOTIONONMYDICKRIGHTNOWIMJUSTSTROKINGMYSHITIMHORNYASFUCKMANIMAFREAKMANLIKE";
        assert_eq!(&translate_inner(test_input).await, correct_output);
    }

    #[test]
    async fn test_reverse_translate() {
        let test_input = "Hello my NAme iS bob.";
        let correct_output = "Hotel Mike November India Bravo";
        assert_eq!(&reverse_translate_inner(test_input).await, correct_output);
    }
}
