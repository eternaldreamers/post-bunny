import { chromium } from 'playwright-core';
import fs from 'node:fs';

const UA =
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36';
const OUT = process.env.OUT || new URL('..', import.meta.url).pathname;
const STATE = process.env.SUNAT_STATE || '/tmp/sunat-state.json';
const dni = process.env.SUNAT_DNI;
const password = process.env.SUNAT_PASSWORD;

if (!dni || !password) {
  console.error('Define SUNAT_DNI y SUNAT_PASSWORD.');
  process.exit(1);
}

fs.mkdirSync(OUT, { recursive: true });

const browser = await chromium.launch({ channel: 'chrome', headless: true });
const context = await browser.newContext({
  userAgent: UA,
  ignoreHTTPSErrors: true,
  viewport: { width: 1500, height: 1000 },
  locale: 'es-PE',
  acceptDownloads: true,
});
const page = await context.newPage();
page.on('dialog', (dialog) => dialog.accept().catch(() => {}));

await page.goto('https://e-menu.sunat.gob.pe/cl-ti-itmenu/MenuInternet.htm', {
  waitUntil: 'domcontentloaded',
  timeout: 60000,
});
await page.waitForTimeout(2500);
await page.click('#btnPorDni');
await page.waitForTimeout(500);
await page.fill('#txtDni', dni);
await page.fill('#txtContrasena', password);
await page.click('#btnAceptar');
await page.waitForURL(/MenuInternet\.htm/, { timeout: 60000 }).catch(() => {});
await page.waitForTimeout(4000);
const userLine =
  (await page.evaluate(() => document.body.innerText)).match(/Bienvenido[^\n]*/)?.[0] || '';
console.log('[login] ok:', userLine || page.url());

await page.click('#aOpcionBuzon');
await page.waitForTimeout(9000);
const frame = page.frames().find((item) => /itvisornoti\/visor\/master/.test(item.url()));
if (!frame) throw new Error('No se encontró el frame del visor del buzón');
const visorUrl = frame.url();
fs.writeFileSync(`${OUT}/visor-url.txt`, visorUrl);
await context.storageState({ path: STATE });

const visor = await context.newPage();
await visor.goto(visorUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });
await visor.waitForTimeout(3000);

const captured = new Map();
const capture = (url) => {
  if (!/listNotiMenPag/.test(url)) return;
  const parsed = new URL(url);
  const key = [
    parsed.searchParams.get('tipoMsj'),
    parsed.searchParams.get('codCarpeta'),
    parsed.searchParams.get('codEtiqueta'),
  ].join('|');
  captured.set(key, url);
};
visor.on('request', (request) => capture(request.url()));

const sections = [
  'Buzón Notificaciones',
  'Buzón Mensajes',
  'Órdenes de Pago',
  'Resoluciones de Ejecución Coactiva',
  'VALORES',
  'RESOLUCIONES DE COBRANZA',
  'RESOLUCIONES DE FRACCIONAMIENTO',
  'RESOLUCIONES NO CONTENCIOSAS',
  'RESOLUCIONES DE FISCALIZACION',
  'NOTIFICACIONES ANTERIORES',
  'AVISOS',
  'INSPECCION NO INTRUSIVA',
  'REQUERIMIENTOS DIVERSOS',
  'CARTAS',
];

const sectionLog = [];
for (const section of sections) {
  const before = captured.size;
  const locator = visor.locator(`text="${section}"`).first();
  if (await locator.count()) {
    await locator.click({ timeout: 5000 }).catch(() => {});
    await visor.waitForTimeout(2500);
  }
  sectionLog.push({
    section,
    newKeys: captured.size - before,
    keys: [...captured.keys()],
  });
  console.log(`[seccion] ${section} -> keys totales ${captured.size}`);
}

const fetchJson = async (url) =>
  visor.evaluate(async (target) => {
    const response = await fetch(target, { credentials: 'include' });
    const text = await response.text();
    try {
      return JSON.parse(text);
    } catch {
      return { raw: text.slice(0, 2000) };
    }
  }, url);

const messages = new Map();
for (const [key, url] of captured) {
  for (let pageNumber = 1; pageNumber <= 20; pageNumber++) {
    const target = new URL(url);
    target.searchParams.set('page', String(pageNumber));
    target.searchParams.set('_', String(Date.now()));
    const data = await fetchJson(target.pathname + target.search);
    const rows = data?.rows || [];
    for (const row of rows) {
      messages.set(row.codMensaje, {
        key,
        tipoMsj: target.searchParams.get('tipoMsj'),
        ...row,
      });
    }
    if (!rows.length) break;
  }
  console.log(`[lista] ${key} -> acumulados ${messages.size}`);
}
fs.writeFileSync(`${OUT}/sections.json`, JSON.stringify(sectionLog, null, 2));

const detalles = [];
const bodies = [];
let index = 0;
for (const [cod, message] of [...messages.entries()].sort(
  (left, right) => right[1].codMensaje - left[1].codMensaje
)) {
  index++;
  const detail = await fetchJson(
    `/ol-ti-itvisornoti/visor/obtenerDetalleNotiMen?codigoMensaje=${cod}&tipoMsj=${message.tipoMsj}&_=${Date.now()}`
  );
  let parsed = null;
  try {
    parsed = JSON.parse(detail.msjMensaje);
  } catch {}
  detalles.push({ ...message, tipoMsj: message.tipoMsj, detalle: detail, msjDatos: parsed });

  let bodyText = '';
  if (detail.url) {
    const bodyPage = await context.newPage();
    try {
      await bodyPage.goto(`https://ww1.sunat.gob.pe${detail.url}`, {
        waitUntil: 'domcontentloaded',
        timeout: 45000,
      });
      await bodyPage.waitForTimeout(1500);
      bodyText = (await bodyPage.evaluate(() => document.body.innerText)).trim();
      fs.writeFileSync(`${OUT}/cuerpo-${cod}.txt`, bodyText);
    } catch (error) {
      bodyText = `[error] ${error.message}`;
    } finally {
      await bodyPage.close().catch(() => {});
    }
  }
  bodies.push({
    codMensaje: cod,
    desAsunto: message.desAsunto,
    fecPublica: message.fecPublica,
    texto: bodyText,
  });
  console.log(
    `[detalle] ${index}/${messages.size} ${cod} ${message.desAsunto?.slice(0, 45)} (cuerpo ${bodyText.length} chars)`
  );
}

fs.writeFileSync(`${OUT}/detalles.json`, JSON.stringify(detalles, null, 2));
fs.writeFileSync(`${OUT}/cuerpos.json`, JSON.stringify(bodies, null, 2));

const resumen = {
  usuario: userLine,
  fecha: new Date().toISOString(),
  totalMensajes: messages.size,
  carpetas: [...captured.keys()],
  mensajes: [...messages.values()].map((message) => ({
    codMensaje: message.codMensaje,
    tipoMsj: message.tipoMsj,
    asunto: message.desAsunto,
    fechaPublicacion: message.fecPublica,
    fechaEnvio: message.fecEnvio,
    vigencia: message.fecVigencia,
    remitente: message.codUsremisor,
    dependencia: message.codDepen,
    leido: message.indEstado === 1,
    etiqueta: message.codEtiqueta,
    adjuntos: message.cantidadArchAdj,
  })),
};
fs.writeFileSync(`${OUT}/resumen.json`, JSON.stringify(resumen, null, 2));
console.log(JSON.stringify(resumen.mensajes, null, 1));

await context.storageState({ path: STATE });
await browser.close();
console.log('\n[OK] datos en', OUT);
