# Guía — Buzón Electrónico SUNAT vía HTTP (sin navegador)

> Guía construida a partir de la ingeniería inversa del flujo real del navegador
> (captura de tráfico con Playwright) y **verificada** replicando el flujo completo
> sin navegador con `fetch` (`sunat-buzon/scripts/client-puro.mjs`).
>
> Verificado el **25/11/2026** con DNI `48703474` → RUC **10487034741**
> (LLAMOCCA NINAQUISPE ALBERTO). Hosts: `e-menu`, `api-seguridad`, `ww1`.

---

## 1. Qué hay que hacer en una frase

1. Hacer login OAuth en `api-seguridad.sunat.gob.pe` con `j_security_check` (form) para obtener la cookie de menú.
2. Pedir `MenuInternet.htm?action=buzon&s=ww1` → devuelve **302** a la URL del visor (`ww1…/visor/master?hc=…&token=…`).
3. Abrir esa URL del visor (obligatorio: crea la sesión del buzón).
4. Consumir el API JSON del visor (`/ol-ti-itvisornoti/visor/...`) con la cookie de sesión y el header `X-Ruc`.
5. Para el cuerpo de cada mensaje: `gendocS01Alias` (HTML). Para adjuntos: `bajarArchivo` (stateful).

---

## 2. Mapa de hosts y sesiones

| Host | Rol | Cookies clave |
|---|---|---|
| `e-menu.sunat.gob.pe` | Menú SOL / entrada al buzón | `ITMENUSESSION`, `S<RUC>MACROHEX`, `srv_id`, `TS…` |
| `api-seguridad.sunat.gob.pe` | SSO OAuth2 (login) | `MENU-SOL-LANGUAGE`, `MENUTIPOLOGIN`, `TS…` |
| `ww1.sunat.gob.pe` | Aplicación del buzón (`/ol-ti-itvisornoti/visor`) + generador de documentos (`/cl-ti-iagenerador`) | `ITVISORNOTISESSION`, `IAGENDOCSESSION`, `f5avra…`, `srv_id`, `TS…` |
| `.sunat.gob.pe` | Compartidas | `<RUC>MACROHEX` |

Las cookies `f5avra…`, `srv_id` y `TS…` las pone el balanceador (F5) y **hay que conservarlas**:
sin ellas el WAF puede reiniciar la conexión o invalidar la sesión.

---

## 3. Regla crítica: User-Agent / huella TLS (WAF F5)

El WAF de SUNAT compara el `User-Agent` con la huella TLS. Comportamiento medido:

| Cliente | `e-menu` | `api-seguridad` | `ww1` |
|---|---|---|---|
| Node `fetch` (undici) **sin UA** | ✅ 200 | ❌ ECONNRESET | ✅ 200 |
| Node `fetch` con **UA de Chrome** | ❌ `Request Rejected` | ✅ 302 | ✅ 200 |
| `curl -A '<Chrome UA>'` | ✅ 200 | ✅ 200 | ✅ 200 |
| `curl -A 'curl/8.x'` | ❌ ECONNRESET | ❌ | ✅ |
| `axios/1.7.7` | ✅ 200 | ❌ ECONNRESET | ✅ |

**Recetas que funcionan:**

- **Node/undici (la que valida esta guía):** enviar UA de navegador **solo** a `api-seguridad` y
  **ningún** `User-Agent` a `e-menu` y `ww1`. (Ver `client-puro.mjs`.)
- **curl / Python requests / curl_cffi:** usar un `User-Agent` de navegador en **todos** los hosts
  (curl pasó los tres), idealmente con impersonación TLS (`curl_cffi`/`requests-impersonate`).

**Detector de bloqueo:** si la respuesta contiene `Request Rejected`, el WAF cortó la petición.
Conviene validar siempre el primer `GET` al menú antes de continuar.

---

## 4. Flujo de login (requests exactos)

### 4.1 `GET` menú → extraer la URL OAuth

```http
GET https://e-menu.sunat.gob.pe/cl-ti-itmenu/MenuInternet.htm
```

Respuesta (1.3 KB) contiene:

```html
<script>
$(document).ready(function(){
  redirect("https://api-seguridad.sunat.gob.pe/v1/clientessol/4f3b88b3-d9d6-402a-b85d-6a0bc857746a/oauth2/authen?redirect_uri=https://e-menu.sunat.gob.pe/cl-ti-itmenu/AutenticaMenuInternet.htm&state=<STATE>&client_id=4f3b88b3-d9d6-402a-b85d-6a0bc857746a&response_type=code");
});
</script>
```

