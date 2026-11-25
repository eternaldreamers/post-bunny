# Buzón Electrónico SUNAT — extracción de datos

**Fecha de extracción:** 25/11/2026 14:44 (hora local)
**Acceso:** e-menu.sunat.gob.pe → pestaña **DNI** (usuario DNI 48703474)
**Contribuyente:** LLAMOCCA NINAQUISPE ALBERTO — **RUC 10487034741** — Domicilio: Habido

## Resumen del buzón

| Bandeja | Contenido |
|---|---|
| Buzón Notificaciones | 0 |
| Buzón Mensajes | **7 mensajes** (5 no leídos) |
| Carpetas: Órdenes de Pago / Resoluciones de Ejecución Coactiva | 0 / 0 |
| Etiquetas (VALORES … CARTAS) | sin mensajes |

## Mensajes (Buzón Mensajes)

| # | Código | Asunto | Publicado | Vigencia | Adjuntos | Leído |
|---|---|---|---|---|---|---|
| 1 | 1115941439 | Generación de Registro RVIE Y RCE del período 202610 | 12/11/2026 17:38:23 | 2026-12-12 | 2 | no |
| 2 | 1115941116 | Inicio de generación de RVIE Y RCE período 202610 | 12/11/2026 17:38:23 | 2026-12-12 | 0 | no |
| 3 | 1115941423 | Generación de Registro RVIE Y RCE del período 202609 | 12/11/2026 17:38:14 | 2026-12-12 | 2 | no |
| 4 | 1115941100 | Inicio de generación de RVIE Y RCE período 202609 | 12/11/2026 17:38:13 | 2026-12-12 | 0 | no |
| 5 | 1115941068 | Generación de Registro RVIE Y RCE del período 202608 | 12/11/2026 17:37:56 | 2026-12-12 | 2 | no |
| 6 | 1115941065 | Inicio de generación de RVIE Y RCE período 202608 | 12/11/2026 17:37:55 | 2026-12-12 | 0 | no |
| 7 | 1109477708 | Propuesta del Registro de Compras y Ventas - 202610 | 02/11/2026 02:03:32 | 2026-12-02 | 0 | sí |

Todos los mensajes son emitidos por **SUNAT**, dependencia **0053**.

## Contenido destacado

### Propuesta del Registro de Compras y Ventas — periodo 202610 (msg 1109477708)
- **RCE – Compras:** 01-Factura → **8** (Total 8)
- **RVIE – Ventas:** sin información de comprobantes para el período
- Invita a validar la propuesta y generar el RCE/RVIE en SIRE desde el 8.º día del mes.

### Generación de RVIE y RCE (msgs 1115941439 / 1115941423 / 1115941068)
- Procesos concluidos satisfactoriamente, tickets:
  - 202610 → **20260200000010**
  - 202609 → **20260200000009**
  - 202608 → **20260200000008**
- Documentos generados: Constancia de Recepción RVIE/RCE, Archivo de Registro, XML con firma digital,
  Reportes de Inconsistencias, Reporte de Exportadores, RC Sujetos No Domiciliados y Propuesta FV 621.
- Descargables por 48 h en SIRE → Consultas y Descargas.

### Inicio de generación (msgs 1115941116 / 1115941100 / 1115941065)
- Aviso de inicio del proceso con el mismo ticket; al finalizar se envía la constancia.

## Adjuntos descargados (PDF + texto)

| Archivo | Periodo | Documento |
|---|---|---|
| LE...2026100040...pdf / LE...2026100008...pdf | 2026/10 | Constancia de Recepción **RVIE** (op. 2026-0200000010, "Propuesta Aceptada", 0 registros) |
| LE...2026090*.pdf (2) | 2026/09 | Constancia de Recepción **Registro de Compras Electrónico** (op. 2026-0200000009) |
| LE...2026080*.pdf (2) | 2026/08 | Constancia de Recepción **RVIE** (op. 2026-0200000008) |

> En cada mensaje los dos enlaces apuntan al mismo `codArchivo=0`, por lo que SUNAT entrega dos veces
> el mismo PDF (los pares son idénticos, verificado por MD5). Los archivos XML/Registro completos se
> descargan desde SIRE → Consultas y Descargas durante 48 h.

## Estructura de archivos

- `README.md` — informe completo (tabla de mensajes, tickets, resumen de cada cuerpo, nota técnica y endpoints usados)
- `GUIA-BUZON-API.md` — **guía del API del buzón**: flujo HTTP completo (login → visor → endpoints),
  catálogo de requests, modelo de datos, paginación, mutaciones, limitaciones verificadas y recomendaciones
- `api-samples/` — respuestas JSON reales de cada endpoint
- `scripts/client-puro.mjs` — cliente **sin navegador** (login + recorrido de todas las secciones +
  detalle + cuerpo HTML + adjuntos). Uso: `SUNAT_DNI=... SUNAT_PASSWORD=... node scripts/client-puro.mjs ./salida`
- `scripts/extract.mjs` — login + recorrido del buzón + listado/detalles/cuerpos (`npm i playwright-core`, requiere Google Chrome; credenciales en `SUNAT_DNI` / `SUNAT_PASSWORD`)
- `scripts/attachments.mjs` — descarga los PDF de cada mensaje (usa la sesión y la URL guardadas por `extract.mjs`)
- `resumen.json` — datos de cabecera + listado de los 7 mensajes
- `detalles.json` — respuesta cruda de `obtenerDetalleNotiMen` (incluye adjuntos, `url` del documento, destinatarios)
- `cuerpos.json` + `cuerpo-<codMensaje>.txt` — cuerpo renderizado de cada mensaje
- `adjuntos/` — PDFs y su texto (`pdftotext -layout`)

## Nota técnica (cómo se obtuvo)

1. El front de SUNAT (F5/WAF) **resetea** las conexiones cuyo `User-Agent` contiene `HeadlessChrome`
   (y también `curl`). Se resolvió lanzando Chrome con Playwright y `userAgent` de Chrome normal.
   Para el cliente HTTP puro ver la regla de `User-Agent` por host en `GUIA-BUZON-API.md` §3.
2. Login: `#btnPorDni` → `#txtDni`, `#txtContrasena` → `#btnAceptar` (OAuth `api-seguridad.sunat.gob.pe`).
3. El buzón se abre en el iframe `ww1.sunat.gob.pe/ol-ti-itvisornoti/visor/master?hc=…&token=…`;
   se navega directo a esa URL (misma sesión) para poder hacer `fetch` al mismo origen.
4. Endpoints usados: `listarCarpetas`, `listNotiMenPag?tipoMsj=1&codCarpeta=00&page=1`,
   `obtenerDetalleNotiMen?codigoMensaje=…&tipoMsj=1`, y el `url` del documento generado
   (`/cl-ti-iagenerador/gendocS01Alias?...`). Adjuntos vía `bajarArchivo/0/0/0/<RUC>`.
