use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    English,
    Spanish,
}

impl Language {
    pub const ALL: [Language; 2] = [Language::English, Language::Spanish];

    pub fn label(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
        }
    }
}

pub fn t(language: Language, english: &'static str, spanish: &'static str) -> &'static str {
    match language {
        Language::English => english,
        Language::Spanish => spanish,
    }
}

pub fn window_title(language: Language) -> &'static str {
    match language {
        Language::English => "Post Bunny — Customer Operations",
        Language::Spanish => "Post Bunny — Operaciones de clientes",
    }
}

pub fn search_placeholder(language: Language) -> &'static str {
    t(
        language,
        "Search by name, RUC, DNI or SOL user…",
        "Buscar por nombre, RUC, DNI o usuario SOL…",
    )
}

pub fn password_placeholder(language: Language) -> &'static str {
    t(language, "SOL password", "Contraseña SOL")
}

pub fn login_mode_title(language: Language, login_label: &str) -> String {
    match language {
        Language::English => format!("{login_label} login"),
        Language::Spanish => format!("Acceso con {login_label}"),
    }
}

pub fn shortcuts_hint(language: Language, modifier: &str) -> String {
    t(
        language,
        "{modifier}1–3 navigate · {modifier}N new customer · {modifier}R sync",
        "{modifier}1–3 navegar · {modifier}N nuevo cliente · {modifier}R sincronizar",
    )
    .replace("{modifier}", modifier)
}

fn plural(
    language: Language,
    count: usize,
    english_one: &'static str,
    english_many: &'static str,
    spanish_one: &'static str,
    spanish_many: &'static str,
) -> String {
    let noun = match language {
        Language::English if count == 1 => english_one,
        Language::English => english_many,
        Language::Spanish if count == 1 => spanish_one,
        Language::Spanish => spanish_many,
    };
    format!("{count} {noun}")
}

pub fn customers(language: Language, count: usize) -> String {
    plural(
        language,
        count,
        "customer",
        "customers",
        "cliente",
        "clientes",
    )
}

pub fn accounts(language: Language, count: usize) -> String {
    plural(language, count, "account", "accounts", "cuenta", "cuentas")
}

pub fn messages(language: Language, count: usize) -> String {
    plural(
        language, count, "message", "messages", "mensaje", "mensajes",
    )
}

pub fn attachments(language: Language, count: usize) -> String {
    plural(
        language,
        count,
        "attachment",
        "attachments",
        "adjunto",
        "adjuntos",
    )
}

pub fn open_alerts(language: Language, count: usize) -> String {
    plural(
        language,
        count,
        "open alert",
        "open alerts",
        "alerta abierta",
        "alertas abiertas",
    )
}

pub fn unread_badge(language: Language, count: usize) -> String {
    match language {
        Language::English => format!("{count} unread"),
        Language::Spanish => format!("{count} sin leer"),
    }
}

pub fn open_badge(language: Language, count: usize) -> String {
    match language {
        Language::English => format!("{count} open"),
        Language::Spanish => format!("{count} abiertas"),
    }
}

pub fn showing_customers(language: Language, shown: usize, total: usize) -> String {
    match language {
        Language::English => format!("Showing {shown} of {total} customers"),
        Language::Spanish => format!("Mostrando {shown} de {total} clientes"),
    }
}

pub fn showing_alerts(language: Language, shown: usize, total: usize) -> String {
    match language {
        Language::English => format!("Showing {shown} of {total} alerts"),
        Language::Spanish => format!("Mostrando {shown} de {total} alertas"),
    }
}

pub fn sync_summary(
    language: Language,
    synced_accounts: usize,
    total_messages: usize,
    new_alerts: usize,
) -> String {
    match language {
        Language::English => format!(
            "Sync complete · {} · {} · {} new alerts",
            accounts(language, synced_accounts),
            messages(language, total_messages),
            new_alerts
        ),
        Language::Spanish => format!(
            "Sincronización completa · {} · {} · {} alertas nuevas",
            accounts(language, synced_accounts),
            messages(language, total_messages),
            new_alerts
        ),
    }
}

pub fn sync_partial(language: Language, ok: usize, failed: usize, first_error: &str) -> String {
    match language {
        Language::English => format!("Sync: {ok} ok, {failed} failed · {first_error}"),
        Language::Spanish => {
            format!("Sincronización: {ok} correctas, {failed} fallidas · {first_error}")
        }
    }
}

pub fn sync_failed(language: Language, error: &str) -> String {
    match language {
        Language::English => format!("Sync failed · {error}"),
        Language::Spanish => format!("Sincronización fallida · {error}"),
    }
}

pub fn deleted_customer(language: Language, name: &str) -> String {
    match language {
        Language::English => format!("{name} was deleted."),
        Language::Spanish => format!("{name} fue eliminado."),
    }
}

pub fn downloaded_attachment(language: Language, filename: &str) -> String {
    match language {
        Language::English => format!("Downloaded {filename} to Downloads."),
        Language::Spanish => format!("Se descargó {filename} en Descargas."),
    }
}