> El navegador primero hace un `HEAD MenuInternet.htm?_=<ts>` para comprobar el host; no es necesario.
> El `client_id` y el `state` **vienen del servidor**: hay que extraerlos del HTML (regex `redirect\("([^"]+)"\)`).

### 4.2 `GET` authen → página de login

```http
GET <URL_authen>            → 302 Location: …/oauth2/loginMenuSol?lang=es-PE&showDni=true&showLanguages=false&originalUrl=…&state=…
GET <URL_loginMenuSol>      → 200 HTML (103 KB) con el formulario
```

### 4.3 `POST` credenciales

```http
POST https://api-seguridad.sunat.gob.pe/v1/clientessol/4f3b88b3-d9d6-402a-b85d-6a0bc857746a/oauth2/j_security_check
Content-Type: application/x-www-form-urlencoded
```

Formulario (login por **DNI**, `tipo=1`):

```text
tipo=1
dni=48703474
custom_ruc=
j_username=
j_password=<CLAVE_SOL>
captcha=
originalUrl=https%3A%2F%2Fe-menu.sunat.gob.pe%2Fcl-ti-itmenu%2FAutenticaMenuInternet.htm
lang=es-PE
state=<STATE>
```

- Login por **RUC + usuario SOL** (`tipo=2`, pestaña RUC por defecto):
  `tipo=2&custom_ruc=<RUC>&j_username=<USUARIO_SOL>&j_password=<CLAVE>&dni=`
- `captcha` normalmente va vacío; si SUNAT activa captcha (tras intentos fallidos) exige 4 caracteres
  (imagen en `txtCaptcha`). Un cliente automatizado debe detectar el bloqueo y alertar.

Respuesta:

```http
302 Location: https://e-menu.sunat.gob.pe/cl-ti-itmenu/AutenticaMenuInternet.htm?state=…
```

### 4.4 Cerrar el circuito

```http
GET https://e-menu.sunat.gob.pe/cl-ti-itmenu/AutenticaMenuInternet.htm?state=…
   → 302 Location: https://e-menu.sunat.gob.pe/cl-ti-itmenu/MenuInternet.htm?pestana=*&agrupacion=*
GET https://e-menu.sunat.gob.pe/cl-ti-itmenu/MenuInternet.htm?pestana=*&agrupacion=*
   → 200 HTML con "Bienvenido, <NOMBRE>" + "Salir"  ← señal de login OK
```

Cookies que quedan (ejemplo real):

```text
e-menu.sunat.gob.pe  srv_id=…
e-menu.sunat.gob.pe  ITMENUSESSION=<~600 chars>     HttpOnly Secure
e-menu.sunat.gob.pe  S10487034741MACROHEX=1         HttpOnly Secure
.sunat.gob.pe        10487034741MACROHEX=1          HttpOnly Secure
api-seguridad…       MENU-SOL-LANGUAGE=es-PE ; MENUTIPOLOGIN=1
```

### 4.5 (Opcional) llamadas que hace el menú al cargar

El front del menú hace, después del login (no son necesarias para el buzón, pero documentan el contexto):

```http
GET  /a/dominios.json                        → {"items":[{"ctx":"default","sdm":"ww1"}]}
POST /cl-ti-itmenu/MenuInternet.htm          (body: action=activo)  → {"error":"false","mensaje":""}
POST /cl-ti-itmenu/MenuInternet.htm          (body: action=campana) → [{"nombreCampania":"NotificacionPorLeer","existe":false,…}, …]
POST /cl-ti-itmenu/MenuInternet.htm          (body: action=prevApp) ← lo dispara el clic en "Buzón Electrónico"
```

---

## 5. Abrir el buzón (3 requests)

```http
# 1) resolver subdominio (el front lo usa para el enlace del buzón)
GET https://e-menu.sunat.gob.pe/a/dominios.json
→ {"items":[{"ctx":"default","sdm":"ww1"}]}

# 2) pedir el buzón
GET https://e-menu.sunat.gob.pe/cl-ti-itmenu/MenuInternet.htm?action=buzon&s=ww1
→ 302 Location: https://ww1.sunat.gob.pe/ol-ti-itvisornoti/visor/master?hc=<40 hex>&token=<base64>

# 3) abrir el visor (IMPRESCINDIBLE)
GET https://ww1.sunat.gob.pe/ol-ti-itvisornoti/visor/master?hc=…&token=…
→ 200 HTML (122 KB). Aquí nacen ITVISORNOTISESSION e IAGENDOCSESSION
```

