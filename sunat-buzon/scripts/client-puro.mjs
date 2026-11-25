const CHROME_UA =
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36';

const MENU = 'https://e-menu.sunat.gob.pe/cl-ti-itmenu/MenuInternet.htm';
const OAUTH =
  'https://api-seguridad.sunat.gob.pe/v1/clientessol/4f3b88b3-d9d6-402a-b85d-6a0bc857746a/oauth2';
const VISOR = 'https://ww1.sunat.gob.pe/ol-ti-itvisornoti/visor';

const jar = new Map();

function storeCookies(response, url) {
  const host = new URL(url).hostname;
  for (const line of response.headers.getSetCookie?.() || []) {
    const [pair, ...attributes] = line.split(';');
    const separator = pair.indexOf('=');
    if (separator < 0) continue;
    let domain = host;
    for (const attribute of attributes) {
      const [key, value] = attribute.split('=').map((part) => (part || '').trim());
      if (/^domain$/i.test(key)) domain = value.replace(/^\./, '');
    }
    const name = pair.slice(0, separator).trim();
    jar.set(`${name}@${domain}`, {
      name,
      value: pair.slice(separator + 1).trim(),
      domain,
    });
  }
}

function cookieHeader(url) {
  const host = new URL(url).hostname;
  return [...jar.values()]
    .filter((cookie) => host === cookie.domain || host.endsWith(`.${cookie.domain}`))
    .map((cookie) => `${cookie.name}=${cookie.value}`)
    .join('; ');
}

async function go(url, init = {}) {
  const host = new URL(url).hostname;
  const headers = { ...(init.headers || {}) };
  if (host.includes('api-seguridad')) headers['user-agent'] = CHROME_UA;
  const cookies = cookieHeader(url);
  if (cookies) headers.cookie = cookies;
  const response = await fetch(url, { ...init, headers, redirect: 'manual' });
  storeCookies(response, url);
  return response;
}

async function login(dni, password) {
  let response = await go(MENU);
  const html = await response.text();
  const authenUrl = html.match(/redirect\("([^"]+)"\)/)?.[1];
  if (!authenUrl) throw new Error('No se pudo obtener la URL OAuth (WAF?)');
  const state = new URL(authenUrl).searchParams.get('state');

  response = await go(authenUrl);
  if (response.status === 302) {
    response = await go(new URL(response.headers.get('location'), authenUrl).href);
  }
  if (response.status !== 200) throw new Error(`loginMenuSol: ${response.status}`);
  await response.text();

  const form = new URLSearchParams({
    tipo: '1',
    dni,
    custom_ruc: '',
    j_username: '',
    j_password: password,
    captcha: '',
    originalUrl: 'https://e-menu.sunat.gob.pe/cl-ti-itmenu/AutenticaMenuInternet.htm',
    lang: 'es-PE',
    state,
  });
  response = await go(`${OAUTH}/j_security_check`, {
    method: 'POST',
    headers: { 'content-type': 'application/x-www-form-urlencoded' },
    body: form.toString(),
  });
  if (response.status !== 302) throw new Error(`j_security_check: ${response.status}`);

  response = await go(response.headers.get('location'));
  if (response.status === 302) response = await go(response.headers.get('location'));
  const menuHtml = await response.text();
  if (!/Bienvenido|Salir/.test(menuHtml)) throw new Error('Login fallido');
  return true;
}

async function abrirBuzon() {
  const domainsResponse = await go('https://e-menu.sunat.gob.pe/a/dominios.json');
  const { items } = await domainsResponse.json();
  const sdm = items?.[0]?.sdm || 'ww1';

  let response = await go(`${MENU}?action=buzon&s=${sdm}`);
  if (response.status !== 302) throw new Error(`action=buzon: ${response.status}`);
  const visorUrl = response.headers.get('location');

  response = await go(visorUrl);
  const visorHtml = await response.text();
  const ruc = visorHtml.match(/<meta name="meta-token" content="([^"]+)"/)?.[1] || null;
  const etiquetas = JSON.parse(
    visorHtml.match(/var listEtiquetas = \$\.parseJSON\('(\[.*?\])'\)/s)?.[1] || '[]'
  );
  if (!ruc) throw new Error('No se obtuvo el RUC (sesión del visor inválida)');
  return { visorUrl, ruc, etiquetas };
}

