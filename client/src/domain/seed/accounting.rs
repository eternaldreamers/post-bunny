use crate::domain::message::{MessageKind, SunatMessage};

use super::builders::{attachment, message, MessageSeed};

pub(super) fn messages() -> Vec<SunatMessage> {
    vec![message(MessageSeed {
        cod_mensaje: 1119942110,
        kind: MessageKind::Message,
        subject: "Constancia de Recepción RVIE - 202609",
        body: "Sr. Contribuyente,\n\nLa constancia de recepción del registro RVIE del período 202609 se encuentra disponible.\n\nDescárguela desde SIRE durante las próximas 48 horas.",
        fec_publica: "18/11/2026 11:22:03",
        fec_envio: "18/11/2026",
        vigencia: Some("18/12/2026"),
        read: false,
        urgent: false,
        attachments: vec![attachment(
            "LE20556677884202609001404000110120.pdf",
            "11.1 KB",
        )],
        label: None,
    })]
}