Ese HTML trae datos que el cliente necesita:

```html
<meta name="meta-token" content="10487034741"/>            <!-- RUC del contribuyente -->
<script>
  var VAR_CONTEXT  = "/ol-ti-itvisornoti/visor";
  var VAR_CONTEXT2 = "/ol-ti-itvisornoti/pagoValor";
  var VAR_UBEAN    = "rO0ABXNy…";                          <!-- UsuarioBean serializado (mismo del token) -->
  var listEtiquetas = $.parseJSON('[{"codEtiqueta":"10","descEtiqueta":"VALORES",…}]');
  var $CountBuzonMenjs = "0";
</script>
```

- **Si se omite este GET**, el API responde `200` pero con `"rows": null` (no error): el cliente debe
  reintentar/relogar. Verificado.
- El `token` es un `UsuarioBean` Java serializado (base64): campos `numRUC`, `nombreCompleto`, `login`,
  `usuarioSOL`, `codDepend` (`0053`), `nivelUOL`, `ticket`, `map` de roles. **No hace falta parsearlo**:
  el RUC se obtiene del `meta-token` (o de la respuesta del listado).
- Las etiquetas **no** vienen de un endpoint: están embebidas en el HTML (catálogo en §8).

---

## 6. Catálogo de endpoints del buzón

Base: `https://ww1.sunat.gob.pe/ol-ti-itvisornoti/visor`
Headers comunes: `Cookie` (sesión) y `X-Ruc: <RUC>`.
`X-Ruc` es **obligatorio en las mutaciones** (`ajax/crearCarpeta`, `modificarCarpeta`, `eliminarCarpeta`,
`actualizarEstado`): sin él WebLogic responde `400 Bad Request`. En los listados y en `consultarAlertas`
es opcional (probado `200` con y sin header); recomendamos enviarlo siempre.

| # | Método | Endpoint | Para qué |
|---|---|---|---|
| 1 | GET | `/ajax/listarCarpetas` | Carpetas del usuario + contadores |
| 2 | GET | `/listNotiMenPag` | Listado paginado de notificaciones/mensajes |
| 3 | GET | `/obtenerDetalleNotiMen` | Detalle **y marca leído**; devuelve cuerpo/adjuntos |
| 4 | GET | `https://ww1.sunat.gob.pe/cl-ti-iagenerador/gendocS01Alias?accion=genhtml&iddoc=…&datos=…` | HTML del mensaje (viene en `detalle.url`) |
| 5 | GET | `/bajarArchivo/{codArchivo}/{anexo}/{sistema}/{RUC}` | Descarga el adjunto del mensaje abierto |
| 6 | POST | `/consultarAlertas` | Alertas de control de lectura (badge de la campana) |
| 7 | POST | `/actualizarEstado` | Marcar destacado / urgente |
| 8 | GET | `/ajax/crearCarpeta` | Crear carpeta personal |
| 9 | GET | `/ajax/modificarCarpeta` | Renombrar carpeta |
| 10 | GET | `/ajax/eliminarCarpeta` | Eliminar carpeta |
| 11 | GET | `/ajax/moverACarpeta` | Mover/copiar mensajes a una carpeta |
| 12 | GET | `/ajax/eliminarMensajes` | Eliminar mensajes |
| — | — | `/consultarNotificaciones` | **404 — endpoint muerto** (el front lo tiene comentado) |
| — | — | `/bajarArchivo?accion=archivo&…` | **405 (GET) / 500 (POST) — no usar** |

### 6.1 Listar carpetas

```http
GET /ol-ti-itvisornoti/visor/ajax/listarCarpetas
X-Ruc: 10487034741
```

```json
[{"codCarpeta":"03","codMensaje":0,"numRuc":null,"nomCarpeta":"Órdenes de Pago","codUsumodif":null,"codUsuregis":null,"cantMensajes":0},
 {"codCarpeta":"04","codMensaje":0,"numRuc":null,"nomCarpeta":"Resoluciones de Ejecución Coactiva","codUsumodif":null,"codUsuregis":null,"cantMensajes":0}]
```

Las carpetas `03` y `04` son fijas (candado, no permiten mover). El resto (`05`, `06`, …) son las que crea el usuario.

### 6.2 Listado

```http
GET /ol-ti-itvisornoti/visor/listNotiMenPag?tipoMsj=1&codCarpeta=00&codEtiqueta=&page=1&des_asunto=&codMensaje=&tipoOrden=NADA
X-Ruc: 10487034741
```