const hdrs = (ruc) => ({ 'X-Ruc': ruc });

async function api(path, { ruc, query, form, method = 'GET' } = {}) {
  const qs = query ? `?${new URLSearchParams(query)}` : '';
  const response = await go(VISOR + path + qs, {
    method,
    headers: {
      ...hdrs(ruc),
      ...(form ? { 'content-type': 'application/x-www-form-urlencoded; charset=UTF-8' } : {}),
    },
    ...(form ? { body: new URLSearchParams(form).toString() } : {}),
  });
  const contentType = response.headers.get('content-type') || '';
  if (contentType.includes('application/json')) {
    const body = await response.json();
    if (body?.rows === null) throw new Error('Sesión del visor expirada (rows=null)');
    return body;
  }
  return {
    status: response.status,
    text: await response.text(),
    headers: Object.fromEntries(response.headers),
  };
}

const listarCarpetas = (ruc) => api('/ajax/listarCarpetas', { ruc });
const listarAlertas = (ruc) => api('/consultarAlertas', { ruc, method: 'POST' });

async function listar(
  ruc,
  {
    tipoMsj = '1',
    codCarpeta = '00',
    codEtiqueta = '',
    page = 1,
    des_asunto = '',
    codMensaje = '',
    tipoOrden = 'NADA',
  } = {}
) {
  return api('/listNotiMenPag', {
    ruc,
    query: { tipoMsj, codCarpeta, codEtiqueta, page, des_asunto, codMensaje, tipoOrden },
  });
}

async function listarTodo(ruc, context) {
  const messages = new Map();
  for (let page = 1; page <= 100; page++) {
    const data = await listar(ruc, { ...context, page });
    const rows = data.rows || [];
    for (const row of rows) messages.set(row.codMensaje, row);
    if (!rows.length) break;
  }
  return [...messages.values()];
}

async function detalle(ruc, codMensaje, tipoMsj = '1') {
  const detail = await api('/obtenerDetalleNotiMen', {
    ruc,
    query: { codigoMensaje: codMensaje, tipoMsj },
  });
  let parsed = null;
  try {
    parsed = JSON.parse(detail.msjMensaje);
  } catch {}
  return { ...detail, parsed };
}

async function cuerpo(detalleResponse) {
  if (!detalleResponse.url) return null;
  const response = await go(`https://ww1.sunat.gob.pe${detalleResponse.url}`);
  return await response.text();
}

async function descargarAdjunto(ruc, { codArchivo = 0, anexo = 0, sistema = 0 } = {}) {
  const response = await go(
    `${VISOR}/bajarArchivo/${codArchivo}/${anexo}/${sistema}/${ruc}`,
    { headers: hdrs(ruc) }
  );
  const buffer = Buffer.from(await response.arrayBuffer());
  const disposition = response.headers.get('content-disposition') || '';
  const filename = disposition.match(/filename=([^;]+)/)?.[1]?.trim();
  return { filename, size: buffer.length, buffer };
}

const actualizarEstado = (ruc, { tipoOperacion, ind, codMensaje }) =>
  api('/actualizarEstado', { ruc, method: 'POST', form: { tipoOperacion, ind, codMensaje } });

const crearCarpeta = (ruc, { nom_carpeta, tipoMsj = '1' }) =>
  api('/ajax/crearCarpeta', { ruc, query: { nom_carpeta, tipoMsj } });

const modificarCarpeta = (ruc, { cod_carpeta, nom_carpeta }) =>
  api('/ajax/modificarCarpeta', { ruc, query: { cod_carpeta, nom_carpeta } });

