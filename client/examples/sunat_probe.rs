use std::env;

use post_bunny_panel::domain::customer::LoginType;
use post_bunny_panel::preview;
use post_bunny_panel::sunat::{self, SunatCredentials};

fn main() {
    let dni = env::var("SUNAT_DNI").expect("SUNAT_DNI");
    let password = env::var("SUNAT_PASSWORD").expect("SUNAT_PASSWORD");
    let credentials = SunatCredentials {
        login_type: LoginType::Dni,
        dni: Some(dni),
        ruc: None,
        sol_user: None,
        password,
    };

    let jar_path = std::env::temp_dir().join(format!(
        "post-bunny-sunat-probe-{}.cookies",
        std::process::id()
    ));
    let outcome = sunat::sync_customer(0, Some(&credentials), None, jar_path);
    if let Some(error) = &outcome.error {
        eprintln!("SYNC ERROR: {error}");
        std::process::exit(1);
    }
    println!(
        "SYNC OK ruc={} name={:?} messages={}",
        outcome.ruc,
        outcome.display_name,
        outcome.messages.len()
    );
    for message in outcome.messages.iter().take(10) {
        println!(
            "  {} read={} kind={:?} attachments={} ctx={} urg={} label={:?} :: {}",
            message.cod_mensaje,
            message.read,
            message.kind,
            message.attachment_count,
            message.context_tipo,
            message.urgent,
            message.label.as_ref().map(|label| label.name.as_str()),
            message.subject
        );
    }
    if let Some(session) = &outcome.session {
        let reused = sunat::sync_customer(
            0,
            None,
            Some(session.clone()),
            std::env::temp_dir().join(format!(
                "post-bunny-sunat-probe-{}.cookies",
                std::process::id()
            )),
        );
        if let Some(error) = &reused.error {
            eprintln!("REUSE ERROR: {error}");
        } else {
            println!("REUSE OK messages={}", reused.messages.len());
        }

        for message in outcome.messages.iter() {
            if message.cod_mensaje == 1115941439 {
                match sunat::fetch_message_detail(
                    session,
                    message.cod_mensaje,
                    &message.context_tipo,
                ) {
                    Ok(detail) => {
                        println!(
                            "DETAIL OK code={} attachments={} body={:?}",
                            message.cod_mensaje,
                            detail.attachments.len(),
                            detail.body.chars().take(200).collect::<String>()
                        );
                        if let Some((index, attachment)) = detail
                            .attachments
                            .iter()
                            .enumerate()
                            .find(|(_, attachment)| attachment.downloadable)
                        {
                            match sunat::fetch_attachment(
                                session,
                                message.cod_mensaje,
                                &message.context_tipo,
                                index,
                            )
                            .and_then(preview::prepare)
                            {
                                Ok(prepared) => {
                                    let kind = match &prepared.content {
                                        preview::PreviewContent::Pdf { image } => {
                                            format!("pdf image={image:?}")
                                        }
                                        preview::PreviewContent::Image(path) => {
                                            format!("image={path:?}")
                                        }
                                        preview::PreviewContent::Text(text) => {
                                            format!("text={} chars", text.len())
                                        }
                                        preview::PreviewContent::Unsupported => {
                                            "unsupported".to_string()
                                        }
                                    };
                                    println!(
                                        "ATTACH OK {} ({}): {} -> {}",
                                        attachment.name,
                                        attachment.size,
                                        kind,
                                        prepared.path.display()
                                    );
                                }
                                Err(error) => eprintln!("ATTACH ERROR: {error}"),
                            }
                        }
                    }
                    Err(error) => eprintln!("DETAIL ERROR: {error}"),
                }
            }
        }
    }
}