| Param | Valores | Notas |
|---|---|---|
| `tipoMsj` | `1` mensajes · `2` notificaciones · vacío cuando se filtra por carpeta/etiqueta | |
| `codCarpeta` | `00` (bandeja del tipoMsj) · `03` · `04` · `05+` | |
| `codEtiqueta` | `10`…`20` (ver §8) | |
| `page` | `1..N` | paginación server-side |
| `des_asunto` | texto | búsqueda server-side (probado: `RVIE` → 6 resultados) |
| `codMensaje` | id | filtro server-side por mensaje (lo usan las alertas) |
| `tipoOrden` | `NADA,TODOS,LEIDOS,NO_LEIDOS,DESTACADOS,SIN_DESTACAR,MUY_URGENTES,NO_MUY_URGENTES` | **el servidor lo ignora**: el filtro lo aplica el front. `NADA` = sin filtro |

Respuesta (real, ver `api-samples/list-mensajes.json`):

```json
{"estadoRespuesta":null,"page":null,"startPage":1,"endPage":9,"total":1,"records":9,
 "rows":[{"numPag":0,"codMensaje":1115941439,"indEstado":1,"indDesta":0,"indUrg":0,
  "fecVigencia":"2026-12-12 00:00:00.0","indTipmsj":1,
  "desAsunto":"Generación de Registro RVIE Y RCE del período 202610",
  "fecEnvio":"12/11/2026","fecPublica":"12/11/2026 17:38:23","codUsremisor":"SUNAT",
  "indTexto":3,"indTipgen":2,"codDepen":"0053","indAviso":1,"cantidadArchAdj":2,
  "codEtiqueta":"00","indMensaje":0,"codCarpeta":null,"numRuc":null}]}
```

### 6.3 Detalle (y marcado de leído)

```http
GET /ol-ti-itvisornoti/visor/obtenerDetalleNotiMen?codigoMensaje=1115941439&tipoMsj=1
X-Ruc: 10487034741
```

- `tipoMsj` debe ser el **código del contexto** desde el que se abrió: `1`, `2`, o el `codCarpeta`/`codEtiqueta`
  (el front manda `tipMenj || codCarpeta || codEtiqueta`). Con `tipoMsj` vacío también responde.
- **Efecto secundario: marca el mensaje como leído** (`indEstado` pasa a `1`) y devuelve
  `updateLeido: true/false`. No existe endpoint para volver a "no leído".

Respuesta (campos):

```json
{
  "msjMensaje": "{...}",
  "indTexto": "3",                    // 1 texto simple · 2 texto con URL · 3 plantilla (usa "url")
  "codUsuario": "10487034741",
  "nombUsuario": "LLAMOCCA NINAQUISPE ALBERTO",
  "listAttach": [
    {"codArchivo":0,"nomArchivo":"LE10487034741202610001404000110120.pdf","cntTamarch":9629,"indMensaje":"2","tamanoArchivoFormat":"9,4 KB"},
    {"codArchivo":null,"numId":50020902,"indMensaje":"3","nomArchivo":null},
    {"codArchivo":0,"nomArchivo":"LE10487034741202610000804000111120.pdf","cntTamarch":10406,"indMensaje":"2","tamanoArchivoFormat":"10,2 KB"}
  ],
  "sistema": 0,
  "url": "/cl-ti-iagenerador/gendocS01Alias?accion=genhtml&iddoc=50020902&datos={…}",
  "countNotiMen": [0,0,5],
  "codTipos": ["00","00","01"],
  "tieneDocumentosAnexos": false,
  "updateLeido": true
}
```

`indMensaje` en `listAttach`: `1|2` = adjunto descargable (`nomArchivo`), `3` = documento generado
(`numId` = `iddoc` del `gendoc`).

`msjMensaje` es un JSON **con doble escapado** (`%22`, `%26%23243;`). Ejemplos:

- Propuesta RCV: `{"sistema":"0","tbodyCompras":"<tr><td>01-Factura</td><td>8</td></tr>…",
  "tbodyVentas":"…","periodo":"202610","numruc":"10487034741","razonSocial":"…"}`
- Constancias: `{"numruc":"…","razonSocial":"…","tipoRegistro":"de los RCE Y RVIE","numTicket":"20260200000010","perPeriodoTributario":"202610","listaDocumentos":"…<br/>…"}`

> Para datos "duros" (montos, cantidades) es más fiable el **HTML** de `gendoc` (§6.4);
> `msjMensaje` sirve para los parámetros y para los fragmentos HTML de la propuesta.

### 6.4 Cuerpo del mensaje (documento)