const eliminarCarpeta = (ruc, { cod_carpeta }) =>
  api('/ajax/eliminarCarpeta', { ruc, query: { cod_carpeta } });

const moverACarpeta = (ruc, { cod_carpeta, nom_carpeta, cod_mensajes, tipoMsj }) =>
  api('/ajax/moverACarpeta', {
    ruc,
    query: { cod_carpeta, nom_carpeta, cod_mensajes, tipoMsj },
  });

const eliminarMensajes = (ruc, { cod_mensajes, mensajes_leidos = 0 }) =>
  api('/ajax/eliminarMensajes', { ruc, query: { cod_mensajes, mensajes_leidos } });

if (import.meta.url === `file://${process.argv[1]}`) {
  const out = process.argv[2] || './buzon-salida';
  const dni = process.env.SUNAT_DNI;
  const password = process.env.SUNAT_PASSWORD;
  if (!dni || !password) {
    console.error('Define SUNAT_DNI y SUNAT_PASSWORD.');
    process.exit(1);
  }

  const fs = await import('node:fs');
  fs.mkdirSync(`${out}/adjuntos`, { recursive: true });

  await login(dni, password);
  const { ruc, etiquetas, visorUrl } = await abrirBuzon();
  console.log('RUC', ruc, '| etiquetas', etiquetas.length);

  const carpetas = await listarCarpetas(ruc);
  const alertas = await listarAlertas(ruc);
  console.log('carpetas', carpetas.map((carpeta) => `${carpeta.codCarpeta}:${carpeta.nomCarpeta}`).join(', '));
  console.log('alertas', alertas.listaAlertas.length);

  const contextos = [
    { tipoMsj: '2', codCarpeta: '00' },
    { tipoMsj: '1', codCarpeta: '00' },
    ...carpetas.map((carpeta) => ({ tipoMsj: '', codCarpeta: carpeta.codCarpeta })),
    ...etiquetas.map((etiqueta) => ({ tipoMsj: '', codEtiqueta: etiqueta.codEtiqueta })),
  ];

  const todos = new Map();
  for (const context of contextos) {
    const rows = await listarTodo(ruc, context);
    for (const row of rows) todos.set(row.codMensaje, { ...row, context });
  }
  console.log('mensajes únicos', todos.size);

  const salida = [];
  for (const message of todos.values()) {
    const tipoMsj = message.context.tipoMsj || message.indTipmsj;
    const detail = await detalle(ruc, message.codMensaje, tipoMsj);
    const html = await cuerpo(detail);
    let adjunto = null;
    if ((detail.listAttach || []).some((a) => a.indMensaje === '1' || a.indMensaje === '2')) {
      adjunto = await descargarAdjunto(ruc, { sistema: detail.sistema || 0 });
      if (adjunto.size) {
        fs.writeFileSync(`${out}/adjuntos/${adjunto.filename}`, adjunto.buffer);
      }
    }
    salida.push({
      codMensaje: message.codMensaje,
      asunto: message.desAsunto,
      fecha: message.fecPublica,
      vigencia: message.fecVigencia,
      leido: message.indEstado === 1,
      destacado: message.indDesta === 1,
      urgente: message.indUrg === 1,
      adjuntos: (detail.listAttach || []).map((item) => item.nomArchivo).filter(Boolean),
      datos: detail.parsed,
      html: html || undefined,
      adjuntoDescargado: adjunto
        ? { archivo: adjunto.filename, bytes: adjunto.size }
        : undefined,
    });
    console.log(
      ' -',
      message.codMensaje,
      message.desAsunto.slice(0, 60),
      adjunto ? `(${adjunto.size}B)` : ''
    );
  }

  fs.writeFileSync(
    `${out}/buzon.json`,
    JSON.stringify({ ruc, visorUrl, carpetas, alertas, etiquetas, mensajes: salida }, null, 2)
  );
  console.log('\nOK ->', `${out}/buzon.json`);
}
