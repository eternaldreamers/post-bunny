use crate::domain::message::{MessageKind, MessageLabel, SunatMessage};

use super::builders::{message, MessageSeed};

pub(super) fn messages() -> Vec<SunatMessage> {
    vec![
        message(MessageSeed {
            cod_mensaje: 1120034411,
            kind: MessageKind::Message,
            subject: "Propuesta del Registro de Compras y Ventas - 202610",
            body: "Estimada/o Contribuyente:\n\nSe ha generado la propuesta de RCE y RVIE para el período 202610.\n\nRCE - Compras: 14 comprobantes\nRVIE - Ventas: 3 comprobantes\n\nRevise la propuesta en SIRE antes del vencimiento.",
            fec_publica: "20/11/2026 08:12:05",
            fec_envio: "20/11/2026",
            vigencia: Some("02/12/2026"),
            read: false,
            urgent: true,
            attachments: Vec::new(),
            label: Some(MessageLabel {
                name: "VALORES".into(),
                color: 0xce0d0e,
            }),
        }),
        message(MessageSeed {
            cod_mensaje: 1120031980,
            kind: MessageKind::Notification,
            subject: "Notificación de fiscalización - período 202609",
            body: "Se ha iniciado una revisión de fiscalización sobre las operaciones declaradas en el período 202609.\n\nRevise los requerimientos asociados en SUNAT Operaciones en Línea y coordine con el área contable.",
            fec_publica: "19/11/2026 16:40:11",
            fec_envio: "19/11/2026",
            vigencia: Some("19/12/2026"),
            read: false,
            urgent: true,
            attachments: Vec::new(),
            label: Some(MessageLabel {
                name: "RESOLUCIONES DE FISCALIZACION".into(),
                color: 0x00b27e,
            }),
        }),
        message(MessageSeed {
            cod_mensaje: 1119987214,
            kind: MessageKind::Message,
            subject: "Recordatorio de vencimiento de declaración",
            body: "Le recordamos que el plazo para presentar la declaración del período 202610 vence el 02/12/2026.",
            fec_publica: "15/11/2026 07:05:44",
            fec_envio: "15/11/2026",
            vigencia: Some("02/12/2026"),
            read: true,
            urgent: false,
            attachments: Vec::new(),
            label: None,
        }),
    ]
}