```http
GET https://ww1.sunat.gob.pe/cl-ti-iagenerador/gendocS01Alias?accion=genhtml&iddoc=50020902&datos={…}
```

- Se usa **tal cual** viene en `detalle.url` (el `datos` va URL-encoded en esa cadena).
- Devuelve HTML completo (8–13 KB) con el membrete y el texto. Es la versión "imprimible"
  (el front la muestra en un `<iframe>`; `Imprimir` imprime ese HTML).
- No requiere `X-Ruc` ni `Referer`; sólo la cookie `IAGENDOCSESSION` (creada al abrir el visor).

### 6.5 Adjuntos

```http
GET /ol-ti-itvisornoti/visor/bajarArchivo/{codArchivo}/{anexo}/{sistema}/{RUC}
X-Ruc: 10487034741
```

- `codArchivo` = `listAttach[i].codArchivo`; `anexo` = `detalle.tieneDocumentosAnexos ? 1 : 0`;
  `sistema` = `detalle.sistema` (es el `#idSistema` del visor).
- Respuesta: `200`, `Content-Type: application/pdf`, `Content-Disposition: attachment; filename=…`.
- ⚠️ **Stateful**: el backend devuelve el adjunto del **último mensaje abierto** en la sesión.
  Hay que llamar `obtenerDetalleNotiMen` del mensaje **inmediatamente antes** de cada descarga.
- ⚠️ **Limitación del lado SUNAT**: cuando un mensaje tiene 2 adjuntos, ambos vienen con
  `codArchivo: 0`; el endpoint no distingue y siempre entrega el mismo archivo
  (comprobado con MD5: los dos PDF descargados eran idénticos). Es un bug del portal:
  la UI del buzón también descarga el mismo archivo dos veces.

### 6.6 Alertas de control de lectura

```http
POST /ol-ti-itvisornoti/visor/consultarAlertas
X-Ruc: 10487034741
```

```json
{"listaAlertas":[]}
```

Forma de cada alerta (según el front): `{codMens, codEtiqueta, descTipDocumento, feclectura1, feclectura2}`.
El clic navega al mensaje con `codMensaje=codMens`.

### 6.7 Marcar destacado / urgente

```http
POST /ol-ti-itvisornoti/visor/actualizarEstado
Content-Type: application/x-www-form-urlencoded
X-Ruc: 10487034741

tipoOperacion=destacado|urgente&ind=1|0&codMensaje=1109477708
```

Respuesta: `1` (éxito) / otro valor (fallo).

### 6.8 Carpetas personales

```http
# crear (requiere X-Ruc: sin él responde 400 Bad Request de WebLogic)
GET /ol-ti-itvisornoti/visor/ajax/crearCarpeta?nom_carpeta=MI%20CARPETA&tipoMsj=1
→ {"type":"SUCCESS","header":"","content":"Se ha creado la carpeta","options":""}

# renombrar
GET /ol-ti-itvisornoti/visor/ajax/modificarCarpeta?cod_carpeta=05&nom_carpeta=NUEVO%20NOMBRE
→ {"type":"SUCCESS","header":"","content":"Se ha cambiado el nombre correctamente","options":""}

# eliminar
GET /ol-ti-itvisornoti/visor/ajax/eliminarCarpeta?cod_carpeta=05
→ {"type":"SUCCESS","header":"","content":"…","options":""}
```

Validaciones del front: nombre sólo `[A-Za-z0-9 ]`, no vacío. Tras crear/modificar/eliminar hay que
volver a `listarCarpetas`.

### 6.9 Mover / eliminar mensajes

```http
GET /ol-ti-itvisornoti/visor/ajax/moverACarpeta?cod_carpeta=05&nom_carpeta=…&cod_mensajes=1115941439&tipoMsj=1
GET /ol-ti-itvisornoti/visor/ajax/eliminarMensajes?cod_mensajes=1115941439&mensajes_leidos=1
```

- `cod_mensajes`: uno o varios `codMensaje` separados por **coma** (`id1,id2,id3`; el front excluye los de etiqueta 10).
- `mensajes_leidos`: cantidad de los seleccionados que ya estaban leídos.
- Respuesta: `{"type":"SUCCESS|DANGER","header":"","content":"mensaje para el usuario","options":""}`.
- Reglas del front: no se puede mover a `03`/`04` ni a la etiqueta `10` (VALORES);
  sólo se pueden eliminar mensajes de `tipoMsj=1` (Buzón Mensajes).
