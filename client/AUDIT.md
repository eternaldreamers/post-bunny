# Auditoría y refactorización

## Alcance

Revisión completa de `client/` (aplicación GPUI, capa SUNAT, persistencia,
previsualización de adjuntos y datos de demostración), el ejemplo de sondeo
`examples/sunat_probe.rs` y los scripts auxiliares de `sunat-buzon/scripts/`.

## Hallazgos principales

| Hallazgo | Impacto | Resolución |
| --- | --- | --- |
| `app.rs`, `views/mod.rs` y `views/customers.rs` concentraban estado, acciones, overlays y pantallas | Archivos de 1.600–1.900 líneas, difíciles de mantener y testear | División por responsabilidad en `panel/`, `ui/chrome/`, `ui/overlays/` y `ui/views/customers/` |
| Los módulos de datos demo usaban nombres de personas reales | Identificadores con datos personales | Renombrados a `personal.rs`, `company.rs` y `accounting.rs` |
| El ejemplo `sunat_probe.rs` no compilaba: duplicaba módulos con `#[path]` y `model.rs` dependía de `i18n` | `cargo check --all-targets` fallaba antes del refactor | Se añadió `src/lib.rs`; el binario y el ejemplo consumen la misma librería |
| Lógica de fecha propia (algoritmo civil-from-days) conviviendo con `chrono` | Duplicación y riesgo de errores | `domain/date.rs` usa `chrono::Local` |
| Ordenación de mensajes con parser manual de fechas | Duplicación y fallos silenciosos | `sunat/mailbox.rs` usa `NaiveDateTime::parse_from_str` |
| Configuración de ventana, entorno de desarrollo, tiempos y valores demo dispersa | Cambios repetidos en varios archivos | Centralizada en `config.rs`; breakpoints y medidas en `ui/layout.rs` |
| Reinicio y carga del formulario duplicados entre acciones, teclado y helpers de desarrollo | Riesgo de divergencias | `CustomerForm::reset`, `apply_draft` y helpers de demo en `panel/forms.rs` |
| `Page::ALL`, `storage::data_dir`, `SELECTION` y otros símbolos sin uso | Código huérfano | Eliminados o conectados a su consumidor real |
| Campo `boxed` de `TextInput` siempre verdadero | Estado muerto | Eliminado |
| Toast de error usaba el icono `trash-2` | Semántica incorrecta | Los errores usan `alert-circle` |
| Texto fijo en inglés en la ficha del cliente | Mezcla de idiomas | Movido a `i18n` |
| Cadenas de resumen de sincronización, borrado y descarga construidas a mano | Lógica de presentación dispersa | Centralizadas en `i18n` |
| `mod app` y `mod store` mezclaban dominio, estado y persistencia | Acoplamiento | Separación en `domain/`, `storage/`, `panel/` y `sunat/` |
| El sondeo y la app podían divergir | Integración no verificable de extremo a extremo | Una sola librería compartida |
| Scripts auxiliares y guía con contraseña SOL en texto plano | Riesgo de seguridad | Credenciales movidas a `SUNAT_DNI` / `SUNAT_PASSWORD`; contraseña eliminada de scripts y documentación |
| `attachments.mjs` escribía en `/tmp/sunat/out` sin garantizar el directorio | Salida huérfana o pérdida de datos | Índice guardado en `${OUT}/adjuntos.json`; rutas de sesión y visor validadas |

## Arquitectura resultante

```text
src/
  main.rs / lib.rs
  config.rs
  bootstrap/        assets, keybindings, window
  domain/           customer, message, alert, navigation, identity, date
  domain/seed/      demo customers, messages and alerts
  storage/          paths, JSON records, keystore, session cache
  sunat/            constants, types, HTTP, response, auth, mailbox, messages, html, sync
  preview/          prepare, PDF rendering, download helpers
  i18n.rs
  panel/            state, queries, actions, keyboard, persistence, render
  ui/
    layout.rs       breakpoints and shared measurements
    theme.rs        palette
    components/     buttons, badges, surfaces, inputs, icons
    text_input/     reusable single-line input and paint element
    chrome/         sidebar, topbar, statusbar
    overlays/       modal shell, form, delete, attachment, language, toast
    views/          home, alerts, customers (list, detail, overview, credentials, inbox, reading)
```

## Criterios aplicados

- Una responsabilidad por archivo; ningún archivo de pantalla supera ~450 líneas.
- Reglas de dominio sin dependencias de GPUI.
- Estado de aplicación separado de la presentación.
- Configuración global centralizada y sin valores mágicos duplicados.
- Sin comentarios en el código.
- Sin módulos, funciones, campos o constantes sin consumidor.
- Sin `allow(clippy::too_many_arguments)`, `allow(clippy::type_complexity)` ni parches equivalentes.
- Persistencia y keystore encapsulados; la UI no conoce rutas ni formatos.
- Errores de red y sesión devueltos como `Result`/`SyncOutcome`, no como estado global.

## Verificación

```sh
cd client
cargo fmt --all --check
cargo check --all-targets
cargo clippy --all-targets
cargo test
```

Resultado: compilación correcta de librería, binario y ejemplo; Clippy sin
diagnósticos; tests de identidad DNI/RUC en verde.

## Pendientes conscientes

- La integración SUNAT sigue dependiendo de `curl` y de un User-Agent de
  navegador porque es el camino validado contra el WAF de SUNAT.
- El conversor HTML a texto es deliberadamente mínimo; no pretende cubrir HTML
  arbitrario, solo los documentos del visor SUNAT.
- No hay prueba automatizada de interfaz GPUI; la verificación funcional de
  pantallas requiere ejecutar la aplicación.
- `sunat-buzon/` conserva respuestas y cuerpos extraídos de una cuenta real.
  Debe tratarse como material sensible y no publicarse sin redactar identidades.
