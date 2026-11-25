import { chromium } from 'playwright-core';
import fs from 'node:fs';

const UA =
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36';
const OUT = process.env.OUT || new URL('../adjuntos', import.meta.url).pathname;
const STATE = process.env.SUNAT_STATE || '/tmp/sunat-state.json';
const VISOR_FILE = process.env.SUNAT_VISOR_FILE ||
  new URL('../visor-url.txt', import.meta.url).pathname;

const subjects = [
  'Generación de Registro RVIE Y RCE del período 202610',
  'Generación de Registro RVIE Y RCE del período 202609',
  'Generación de Registro RVIE Y RCE del período 202608',
];

if (!fs.existsSync(STATE)) {
  console.error(`No existe la sesión ${STATE}. Ejecuta extract.mjs primero.`);
  process.exit(1);
}
if (!fs.existsSync(VISOR_FILE)) {
  console.error(`No existe ${VISOR_FILE}. Ejecuta extract.mjs primero.`);
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
  storageState: STATE,
});
const page = await context.newPage();
const visorUrl = fs.readFileSync(VISOR_FILE, 'utf8').trim();
await page.goto(visorUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });
await page.waitForTimeout(3000);
await page
  .getByText('Buzón Mensajes', { exact: false })
  .first()
  .click()
  .catch(() => {});
await page.waitForTimeout(3000);

const saved = [];
for (const subject of subjects) {
  await page
    .getByText(subject, { exact: true })
    .first()
    .click()
    .catch((error) => console.log('click subj err', error.message.slice(0, 60)));
  await page.waitForTimeout(3500);

  const count = await page.locator('a', { hasText: /\.pdf$/i }).count();
  console.log(`[${subject}] adjuntos visibles: ${count}`);

  for (let index = 0; index < count; index++) {
    const link = page.locator('a', { hasText: /\.pdf$/i }).nth(index);
    const name = (await link.innerText()).trim();
    const downloadPromise = page
      .waitForEvent('download', { timeout: 20000 })
      .catch(() => null);
    await link
      .click({ force: true })
      .catch((error) => console.log('  click err', error.message.slice(0, 60)));
    const download = await downloadPromise;
    if (download) {
      const path = `${OUT}/${name.replace(/\.pdf$/i, '')}.pdf`;
      await download.saveAs(path);
      const size = fs.statSync(path).size;
      console.log('  guardado', path, size, 'bytes');
      saved.push({ mensaje: subject, archivo: name, path, size });
    } else {
      console.log('  sin download para', name);
    }
    await page.waitForTimeout(1500);
  }
}

fs.writeFileSync(`${OUT}/adjuntos.json`, JSON.stringify(saved, null, 2));
await context.storageState({ path: STATE });
await browser.close();