- ⚠️ No probamos mover/eliminar de verdad (mutaciones destructivas). Probamos con parámetros vacíos
  y devuelven `{"type":"SUCCESS","content":"No se ha podido eliminar"}` /
  `{"type":"DANGER","content":"No se han podido mover los mensajes"}`.

---

## 7. Modelo de datos del listado (`rows[]`)

| Campo | Significado |
|---|---|
| `codMensaje` | ID del mensaje (clave para detalle/adjuntos) |
| `indEstado` | `0` no leído · `1` leído |
| `indDesta` | `1` destacado (favorito) |
| `indUrg` | `1` urgente |
| `fecVigencia` | vencimiento (`2026-12-12 00:00:00.0`) |
| `indTipmsj` | `1` mensaje · `2` notificación |
| `desAsunto` | asunto |
| `fecEnvio` | fecha de envío (`dd/mm/yyyy`) |
| `fecPublica` | fecha/hora de publicación |
| `codUsremisor` | emisor (`SUNAT`) |
| `indTexto` | `1` texto · `2` texto con URL · `3` plantilla (requiere `gendoc`) |
| `indTipgen` | tipo de generación (2 = plantilla SIRE, etc.) |
| `codDepen` | dependencia emisora (`0053`) |
| `indAviso` | `1` aviso |
| `cantidadArchAdj` | nº de adjuntos |
| `codEtiqueta` | `00` sin etiqueta, `10`…`20` |
| `indMensaje` | indicador interno |
| `codCarpeta` | carpeta destino |

Contadores: en el detalle vienen `countNotiMen` + `codTipos`. El front los mapea así:

```js
TIPO_CARPETA=1, TIPO_ETIQUETA=2, TIPO_NOTI_MEN=0     // orden de las 3 posiciones
codTipos[2]="01" → countNotiMen[2]=5 → badge "Buzón Mensajes (5)"
codTipos[?]="02" → badge "Buzón Notificaciones (n)"
TIPO_CARPETA con codigo!=00 → contador de esa carpeta; TIPO_ETIQUETA con codigo!=00 → contador de esa etiqueta
```

---

## 8. Mapa completo de secciones (todo el buzón)

| Sección de la UI | Request |
|---|---|
| **Buzón Notificaciones** | `listNotiMenPag?tipoMsj=2&codCarpeta=00&codEtiqueta=&page=1…` |
| **Buzón Mensajes** | `listNotiMenPag?tipoMsj=1&codCarpeta=00&codEtiqueta=&page=1…` |
| Órdenes de Pago | `tipoMsj=&codCarpeta=03` |
| Resoluciones de Ejecución Coactiva | `tipoMsj=&codCarpeta=04` |
| …carpetas del usuario | `tipoMsj=&codCarpeta=<05+>` |
| 10 etiquetas | `tipoMsj=&codCarpeta=&codEtiqueta=<10…20>` |

Catálogo de etiquetas (viene embebido en el HTML del visor; **17 no existe**):

| cod | Nombre | Color |
|---|---|---|
| 10 | VALORES | `ce0d0e` |
| 11 | RESOLUCIONES DE COBRANZA | `ff9200` |
| 12 | RESOLUCIONES DE FRACCIONAMIENTO | `00afff` |
| 13 | RESOLUCIONES NO CONTENCIOSAS | `89bd12` |
| 14 | RESOLUCIONES DE FISCALIZACION | `00b27e` |
| 15 | NOTIFICACIONES ANTERIORES | `ff4546` |
| 16 | AVISOS | `d45aed` |
| 18 | INSPECCION NO INTRUSIVA | `daf7a6` |
| 19 | REQUERIMIENTOS DIVERSOS | `FFC857` |
| 20 | CARTAS | `A0522D` |

> **Para "toda la información" hay que recorrer las 14 consultas**: notificaciones (2), mensajes (1),
> `N` carpetas y las 10 etiquetas, deduplicando por `codMensaje` (un mensaje puede aparecer en varias vistas).
> En esta cuenta: 7 mensajes únicos (6 RVIE/RCE + 1 propuesta RCV) y 0 notificaciones/carpetas/etiquetas.

---

## 9. Paginación, búsqueda y orden

- `page` empieza en **1**. El front asume **26 registros por página** (`records < 26` ⇒ oculta "siguiente").
- Respuesta: `startPage`/`endPage` = rango mostrado, `records` = total de registros, `total` = nº de páginas.
- ⚠️ **Quirk real**: con 7 filas devueltas, `records` y `endPage` valían **9** y `total` 1.
  Por eso el cliente **no debe confiar en `records`**: iterar `page=1,2,3…` hasta `rows.length === 0`
  y deduplicar por `codMensaje`.
