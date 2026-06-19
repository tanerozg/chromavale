# ChromaVale

**Your screen, tuned to your eyes.**

ChromaVale is a free desktop app for macOS and Windows that adjusts the colors
of your entire screen in real time. From a warm reading mode to personal
color-blind correction tuned to your exact type and severity. Not one filter
for everyone, but control made for you.

This repository contains the **marketing site and interactive demo** for
ChromaVale, built on Laravel + Inertia + Vue 3.

## What's inside

- **Landing page** (`/`) - a premium, fully bilingual (EN/NL) marketing page
  with an EN/NL flag switcher in the navbar. Language is auto-detected from the
  browser on first visit and remembered in `localStorage`.
- **Live demo** (`/try`) - a working in-browser color playground:
  - A real-time color engine (temperature, saturation, contrast, brightness,
    and per-channel red/green/blue) driven by an SVG color filter.
  - Color-blindness simulation (protanopia, deuteranopia, tritanopia) and a
    daltonization correction toggle, so you can experience the core feature.
- Authentication, two-factor, passkeys and account settings from the Laravel
  starter kit, rebranded as ChromaVale.

## Tech stack

- **Backend:** Laravel, Inertia (server-driven routing)
- **Frontend:** Vue 3 (`<script setup>`, TypeScript), Tailwind CSS v4
- **Build:** Vite
- **Auth:** Laravel Fortify (2FA, passkeys)

## Getting started

Requirements: PHP, Composer, Node.js 20+.

```bash
# Install dependencies
composer install
npm install

# Environment
cp .env.example .env
php artisan key:generate

# Database (set DB_* in .env, then migrate)
php artisan migrate

# Run the dev server (Vite + Laravel)
composer run dev
```

Then open the app in your browser. The marketing landing is served at `/` and
the interactive demo at `/try`.

### Waitlist emails (Resend)

The `/download` page collects waitlist signups and sends a welcome email via
[Resend](https://resend.com). To enable sending, set in `.env`:

```bash
MAIL_MAILER=resend
RESEND_API_KEY=your_resend_api_key
MAIL_FROM_ADDRESS="hello@yourdomain.com"   # must be a verified Resend domain
```

Without a key, signups are still stored; the email is skipped and logged.
Subscribers are viewable at `/admin/waitlist` (authenticated).

> **Windows note:** if `npm run build` fails with a missing `rolldown` native
> binding, install it explicitly:
> `npm install @rolldown/binding-win32-x64-msvc --no-save`

## Useful scripts

```bash
npm run dev          # Vite dev server
npm run build        # Production build
npm run lint         # ESLint (auto-fix)
npm run format       # Prettier
npm run types:check  # vue-tsc type checking
```

## A note on the demo

The `/try` page is a browser illustration of what ChromaVale does. The
color-blindness simulation uses standard transformation matrices and the
correction uses daltonization. The desktop app applies these adjustments across
your entire screen and every application, with per-app profiles, hotkeys and
schedules.
