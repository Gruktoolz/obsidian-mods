fn hook(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::strangely_charming_commands::{self as commands, Command};
    let Some(parsed) = commands::parse(command) else {
        return;
    };
    *ran = true;
    let output = match parsed {
        Ok(Command::SpawnName(name)) => Some(commands::summon(&format!("{{name:\"{name}\"}}"))),
        Ok(Command::SpawnTexture { hash, slim }) => {
            let mut skin =
                serde_json::json!({"url":format!("https://textures.minecraft.net/texture/{hash}")});
            if slim {
                skin["metadata"] = serde_json::json!({"model":"slim"});
            }
            let value = commands::base64(
                serde_json::json!({"textures":{"SKIN":skin}})
                    .to_string()
                    .as_bytes(),
            );
            Some(commands::summon(&format!(
                "{{properties:[{{name:\"textures\",value:\"{value}\"}}]}}"
            )))
        }
        Ok(Command::Local { file, slim }) => {
            match horizon_client_render::strangely_charming_skin::load_local(&file) {
                Ok(hash) => {
                    let mut skin = serde_json::json!({"url":format!("https://textures.minecraft.net/texture/{hash}")});
                    if slim {
                        skin["metadata"] = serde_json::json!({"model":"slim"});
                    }
                    let value = commands::base64(
                        serde_json::json!({"textures":{"SKIN":skin}})
                            .to_string()
                            .as_bytes(),
                    );
                    Some(commands::summon(&format!(
                        "{{properties:[{{name:\"textures\",value:\"{value}\"}}]}}"
                    )))
                }
                Err(error) => {
                    game.sc_notice(error);
                    None
                }
            }
        }
        Ok(Command::Remove) => Some(commands::REMOVE.to_owned()),
        Ok(Command::Help) => {
            game.sc_notice("Strangely Charming: /strangely_charming dummy <username> | dummy texture <64-digit hash> [slim] | dummy file \"<PNG path>\" [slim] | dummy remove. Requires summon permission. Remove clears only tagged dummies within 32 blocks.");
            None
        }
        Err(message) => {
            game.sc_notice(message);
            None
        }
    };
    if let Some(line) = output {
        if let Err(error) = game.native_send_chat(&line) {
            game.sc_notice(format!(
                "Strangely Charming: {error}"
            ));
        }
    }
}