- La UI muestra "1-9 de 9" (usa `startPage-endPage de records`), aunque renderiza 7 filas.
- Búsqueda: `des_asunto=<texto>` (server-side). El front la dispara con `page=1` y conserva
  `tipoMsj/codCarpeta/codEtiqueta` actuales (`'?'` = mantener).
- Filtros (`tipoOrden`) son **client-side**.

---

## 10. Pseudo-flujo del cliente

```text
login(dni, clave)
  ├─ GET menu → authenUrl (state)
  ├─ GET authen → loginMenuSol
  ├─ POST j_security_check (tipo=1) → 302 AutenticaMenuInternet
  ├─ GET AutenticaMenuInternet → 302 MenuInternet?pestana=*&agrupacion=*
  └─ GET MenuInternet?pestana=… → validar "Bienvenido"

abrirBuzon()
  ├─ GET /a/dominios.json → sdm
  ├─ GET MenuInternet.htm?action=buzon&s=ww1 → 302 visorUrl(hc, token)
  └─ GET visorUrl → HTML: meta-token=RUC + listEtiquetas      (obligatorio)

recolectar()
  ├─ GET  /ajax/listarCarpetas
  ├─ POST /consultarAlertas
  ├─ for ctx in [ (2,00), (1,00), ( ,03), ( ,04), carpetas…, etiquetas 10..20 ]:
  │     for page in 1..N:  GET /listNotiMenPag(ctx, page)   hasta rows vacío
  ├─ dedupe por codMensaje
  └─ for m in mensajes:
        ├─ GET /obtenerDetalleNotiMen?codigoMensaje=m&tipoMsj=<ctx>   (marca leído)
        ├─ GET https://ww1…+detalle.url                                (HTML del cuerpo)
        └─ si listAttach tiene indMensaje 1|2:
              GET /bajarArchivo/{codArchivo}/{anexo}/{sistema}/{RUC}   (justo después del detalle)
```

Implementación verificada: **`sunat-buzon/scripts/client-puro.mjs`** (sin dependencias, `node >= 20`).

```bash
SUNAT_DNI=... SUNAT_PASSWORD=... node sunat-buzon/scripts/client-puro.mjs ./salida
# → salida/buzon.json + salida/adjuntos/*.pdf
```

---

## 11. Errores, sesión y casos borde (todo verificado)

| Situación | Comportamiento observado | Acción recomendada |
|---|---|---|
| Falta abrir `visor/master` | `listNotiMenPag` → `200` con `rows: null` (sin error) | Detectar `rows === null` → relogin |
| Sesión expirada | `MenuInternet.htm?action=buzon` devuelve `200` sin `Location` (en vez del `302` esperado) | Re-login |
| Sin `X-Ruc` en mutaciones (`crearCarpeta`, `actualizarEstado`, …) | `400 Bad Request` (WebLogic) | Enviar siempre `X-Ruc = RUC` |
| `tipoOrden` | Ignorado por el servidor | Filtrar en cliente |
| `bajarArchivo` sin detalle previo | `200` con **0 bytes** | Abrir detalle justo antes |
| 2 adjuntos en un mensaje | Ambos con `codArchivo: 0` → mismo archivo | Documentar como limitación de SUNAT |
| `consultarNotificaciones` | `404` | No usar |
| `bajarArchivo?accion=archivo…` | `405` GET / `500` POST | No usar |
| Abrir detalle | Marca leído | Si se quiere sólo leer sin marcar: no hay forma |
| `Request Rejected` (245 B) | WAF (UA/huella TLS) | Ver §3 |
| `captcha` | Vacío en login normal; puede exigirse tras fallos | Detectar y pedir intervención humana |
| Concurrencia | `bajarArchivo` es stateful por sesión | Descargar secuencialmente, 1 mensaje a la vez |

Notas de sesión:

- No hay token CSRF: la autenticación es por cookie + `X-Ruc`.
- `hc` (40 hex) y `token` (UsuarioBean) son de un solo uso por sesión: se obtienen del 302 y
  sirven para abrir el visor; el API no los necesita después.
- Si se reinicia el proceso, hay que repetir login + `action=buzon` (no se puede "rehidratar" sólo con cookies
  si el visor expiró; sí se puede persistir el cookie jar mientras la sesión viva).
- Los campos de fecha vienen en formatos distintos: `dd/mm/yyyy` (`fecEnvio`, `fecPublica`),
  `yyyy-MM-dd HH:mm:ss.s` (`fecVigencia`).

---

