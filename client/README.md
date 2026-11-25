# Post Bunny — SUNAT Customer Operations

A desktop panel for a team that monitors **SUNAT users** and their **Buzón
Electrónico**. Built with **Rust** and **[GPUI](https://gpui.rs)**, the
GPU-accelerated UI framework behind Zed.

The interface is fully available in **English** and **Spanish**. Use the
language icon in the header to switch; the choice is persisted. SUNAT document
content, demo customer notes and low-level provider errors keep their original
language.

## What is a customer?

A customer is a SUNAT user. Adding one only requires its SOL login:

- **DNI login:** DNI + SOL password. The RUC is derived automatically
  (`10` + DNI + check digit).
- **RUC login:** RUC + SOL user + SOL password. The RUC check digit is validated.

A customer record stores the login type, document, SOL user and password, plus
the cached mailbox data and sync state. The HTTP flow for the real SUNAT
integration is documented in `../sunat-buzon/GUIA-BUZON-API.md`.

## Screens

- **Sync** — real SUNAT login + mailbox synchronization for any customer with a
  real SOL password. `Sync` in the customer header or `⌘R` for all accounts.
  Demo accounts are skipped. After a sync, message bodies are loaded on demand
  when you open a message.
- **Home** — three headline metrics, open alerts and mailbox activity.
  Clicking an alert opens its message directly.
- **Customers** — searchable list (name, RUC, DNI or SOL user) and the full
  customer detail:
  - account overview (RUC, DNI, SOL user, last sync, notes)
  - **Credentials** — the stored SUNAT SOL login, with reveal/hide password and
    an edit flow
  - **Inbox** — the SUNAT mailbox, with message list, reading pane, labels,
    urgent flags and attachment previews (PDF first page, images and text) in a
    white modal with a Download button that saves the original file to Downloads
  - create, edit and delete customers
- **Alerts** — a compact queue of new SUNAT messages that need attention.
  Clicking a row opens that alert's message directly in the customer inbox and
  resolves the alert; the check button resolves it without opening. Filters:
  All / Open / Resolved.

## Run

Requirements: stable Rust and the platform dependencies described in the
[GPUI README](https://github.com/zed-industries/zed/tree/main/crates/gpui#dependencies).

```sh
cd client
cargo run
```

On macOS the app uses GPUI's Blade backend so it does not require the optional
Metal shader toolchain. Linux and Windows use GPUI's default backends.

## Keyboard

- `⌘1` / `⌘2` / `⌘3` (or `Ctrl+1…3`) — Home / Customers / Alerts
- `⌘N` — new customer
- `⌘R` — sync all accounts
- `Esc` — dismiss the current overlay
- `Tab` / `Shift+Tab` — move through the form fields
- `Enter` — save the form

## Project layout

```
client/
  assets/icons/          lucide icons served through a small AssetSource
  examples/sunat_probe.rs headless SUNAT integration probe
  src/main.rs            binary entry point
  src/lib.rs             library surface used by the binary, probe and tests
  src/config.rs          window, timing, demo and environment configuration
  src/bootstrap/         assets, keybindings and window setup
  src/domain/            customer, message, alert, navigation and identity rules
  src/domain/seed/       SUNAT-shaped demo data
  src/storage/           JSON persistence, paths, sessions and OS keystore
  src/sunat/             OAuth, HTTP transport, mailbox traversal and parsing
  src/preview/           attachment downloads, PDF rasterization and text preview
  src/i18n.rs            English/Spanish copy and pluralization helpers
  src/panel/             application state, queries, actions and keyboard handling
  src/ui/                theme, layout, components, chrome, overlays and screens
  AUDIT.md               refactor findings, architecture and verification
```

## Data

On first launch the app starts with SUNAT-shaped demo data so the full interface
can be reviewed without credentials. Customers, mailbox caches and alerts are
then stored under the platform application-data directory, so they survive
restarts. Accounts without a real SOL password are skipped by the sync action;
add real credentials through **Edit credentials**.

The real flow follows `../sunat-buzon/GUIA-BUZON-API.md`: OAuth form login on
`api-seguridad`, viewer bootstrap on `ww1`, `listNotiMenPag` across notifications,
messages, folders and labels, and on-demand `obtenerDetalleNotiMen` + `gendoc`
for message bodies. `src/sunat/http.rs` shells out to **curl** with a browser
User-Agent because that is the path verified against SUNAT's F5 WAF.

Customers, mailbox caches, alerts and the selected language are persisted under
the platform application-data directory. SOL passwords are stored in the **OS keystore**
(macOS Keychain, Windows Credential Manager, Secret Service on Linux), never in
the JSON store. SUNAT session cookies and metadata are cached there too and
reused across syncs; the app only logs in again when the cached session is no
longer valid. Opening a message body calls SUNAT's detail endpoint, which marks
the message as read in SUNAT. Attachments are downloaded on demand when you
open a preview. PDF previews are rasterized to their first page with `sips`
(macOS) or `pdftoppm` (Poppler). The preview modal can save the original file to
the system Downloads folder.

## Manual SUNAT probe

The integration can be exercised without the UI (DNI login):

```sh
SUNAT_DNI=... SUNAT_PASSWORD=... cargo run --example sunat_probe
```

It prints the resolved RUC, name, message list and one rendered body. The probe
uses the same `src/sunat` client as the app. Like the app, opening a detail
marks the message as read in SUNAT.

## Developer previews

Optional environment variables for jumping straight into a screen while working
on the layout:

```sh
PANEL_WIDTH=760 PANEL_HEIGHT=820   # initial window size
PANEL_PAGE=customers               # home | customers | alerts
PANEL_CUSTOMER=1                   # open a customer detail view
PANEL_TAB=credentials              # overview | credentials | inbox
PANEL_MESSAGE=1115941439           # open a specific SUNAT message
PANEL_MODAL=new                    # new | edit
PANEL_LOGIN=ruc                    # start the new-customer form in RUC mode
PANEL_SEARCH=andes                 # pre-fill the customer search
PANEL_DRAWER=1                     # open the narrow-screen drawer
PANEL_DELETE=1                     # open the delete confirmation
PANEL_ALERT=1                      # open alert #1 and jump to its inbox
```
