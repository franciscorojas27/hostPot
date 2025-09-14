# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recomendaciones de Configuración del IDE

<div align="center">

# HostPot

Aplicación de escritorio (Windows) construida con **Tauri 2**, **React 19**, **TypeScript** y **TailwindCSS**, para gestionar y automatizar un punto de acceso (Hosted Network) Wi‑Fi en Windows mediante los comandos `netsh`.

![Estado](https://img.shields.io/badge/Estado-En%20desarrollo-blue) ![Tauri](https://img.shields.io/badge/Tauri-2.x-orange) ![React](https://img.shields.io/badge/React-19-61dafb) ![License](https://img.shields.io/badge/License-MIT-green)

</div>

## 🚀 Objetivo
Simplificar el inicio, configuración y monitoreo de un Hosted Network en Windows, mostrando estado, SSID, clientes máximos, autenticación, cifrado y contraseña; además de permitir autostart y notificaciones del sistema.

## ✨ Características Principales
- Inicio / detención del Hosted Network (elevado con `ShellExecuteW runas`).
- Configuración de SSID y contraseña persistente.
- Almacenamiento local usando `tauri-plugin-store` (`app_data.json`).
- Autenticación básica (login / logout en memoria con bandera global atómica).
- Auto Start opcional del hotspot al abrir la aplicación.
- Notificaciones nativas (plugin notification).
- Modal personalizado para mensajes en lugar de `alert`.
- Sanitización y migración de valores (ej. `autoStart` de string -> boolean).
- Arquitectura segura de mutabilidad usando `Arc<Mutex<...>>` para el estado del hotspot.

## 🧱 Stack Técnico
Frontend:
- React 19 + TypeScript
- Vite 7
- TailwindCSS 4 (a través de `@tailwindcss/vite`)

Backend (Tauri / Rust):
- Tauri 2 (runtime WRY)
- Plugins: store, notification, dialog, opener, autostart
- Acceso Win32 (`windows` crate) para elevar comandos `netsh`.

## 📂 Estructura Resumida
```
root
├─ src/ (React UI)
│  ├─ pages/Home.tsx        # Pantalla principal (estado y controles)
│  ├─ contexts/AuthContext.tsx
│  └─ middleware/Guard.tsx
├─ src-tauri/
│  ├─ src/lib.rs            # Comandos Tauri y lógica Hosted Network
│  ├─ src/main.rs           # Entrada binaria
│  ├─ tauri.conf.json       # Configuración Tauri
│  └─ Cargo.toml            # Dependencias Rust
└─ public/
```

## 🔐 Autenticación
Se maneja un usuario único almacenado en el Store bajo la clave `usuario` con estructura `{ name, password }`. El estado de sesión se mantiene con una `AtomicBool`. (No apto aún para producción: faltan hashing y múltiples usuarios.)

## 🛰️ Comandos Principales (Rust / Tauri)
| Comando | Descripción |
|---------|-------------|
| `login` | Valida credenciales y activa sesión |
| `logout` | Cierra sesión |
| `is_authenticated` | Retorna booleano de sesión |
| `get_hosted_network_settings_to_fronted` | Devuelve estado actual del Hosted Network |
| `config_hosted_network` | Configura SSID y clave persistente |
| `start_hosted_network` / `stop_hosted_network` | Controla el hotspot |
| `is_alive` | Verifica si está iniciado |
| `get_auto_start` / `set_auto_start` | Gestión de inicio automático |

## 🛠️ Requisitos Previos
- Windows 10/11 con soporte Hosted Network (algunos adaptadores nuevos lo desactivan).
- Rust (toolchain estable) -> https://www.rust-lang.org
- Node.js / Bun (estás usando Bun para scripts; también funciona npm/pnpm si se adapta).

## ▶️ Ejecución en Desarrollo
Instala dependencias JS y ejecuta el modo dev (Tauri compila el lado Rust automáticamente):

```powershell
# Instalar dependencias (ej: bun)
bun install

# Modo desarrollo (frontend + backend)

```

Si prefieres npm:
```powershell
npm install
npm run tauri dev
```

## 📦 Build Producción
```powershell
# Generar ejecutable
bun run tauri build
```
El binario y artefactos quedarán en `src-tauri/target/release/` y el bundle según el configurador (NSIS / MSI / etc.).

## 🧪 Notas sobre Estado y Persistencia
- El store `app_data.json` puede contener: `usuario`, `autoStart`, `password`.
- Migraciones simples: si `autoStart` era string, se convierte automáticamente a boolean.
- La contraseña por defecto se fuerza a `"12345678"` si no existe.

## ⚠️ Limitaciones Actuales
- No gestiona múltiples interfaces de red.
- No reintenta elevación fallida.
- No hay hashing de contraseña (solo desarrollo / demo).
- Posible incompatibilidad si Microsoft deshabilita Hosted Network en el adaptador.

## 🧩 Próximas Mejores (Ideas)
- Hash + sal para credenciales.
- UI para regenerar / mostrar clave parcialmente enmascarada.
- Logs persistentes de eventos (inicio/detención/error).
- Internacionalización (i18n) multi-idioma.
- Test unitarios en Rust (parsing salida de `netsh`).
- Integración con bandeja del sistema (icono dinámico según estado).

## 🛡️ Seguridad Básica
Actualmente orientado a entorno cerrado/desarrollo. Para producción considera:
- Evitar exponer comandos peligrosos sin validación.
- Encriptar (o al menos ofuscar) valores sensibles en el store.
- Añadir sandboxing y hardening (Tauri ya reduce superficie, pero hay más por hacer).

## 🤝 Contribución
1. Haz fork / rama.
2. Crea cambios pequeños y descriptivos.
3. Ejecuta `bun run tauri dev` para validar.
4. Envía PR con contexto claro.

## 🐞 Depuración Rápida
Si la app se cierra con panic por datos corruptos en el store:
1. Cierra la app.
2. Localiza el archivo `app_data.json` (normalmente en la carpeta de datos de la app / `%APPDATA%`).
3. Elimínalo o corrígelo manualmente.

## 📜 Licencia
MIT. Si usas este código para algo productivo, añade mención y endurece seguridad.

---
¿Necesitas que genere scripts de limpieza, un componente para logs o empaquetado avanzado (MSIX)? Pídelo y lo agrego.