## 12. Checklist de verificación (25/11/2026)

| Endpoint | Método | Probado | Resultado |
|---|---|---|---|
| `GET /cl-ti-itmenu/MenuInternet.htm` | GET | ✅ | 200, contiene URL OAuth + `state` |
| `oauth2/j_security_check` | POST | ✅ | 302 → `AutenticaMenuInternet.htm?state=…` |
| `GET /a/dominios.json` | GET | ✅ | `sdm: ww1` |
| `MenuInternet.htm?action=buzon&s=ww1` | GET | ✅ | 302 → `visor/master?hc&token` |
| `GET /visor/master` | GET | ✅ | 200, `meta-token` + `listEtiquetas` |
| `/ajax/listarCarpetas` | GET | ✅ | 2 carpetas fijas |
| `/listNotiMenPag` (1/00) | GET | ✅ | 7 filas, 5 no leídas |
| `/listNotiMenPag` (2/00) | GET | ✅ | 0 filas |
| `/listNotiMenPag` (carpeta 03/04) | GET | ✅ | 0 filas |
| `/listNotiMenPag` (etiquetas 10–20) | GET | ✅ | 0 filas |
| `/listNotiMenPag?des_asunto=RVIE` | GET | ✅ | 6 filas |
| `/listNotiMenPag?codMensaje=…` | GET | ✅ | 1 fila |
| `/obtenerDetalleNotiMen` | GET | ✅ | detalle + `listAttach` + `url`; marca leído |
| `gendocS01Alias` | GET | ✅ | HTML 8–13 KB |
| `/bajarArchivo/0/0/0/<RUC>` | GET | ✅ | PDF (9.6/10.4 KB) |
| `/consultarAlertas` | POST | ✅ | `{"listaAlertas":[]}` |
| `/actualizarEstado` (destacado 1→0) | POST | ✅ | `1` |
| `/ajax/crearCarpeta` → `modificarCarpeta` → `eliminarCarpeta` | GET | ✅ | SUCCESS / renombrada / eliminada (cuenta quedó igual) |
| `/ajax/moverACarpeta`, `/ajax/eliminarMensajes` | GET | ⚠️ | sólo validación de error (no se ejecutaron) |
| `/consultarNotificaciones` | POST | ❌ | 404 |

---

## 13. Ejemplos de respuesta

En `sunat-buzon/api-samples/`:

| Archivo | Contenido |
|---|---|
| `list-mensajes.json` | Buzón Mensajes completo (7 filas) |
| `list-notificaciones.json` | Buzón Notificaciones vacío |
| `list-carpeta-03.json` / `list-etiqueta-10.json` | carpetas/etiquetas vacías |
| `carpetas.json` | carpetas del usuario |
| `alertas.json` | alertas |
| `detalle-1115941439.json` | mensaje con 2 adjuntos + plantilla |
| `detalle-1109477708.json` | propuesta RCV (fragmentos HTML de compras/ventas) |
| `etiquetas.json` | catálogo de etiquetas extraído del HTML |

Datos ya extraídos de esta cuenta: `sunat-buzon/resumen.json`, `cuerpos.json`, `detalles.json`,
`adjuntos/` (ver `sunat-buzon/README.md`).

---

## 14. Recomendaciones para el cliente de producción

1. **Un solo hilo por sesión**: los endpoints de adjuntos son stateful y el buzón no tolera bien la concurrencia.
2. **Persistir el cookie jar** (cifrado) y **verificar sesión** con `listNotiMenPag` (si `rows === null`, relogin).
3. **Reintentos** con backoff sólo en 5xx/timeouts; **nunca** reintentar `j_security_check` con la misma
   `state` (es de un solo uso) → reiniciar el flujo desde el `GET` del menú.
4. **Salud del WAF**: si aparece `Request Rejected`, rotar estrategia de UA/TLS (§3) y bajar la frecuencia.
5. **Idempotencia**: guardar por `codMensaje` (hash del HTML) para no duplicar mensajes entre carpetas/etiquetas.
6. **Fechas**: parsear los 3 formatos; guardar ISO 8601 UTC.
7. **Adjuntos**: guardar `nomArchivo` + `cntTamarch` de `listAttach` (fuente de verdad de cuántos son).
   Descargar el PDF disponible y, si `listAttach` declara más adjuntos de los que se pueden distinguir
   (`codArchivo` repetido), registrar la inconsistencia en vez de reintentar en bucle.
8. **Cumplimiento**: son datos personales/tributarios de un tercero; almacenar cifrado, con retención
   mínima y trazabilidad de accesos.
