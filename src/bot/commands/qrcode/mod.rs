use serde::{Deserialize, Serialize};
use serenity::all::{CommandInteraction, CommandOptionType, CreateAttachment, CreateCommandOption, CreateInteractionResponseMessage};
use serenity::builder::CreateCommand;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandQrCode {
    data: String,
}

pub async fn run(command_interaction: &CommandInteraction) -> CreateInteractionResponseMessage {
    let data_for_qrcode =
        command_interaction.data.options
            .iter().find(|opt| opt.name == "data")
            .iter()
            .map(|cmd| cmd.value.as_str())
            .flatten()
            .collect::<Vec<&str>>()
            .first().map(|x| *x)
            .unwrap_or("not found");

    let response = reqwest::Client::new()
        .post("http://192.168.1.19:9001/qrcode/generate-image")
        .json(&CommandQrCode {
            data: data_for_qrcode.to_string()
        })
        .send().await;

    match response {
        Ok(response) => {
            let bytes_vec = response
                .bytes().await.map(|bytes| bytes.to_vec())
                .unwrap_or(vec![]);

            let attachment = CreateAttachment::bytes(
                bytes_vec.as_slice(),
                "image.png".to_string(),
            );

            // Répondre à la commande avec l'image en pièce jointe
            CreateInteractionResponseMessage::new().content("qrcode généré").add_file(attachment)
        }
        Err(e) => {
            let message_err = format!("erreur lors du call au service de qrcode : {}", e.to_string());
            CreateInteractionResponseMessage::new().content(message_err)
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("qrcode")
        .description("Créer un qrcode de ce que vous voulez")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                String::from("data"),
                String::from("data a transformer en qrcode"),
            )
        )
}
