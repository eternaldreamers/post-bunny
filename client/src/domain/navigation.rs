use crate::i18n::{self, Language};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Home,
    Customers,
    Alerts,
}

impl Page {
    pub fn title(self, language: Language) -> &'static str {
        match self {
            Page::Home => i18n::t(language, "Home", "Inicio"),
            Page::Customers => i18n::t(language, "Customers", "Clientes"),
            Page::Alerts => i18n::t(language, "Alerts", "Alertas"),
        }
    }

    pub fn subtitle(self, language: Language) -> &'static str {
        match self {
            Page::Home => i18n::t(
                language,
                "A quick pulse on your SUNAT accounts, mailboxes and open alerts.",
                "Un vistazo rápido a tus cuentas SUNAT, buzones y alertas abiertas.",
            ),
            Page::Customers => i18n::t(
                language,
                "SUNAT users monitored from this workspace.",
                "Usuarios SUNAT monitoreados desde este espacio de trabajo.",
            ),
            Page::Alerts => i18n::t(
                language,
                "Triage every customer alert from one place.",
                "Gestiona todas las alertas de clientes desde un solo lugar.",
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DetailTab {
    Overview,
    Credentials,
    Inbox,
}

impl DetailTab {
    pub fn label(self, language: Language) -> &'static str {
        match self {
            DetailTab::Overview => i18n::t(language, "Overview", "Resumen"),
            DetailTab::Credentials => i18n::t(language, "Credentials", "Credenciales"),
            DetailTab::Inbox => i18n::t(language, "Inbox", "Bandeja"),
        }
    }
}
