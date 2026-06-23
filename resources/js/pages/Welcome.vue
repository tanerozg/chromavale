<script setup lang="ts">
import { Head, Link } from '@inertiajs/vue3';
import { ref, computed, watch, onMounted } from 'vue';
import { dashboard, login } from '@/routes';

type Lang = 'en' | 'nl';
const STORAGE_KEY = 'chromavale-lang';
const lang = ref<Lang>('en');

function setLang(value: Lang) {
    lang.value = value;
}

onMounted(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === 'en' || saved === 'nl') {
        lang.value = saved;
    } else if (navigator.language?.toLowerCase().startsWith('nl')) {
        lang.value = 'nl';
    }
});

watch(lang, (value) => {
    localStorage.setItem(STORAGE_KEY, value);
    document.documentElement.lang = value;
});

const featureIcons = [
    'sliders',
    'layers',
    'window',
    'command',
    'clock',
    'transition',
] as const;

const content = {
    en: {
        nav: {
            features: 'Features',
            colorblind: 'Color blindness',
            pricing: 'Pricing',
            demo: 'Demo',
            test: 'Free test',
            signIn: 'Sign in',
            dashboard: 'Dashboard',
            download: 'Download free',
        },
        hero: {
            platforms: 'macOS & Windows',
            title1: 'Your screen,',
            title2: 'tuned to',
            title3: 'your eyes.',
            lede: 'ChromaVale adjusts the colors of your entire screen in real time. From a warm reading mode to personal color-blind correction tuned to your exact type and severity. Not one filter for everyone, but control made for you.',
            ctaPrimary: 'Download free',
            ctaSecondary: 'See features',
            fine: 'Free to use. One-time Pro upgrade, no subscription. macOS 12+ and Windows 10/11.',
            preview: 'Preview',
            chips: ['Comfort', 'Reading', 'Night', 'Vivid'],
            temperature: 'Temperature',
            saturation: 'Saturation',
        },
        stats: [
            {
                num: '8%',
                label: 'of men are color blind, and are served generic presets everywhere.',
            },
            {
                num: '2 → 1',
                label: 'Mac and Windows in one app, with the same look and controls.',
            },
            {
                num: '€0',
                label: 'to get started. One-time for Pro, never a subscription.',
            },
        ],
        featuresHead: {
            kicker: 'Features',
            heading: 'Everything Windows Color Filters doesn’t do.',
            sub: 'The built-in filters are basic, hidden and Windows-only. ChromaVale gives you deep control, on both platforms, in an interface you’ll actually want to open.',
        },
        features: [
            {
                title: 'Color engine',
                body: 'Set color temperature, tint, saturation, contrast and gamma. Adjustable per channel for red, green and blue, in real time across your whole screen.',
            },
            {
                title: 'Ready-made presets',
                body: 'Comfort, reading mode, high contrast, grayscale and vivid. Beautiful profiles for instant value, ready to use.',
            },
            {
                title: 'Per-app filtering',
                body: 'Apply a profile to specific apps only. Warm tones in your editor, neutral in your photo tool.',
            },
            {
                title: 'Profiles and shortcuts',
                body: 'Build unlimited custom color profiles and switch instantly with a hotkey. No menu to dig through.',
            },
            {
                title: 'Smart schedules',
                body: 'Automatically warmer in the evening, your correction profile by day. Your screen adapts without you thinking about it.',
            },
            {
                title: 'Smooth transitions',
                body: 'Profiles fade gently into each other, no harsh jump. Control from the system tray, always within reach.',
            },
        ],
        spotlight: {
            kicker: 'The killer feature',
            heading: 'Not a filter. A tool that fits your eyes.',
            sub: 'A short test in the app determines your exact type, protan, deutan or tritan, and its severity. Then ChromaVale remaps precisely the colors that confuse you into tones you can actually tell apart. Not generic. Made to measure.',
            checks: [
                'Interactive calibration determines type and severity',
                'Personal daltonization, not a fixed preset',
                'Distinction mode pulls tricky colors apart',
            ],
            cta: 'Try the correction',
            without: 'Without correction',
            with: 'With ChromaVale',
            withoutNote: 'Red and green blur together.',
            withNote: 'Each tone gets its own space. Distinguishable again.',
        },
        compare: {
            kicker: 'Comparison',
            heading: 'Why ChromaVale is better.',
            sub: 'Three real differences: cross-platform, personal color-blind correction, and deep control with automation.',
            cols: ['Windows Filters', 'ScreenTint / f.lux', 'ChromaVale'],
            rows: [
                {
                    label: 'Platform',
                    windows: 'Windows only',
                    flux: 'Mostly Windows',
                    chroma: 'Mac and Windows',
                },
                {
                    label: 'Color control',
                    windows: '6 presets + intensity',
                    flux: 'Warmth and palettes',
                    chroma: 'Fully per channel, unlimited palettes',
                },
                {
                    label: 'Color blindness',
                    windows: '3 generic presets',
                    flux: 'Basic or none',
                    chroma: 'Personal test and made-to-measure daltonization',
                },
                {
                    label: 'Per-app filter',
                    windows: false,
                    flux: 'Sometimes',
                    chroma: true,
                },
                {
                    label: 'Profiles, hotkeys, schedules',
                    windows: false,
                    flux: 'Limited',
                    chroma: true,
                },
                {
                    label: 'Interface',
                    windows: 'Hidden in settings',
                    flux: 'Simple',
                    chroma: 'Polished app with tray and smooth transitions',
                },
            ],
        },
        pricing: {
            kicker: 'Pricing',
            heading: 'Free to start. One-time for Pro.',
            sub: 'For most people ChromaVale feels completely free. Those who use the personal correction daily pay once. No subscription.',
            recommended: 'Recommended',
            free: {
                name: 'Free',
                desc: 'The magnet. Instant value.',
                amount: '€0',
                per: 'forever',
                points: [
                    'Basic screen color adjustment',
                    'Fixed presets: night, grayscale, contrast, comfort',
                    'Basic intensity and brightness',
                    'Fully on Mac and Windows',
                ],
                cta: 'Download for Mac and Windows',
            },
            pro: {
                name: 'Pro',
                desc: 'One-time. The full toolkit.',
                amount: '€15',
                per: 'one-time, no sub',
                points: [
                    'Personal color-blind calibration and daltonization',
                    'Unlimited custom palettes',
                    'Per-app and per-window filtering',
                    'Profiles, shortcuts and automatic schedules',
                ],
                cta: 'Unlock Pro',
                fine: 'Try the color-blind correction for free first.',
            },
        },
        cta: {
            title: 'Ready to tune your screen to your eyes?',
            sub: 'Download free for Mac and Windows. Set up in a minute.',
            mac: 'Download for macOS',
            win: 'Download for Windows',
        },
        footer: {
            line: 'Your screen, your colors. For Mac and Windows.',
        },
    },
    nl: {
        nav: {
            features: 'Functies',
            colorblind: 'Kleurenblind',
            pricing: 'Prijzen',
            demo: 'Demo',
            test: 'Gratis test',
            signIn: 'Inloggen',
            dashboard: 'Dashboard',
            download: 'Download gratis',
        },
        hero: {
            platforms: 'macOS & Windows',
            title1: 'Jouw scherm,',
            title2: 'afgestemd op',
            title3: 'jouw ogen.',
            lede: 'ChromaVale past de kleuren van je hele scherm aan in realtime. Van een warme leesmodus tot persoonlijke kleurenblind-correctie die is afgestemd op jouw type en ernst. Niet één filter voor iedereen, maar controle op maat.',
            ctaPrimary: 'Download gratis',
            ctaSecondary: 'Bekijk functies',
            fine: 'Gratis te gebruiken. Eenmalige Pro-upgrade, geen abonnement. macOS 12+ en Windows 10/11.',
            preview: 'Voorbeeld',
            chips: ['Comfort', 'Lezen', 'Nacht', 'Levendig'],
            temperature: 'Temperatuur',
            saturation: 'Verzadiging',
        },
        stats: [
            {
                num: '8%',
                label: 'van de mannen is kleurenblind, en wordt overal met generieke presets bediend.',
            },
            {
                num: '2 → 1',
                label: 'Mac en Windows in één app, met dezelfde uitstraling en bediening.',
            },
            {
                num: '€0',
                label: 'om te beginnen. Eenmalig voor Pro, nooit een abonnement.',
            },
        ],
        featuresHead: {
            kicker: 'Functies',
            heading: 'Alles wat Windows Color Filters niet doet.',
            sub: 'De ingebouwde filters zijn basaal, verstopt en Windows-only. ChromaVale geeft je diepe controle, op beide platformen, in een interface die je graag opent.',
        },
        features: [
            {
                title: 'Kleuren-engine',
                body: 'Stel kleurtemperatuur, tint, verzadiging, contrast en gamma in. Per kanaal regelbaar voor rood, groen en blauw, in realtime over je hele scherm.',
            },
            {
                title: 'Kant-en-klare presets',
                body: 'Comfort, leesmodus, hoog contrast, grijswaarden en levendig. Mooie profielen voor directe waarde, klaar om te gebruiken.',
            },
            {
                title: 'Per-app filtering',
                body: 'Pas een profiel alleen toe op specifieke programma’s. Warme tinten in je editor, neutraal in je fotobewerker.',
            },
            {
                title: 'Profielen en sneltoetsen',
                body: 'Bouw onbeperkt eigen kleurprofielen en wissel er direct tussen met een hotkey. Geen menu in om te duiken.',
            },
            {
                title: 'Slimme schema’s',
                body: 'Automatisch warmer in de avond, je correctieprofiel overdag. Je scherm past zich aan zonder dat je eraan denkt.',
            },
            {
                title: 'Vloeiende overgangen',
                body: 'Profielen faden zacht in elkaar over, zonder harde sprong. Bediening vanuit de systeemtray, altijd binnen handbereik.',
            },
        ],
        spotlight: {
            kicker: 'De killer-feature',
            heading: 'Geen filter. Een hulpmiddel dat past bij jouw ogen.',
            sub: 'Een korte test in de app bepaalt je exacte type, protan, deutan of tritan, en de ernst ervan. Daarna hermapt ChromaVale precies de kleuren die voor jou verwarren naar tinten die je wél kunt onderscheiden. Niet generiek. Op maat.',
            checks: [
                'Interactieve kalibratie bepaalt type en ernst',
                'Persoonlijke daltonisatie, geen vaste preset',
                'Onderscheid-modus trekt lastige kleuren uit elkaar',
            ],
            cta: 'Probeer de correctie',
            without: 'Zonder correctie',
            with: 'Met ChromaVale',
            withoutNote: 'Rood en groen lopen in elkaar over.',
            withNote: 'Elke tint krijgt eigen ruimte. Weer te onderscheiden.',
        },
        compare: {
            kicker: 'Vergelijking',
            heading: 'Waarom ChromaVale beter is.',
            sub: 'Drie echte verschillen: cross-platform, persoonlijke kleurenblind-correctie en diepe controle met automatisering.',
            cols: ['Windows Filters', 'ScreenTint / f.lux', 'ChromaVale'],
            rows: [
                {
                    label: 'Platform',
                    windows: 'Alleen Windows',
                    flux: 'Vooral Windows',
                    chroma: 'Mac en Windows',
                },
                {
                    label: 'Kleurcontrole',
                    windows: '6 presets + intensiteit',
                    flux: 'Warmte en paletten',
                    chroma: 'Volledig per kanaal, onbeperkte paletten',
                },
                {
                    label: 'Kleurenblind',
                    windows: '3 generieke presets',
                    flux: 'Basis of geen',
                    chroma: 'Persoonlijke test en daltonisatie op maat',
                },
                {
                    label: 'Per-app filter',
                    windows: false,
                    flux: 'Soms',
                    chroma: true,
                },
                {
                    label: 'Profielen, hotkeys, schema’s',
                    windows: false,
                    flux: 'Beperkt',
                    chroma: true,
                },
                {
                    label: 'Interface',
                    windows: 'Verstopt in instellingen',
                    flux: 'Simpel',
                    chroma: 'Gepolijste app met tray en vloeiende overgangen',
                },
            ],
        },
        pricing: {
            kicker: 'Prijzen',
            heading: 'Gratis te beginnen. Eenmalig voor Pro.',
            sub: 'Voor de meeste mensen voelt ChromaVale volledig gratis. Wie de persoonlijke correctie dagelijks gebruikt, betaalt één keer. Geen abonnement.',
            recommended: 'Aanrader',
            free: {
                name: 'Gratis',
                desc: 'De magneet. Direct waarde.',
                amount: '€0',
                per: 'voor altijd',
                points: [
                    'Basale schermkleur-aanpassing',
                    'Vaste presets: nacht, grijswaarden, contrast, comfort',
                    'Basisintensiteit en helderheid',
                    'Volledig op Mac en Windows',
                ],
                cta: 'Download voor Mac en Windows',
            },
            pro: {
                name: 'Pro',
                desc: 'Eenmalig. De volledige toolkit.',
                amount: '€15',
                per: 'eenmalig, geen abo',
                points: [
                    'Persoonlijke kleurenblind-kalibratie en daltonisatie',
                    'Onbeperkte custom paletten',
                    'Per-app en per-venster filtering',
                    'Profielen, sneltoetsen en automatische schema’s',
                ],
                cta: 'Pro ontgrendelen',
                fine: 'Probeer de kleurenblind-correctie eerst gratis.',
            },
        },
        cta: {
            title: 'Klaar om je scherm op jouw ogen af te stemmen?',
            sub: 'Download gratis voor Mac en Windows. In een minuut ingesteld.',
            mac: 'Download voor macOS',
            win: 'Download voor Windows',
        },
        footer: {
            line: 'Jouw scherm, jouw kleuren. Voor Mac en Windows.',
        },
    },
} as const;

const t = computed(() => content[lang.value]);
</script>

<template>
    <Head title="ChromaVale - Your screen, your colors">
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link
            rel="preconnect"
            href="https://fonts.gstatic.com"
            crossorigin
        />
        <link
            href="https://fonts.googleapis.com/css2?family=Instrument+Sans:ital,wght@0,400..700;1,400..700&display=swap"
            rel="stylesheet"
        />
    </Head>

    <div class="page">
        <div class="halo" aria-hidden="true"></div>

        <!-- Navigation -->
        <header class="nav-wrap">
            <nav class="nav">
                <a href="#top" class="brand">
                    <span class="brand-mark">
                        <svg viewBox="0 0 32 32" fill="none" aria-hidden="true">
                            <circle
                                cx="16"
                                cy="16"
                                r="13.5"
                                stroke="currentColor"
                                stroke-width="2.5"
                            />
                            <circle
                                cx="16"
                                cy="16"
                                r="6.5"
                                fill="url(#prism)"
                            />
                            <defs>
                                <linearGradient
                                    id="prism"
                                    x1="9"
                                    y1="9"
                                    x2="23"
                                    y2="23"
                                    gradientUnits="userSpaceOnUse"
                                >
                                    <stop stop-color="#FF7A6B" />
                                    <stop offset="0.3" stop-color="#FFC24B" />
                                    <stop offset="0.55" stop-color="#5BD6A0" />
                                    <stop offset="0.78" stop-color="#4DA6FF" />
                                    <stop offset="1" stop-color="#9B6BE5" />
                                </linearGradient>
                            </defs>
                        </svg>
                    </span>
                    <span class="brand-name">ChromaVale</span>
                </a>

                <div class="nav-links">
                    <a href="#features">{{ t.nav.features }}</a>
                    <a href="#colorblind">{{ t.nav.colorblind }}</a>
                    <a href="#pricing">{{ t.nav.pricing }}</a>
                    <Link href="/try">{{ t.nav.demo }}</Link>
                    <Link href="/calibrate">{{ t.nav.test }}</Link>
                </div>

                <div class="nav-actions">
                    <div class="lang" role="group" aria-label="Language">
                        <button
                            type="button"
                            class="lang-btn"
                            :class="{ active: lang === 'en' }"
                            :aria-pressed="lang === 'en'"
                            title="English"
                            @click="setLang('en')"
                        >
                            <svg viewBox="0 0 20 14" aria-hidden="true">
                                <clipPath id="gbclip">
                                    <rect width="20" height="14" rx="2" />
                                </clipPath>
                                <g clip-path="url(#gbclip)">
                                    <rect width="20" height="14" fill="#012169" />
                                    <path
                                        d="M0 0l20 14M20 0L0 14"
                                        stroke="#fff"
                                        stroke-width="2.6"
                                    />
                                    <path
                                        d="M0 0l20 14M20 0L0 14"
                                        stroke="#C8102E"
                                        stroke-width="1.4"
                                    />
                                    <path
                                        d="M10 0v14M0 7h20"
                                        stroke="#fff"
                                        stroke-width="4"
                                    />
                                    <path
                                        d="M10 0v14M0 7h20"
                                        stroke="#C8102E"
                                        stroke-width="2.2"
                                    />
                                </g>
                            </svg>
                            <span>EN</span>
                        </button>
                        <button
                            type="button"
                            class="lang-btn"
                            :class="{ active: lang === 'nl' }"
                            :aria-pressed="lang === 'nl'"
                            title="Nederlands"
                            @click="setLang('nl')"
                        >
                            <svg viewBox="0 0 20 14" aria-hidden="true">
                                <clipPath id="nlclip">
                                    <rect width="20" height="14" rx="2" />
                                </clipPath>
                                <g clip-path="url(#nlclip)">
                                    <rect width="20" height="14" fill="#fff" />
                                    <rect width="20" height="4.67" fill="#AE1C28" />
                                    <rect
                                        y="9.33"
                                        width="20"
                                        height="4.67"
                                        fill="#21468B"
                                    />
                                </g>
                            </svg>
                            <span>NL</span>
                        </button>
                    </div>

                    <Link
                        v-if="$page.props.auth.user"
                        :href="dashboard()"
                        class="text-link"
                    >
                        {{ t.nav.dashboard }}
                    </Link>
                    <Link v-else :href="login()" class="text-link">
                        {{ t.nav.signIn }}
                    </Link>
                    <Link href="/download" class="btn btn-ink btn-sm">
                        {{ t.nav.download }}
                    </Link>
                </div>
            </nav>
        </header>

        <main id="top">
            <!-- Hero -->
            <section class="hero">
                <div class="hero-copy">
                    <span class="platforms">
                        <svg
                            class="plat-ico"
                            viewBox="0 0 16 16"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                d="M11.18 8.5c-.02-1.5 1.22-2.22 1.28-2.26-.7-1.02-1.79-1.16-2.18-1.18-.93-.09-1.81.54-2.28.54-.47 0-1.2-.53-1.97-.51-1.01.01-1.94.59-2.46 1.49-1.05 1.82-.27 4.51.75 5.99.5.72 1.1 1.53 1.88 1.5.75-.03 1.04-.49 1.95-.49.91 0 1.17.49 1.97.47.81-.01 1.33-.74 1.83-1.46.58-.84.82-1.65.83-1.69-.02-.01-1.6-.61-1.62-2.43Zm-1.5-4.47c.41-.5.69-1.2.61-1.9-.59.02-1.31.39-1.74.89-.38.44-.72 1.15-.63 1.83.66.05 1.34-.33 1.76-.82Z"
                            />
                        </svg>
                        <svg
                            class="plat-ico"
                            viewBox="0 0 16 16"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                d="M0 2.2 6.5 1.3v6.2H0V2.2Zm0 11.6 6.5.9V8.5H0v5.3ZM7.3 1.2 16 0v7.5H7.3V1.2Zm0 13.6L16 16V8.5H7.3v6.3Z"
                            />
                        </svg>
                        <span>{{ t.hero.platforms }}</span>
                    </span>
                    <h1 class="display">
                        {{ t.hero.title1 }}<br />
                        {{ t.hero.title2 }}<br />
                        <span class="ink-grad">{{ t.hero.title3 }}</span>
                    </h1>
                    <p class="lede">{{ t.hero.lede }}</p>
                    <div class="hero-cta">
                        <Link href="/download" class="btn btn-ink btn-lg">
                            <svg
                                viewBox="0 0 20 20"
                                fill="none"
                                class="btn-ico"
                                aria-hidden="true"
                            >
                                <path
                                    d="M10 3v10m0 0 4-4m-4 4-4-4M4 16h12"
                                    stroke="currentColor"
                                    stroke-width="1.7"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                />
                            </svg>
                            {{ t.hero.ctaPrimary }}
                        </Link>
                        <a href="#features" class="btn btn-soft btn-lg">
                            {{ t.hero.ctaSecondary }}
                        </a>
                    </div>
                    <p class="hero-fine">{{ t.hero.fine }}</p>
                </div>

                <!-- App window mockup -->
                <div class="hero-visual">
                    <div class="window">
                        <div class="window-bar">
                            <div class="window-title">
                                <span class="brand-mark sm">
                                    <svg
                                        viewBox="0 0 32 32"
                                        fill="none"
                                        aria-hidden="true"
                                    >
                                        <circle
                                            cx="16"
                                            cy="16"
                                            r="13.5"
                                            stroke="currentColor"
                                            stroke-width="2.5"
                                        />
                                        <circle
                                            cx="16"
                                            cy="16"
                                            r="6.5"
                                            fill="url(#prism)"
                                        />
                                    </svg>
                                </span>
                                ChromaVale
                            </div>
                            <div class="window-tag">{{ t.hero.chips[0] }}</div>
                        </div>

                        <div class="window-body">
                            <div class="preview">
                                <div class="preview-sky"></div>
                                <div class="preview-grid">
                                    <span style="--c: #ef6f5e"></span>
                                    <span style="--c: #f2b04a"></span>
                                    <span style="--c: #e9d35b"></span>
                                    <span style="--c: #6cc08b"></span>
                                    <span style="--c: #4ea1d3"></span>
                                    <span style="--c: #7a6cd6"></span>
                                </div>
                                <div class="preview-label">
                                    {{ t.hero.preview }}
                                </div>
                            </div>

                            <div class="controls">
                                <div class="chips">
                                    <span
                                        v-for="(chip, i) in t.hero.chips"
                                        :key="chip"
                                        class="chip"
                                        :class="{ 'chip-on': i === 0 }"
                                    >
                                        {{ chip }}
                                    </span>
                                </div>

                                <div class="slider">
                                    <div class="slider-head">
                                        <span>{{ t.hero.temperature }}</span>
                                        <span class="slider-val">4 200 K</span>
                                    </div>
                                    <div class="track">
                                        <div
                                            class="fill warm"
                                            style="width: 62%"
                                        >
                                            <span class="knob"></span>
                                        </div>
                                    </div>
                                </div>

                                <div class="slider">
                                    <div class="slider-head">
                                        <span>{{ t.hero.saturation }}</span>
                                        <span class="slider-val">+8</span>
                                    </div>
                                    <div class="track">
                                        <div
                                            class="fill ink"
                                            style="width: 44%"
                                        >
                                            <span class="knob"></span>
                                        </div>
                                    </div>
                                </div>

                                <div class="channels">
                                    <div class="ch">
                                        <span class="ch-dot r"></span>
                                        <div class="ch-track">
                                            <div
                                                class="ch-fill r"
                                                style="width: 88%"
                                            ></div>
                                        </div>
                                    </div>
                                    <div class="ch">
                                        <span class="ch-dot g"></span>
                                        <div class="ch-track">
                                            <div
                                                class="ch-fill g"
                                                style="width: 72%"
                                            ></div>
                                        </div>
                                    </div>
                                    <div class="ch">
                                        <span class="ch-dot b"></span>
                                        <div class="ch-track">
                                            <div
                                                class="ch-fill b"
                                                style="width: 54%"
                                            ></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Stat band -->
            <section class="stats">
                <div v-for="s in t.stats" :key="s.num" class="stat">
                    <span class="stat-num">{{ s.num }}</span>
                    <span class="stat-label">{{ s.label }}</span>
                </div>
            </section>

            <!-- Features -->
            <section id="features" class="section">
                <div class="section-head">
                    <span class="kicker">{{ t.featuresHead.kicker }}</span>
                    <h2 class="heading">{{ t.featuresHead.heading }}</h2>
                    <p class="sub">{{ t.featuresHead.sub }}</p>
                </div>

                <div class="grid">
                    <article
                        v-for="(f, i) in t.features"
                        :key="f.title"
                        class="card"
                    >
                        <span class="card-ico">
                            <svg
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="1.6"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                aria-hidden="true"
                            >
                                <template v-if="featureIcons[i] === 'sliders'">
                                    <path d="M4 7h10M18 7h2M4 12h2M10 12h10M4 17h7M15 17h5" />
                                    <circle cx="16" cy="7" r="2" />
                                    <circle cx="8" cy="12" r="2" />
                                    <circle cx="13" cy="17" r="2" />
                                </template>
                                <template v-else-if="featureIcons[i] === 'layers'">
                                    <path d="M12 3 3 8l9 5 9-5-9-5Z" />
                                    <path d="M3 13l9 5 9-5" />
                                    <path d="M3 18l9 5 9-5" opacity="0.4" />
                                </template>
                                <template v-else-if="featureIcons[i] === 'window'">
                                    <rect x="3" y="4" width="18" height="16" rx="2.5" />
                                    <path d="M3 9h18" />
                                    <path d="M6.5 6.5h.01M9 6.5h.01" />
                                </template>
                                <template v-else-if="featureIcons[i] === 'command'">
                                    <path d="M9 6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6Z" />
                                </template>
                                <template v-else-if="featureIcons[i] === 'clock'">
                                    <circle cx="12" cy="12" r="9" />
                                    <path d="M12 7v5l3.5 2" />
                                </template>
                                <template v-else>
                                    <path d="M4 12h12" />
                                    <path d="M14 7l5 5-5 5" />
                                    <path d="M4 6v12" opacity="0.4" />
                                </template>
                            </svg>
                        </span>
                        <h3 class="card-title">{{ f.title }}</h3>
                        <p class="card-body">{{ f.body }}</p>
                    </article>
                </div>
            </section>

            <!-- Colorblind spotlight -->
            <section id="colorblind" class="spotlight">
                <div class="spotlight-inner">
                    <div class="spotlight-copy">
                        <span class="kicker grad">
                            {{ t.spotlight.kicker }}
                        </span>
                        <h2 class="heading">{{ t.spotlight.heading }}</h2>
                        <p class="sub">{{ t.spotlight.sub }}</p>
                        <ul class="check-list">
                            <li
                                v-for="c in t.spotlight.checks"
                                :key="c"
                            >
                                <span class="check"></span>{{ c }}
                            </li>
                        </ul>
                        <Link href="/try" class="btn btn-ink btn-lg">
                            {{ t.spotlight.cta }}
                        </Link>
                    </div>

                    <div class="demo">
                        <div class="demo-card">
                            <div class="demo-head">
                                <span class="demo-x">
                                    {{ t.spotlight.without }}
                                </span>
                            </div>
                            <div class="ramp ramp-flat">
                                <span style="--c: #8a8f3a"></span>
                                <span style="--c: #93923a"></span>
                                <span style="--c: #8f8c3c"></span>
                                <span style="--c: #87913e"></span>
                                <span style="--c: #8d8e3b"></span>
                            </div>
                            <p class="demo-note">
                                {{ t.spotlight.withoutNote }}
                            </p>
                        </div>
                        <div class="demo-arrow" aria-hidden="true">
                            <svg viewBox="0 0 24 24" fill="none">
                                <path
                                    d="M5 12h14m0 0-5-5m5 5-5 5"
                                    stroke="currentColor"
                                    stroke-width="1.8"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                />
                            </svg>
                        </div>
                        <div class="demo-card demo-card-on">
                            <div class="demo-head">
                                <span class="demo-x on">
                                    {{ t.spotlight.with }}
                                </span>
                            </div>
                            <div class="ramp">
                                <span style="--c: #d2603f"></span>
                                <span style="--c: #e0a23e"></span>
                                <span style="--c: #c9c84a"></span>
                                <span style="--c: #5aa66a"></span>
                                <span style="--c: #2f8fb0"></span>
                            </div>
                            <p class="demo-note">{{ t.spotlight.withNote }}</p>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Comparison -->
            <section class="section">
                <div class="section-head">
                    <span class="kicker">{{ t.compare.kicker }}</span>
                    <h2 class="heading">{{ t.compare.heading }}</h2>
                    <p class="sub">{{ t.compare.sub }}</p>
                </div>

                <div class="table">
                    <div class="table-row table-head">
                        <span></span>
                        <span class="col">{{ t.compare.cols[0] }}</span>
                        <span class="col">{{ t.compare.cols[1] }}</span>
                        <span class="col col-chroma">
                            {{ t.compare.cols[2] }}
                        </span>
                    </div>
                    <div
                        v-for="row in t.compare.rows"
                        :key="row.label"
                        class="table-row"
                    >
                        <span class="row-label">{{ row.label }}</span>
                        <span class="col">
                            <template v-if="row.windows === false">
                                <span class="mark no">&times;</span>
                            </template>
                            <template v-else>{{ row.windows }}</template>
                        </span>
                        <span class="col">
                            <template v-if="row.flux === false">
                                <span class="mark no">&times;</span>
                            </template>
                            <template v-else>{{ row.flux }}</template>
                        </span>
                        <span class="col col-chroma">
                            <template v-if="row.chroma === true">
                                <span class="mark yes">
                                    <svg viewBox="0 0 16 16" fill="none">
                                        <path
                                            d="M3 8.5l3 3 7-7"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                        />
                                    </svg>
                                </span>
                            </template>
                            <template v-else>{{ row.chroma }}</template>
                        </span>
                    </div>
                </div>
            </section>

            <!-- Pricing -->
            <section id="pricing" class="section">
                <div class="section-head">
                    <span class="kicker">{{ t.pricing.kicker }}</span>
                    <h2 class="heading">{{ t.pricing.heading }}</h2>
                    <p class="sub">{{ t.pricing.sub }}</p>
                </div>

                <div class="pricing">
                    <article class="plan">
                        <div class="plan-top">
                            <h3 class="plan-name">{{ t.pricing.free.name }}</h3>
                            <p class="plan-desc">{{ t.pricing.free.desc }}</p>
                        </div>
                        <div class="plan-price">
                            <span class="amount">
                                {{ t.pricing.free.amount }}
                            </span>
                            <span class="per">{{ t.pricing.free.per }}</span>
                        </div>
                        <ul class="plan-list">
                            <li
                                v-for="p in t.pricing.free.points"
                                :key="p"
                            >
                                <span class="check"></span>{{ p }}
                            </li>
                        </ul>
                        <Link href="/download" class="btn btn-soft btn-block btn-lg">
                            {{ t.pricing.free.cta }}
                        </Link>
                    </article>

                    <article class="plan plan-pro">
                        <div class="plan-badge">
                            {{ t.pricing.recommended }}
                        </div>
                        <div class="plan-top">
                            <h3 class="plan-name">{{ t.pricing.pro.name }}</h3>
                            <p class="plan-desc">{{ t.pricing.pro.desc }}</p>
                        </div>
                        <div class="plan-price">
                            <span class="amount">
                                {{ t.pricing.pro.amount }}
                            </span>
                            <span class="per">{{ t.pricing.pro.per }}</span>
                        </div>
                        <ul class="plan-list">
                            <li
                                v-for="p in t.pricing.pro.points"
                                :key="p"
                            >
                                <span class="check on"></span>{{ p }}
                            </li>
                        </ul>
                        <Link href="/download" class="btn btn-accent btn-block btn-lg">
                            {{ t.pricing.pro.cta }}
                        </Link>
                        <p class="plan-fine">{{ t.pricing.pro.fine }}</p>
                    </article>
                </div>
            </section>

            <!-- Final CTA -->
            <section class="cta">
                <div class="cta-card">
                    <h2 class="cta-title">{{ t.cta.title }}</h2>
                    <p class="cta-sub">{{ t.cta.sub }}</p>
                    <div class="cta-actions">
                        <Link href="/download" class="btn btn-light btn-lg">
                            <svg
                                viewBox="0 0 16 16"
                                fill="currentColor"
                                class="btn-ico"
                                aria-hidden="true"
                            >
                                <path
                                    d="M11.18 8.5c-.02-1.5 1.22-2.22 1.28-2.26-.7-1.02-1.79-1.16-2.18-1.18-.93-.09-1.81.54-2.28.54-.47 0-1.2-.53-1.97-.51-1.01.01-1.94.59-2.46 1.49-1.05 1.82-.27 4.51.75 5.99.5.72 1.1 1.53 1.88 1.5.75-.03 1.04-.49 1.95-.49.91 0 1.17.49 1.97.47.81-.01 1.33-.74 1.83-1.46.58-.84.82-1.65.83-1.69-.02-.01-1.6-.61-1.62-2.43Zm-1.5-4.47c.41-.5.69-1.2.61-1.9-.59.02-1.31.39-1.74.89-.38.44-.72 1.15-.63 1.83.66.05 1.34-.33 1.76-.82Z"
                                />
                            </svg>
                            {{ t.cta.mac }}
                        </Link>
                        <Link href="/download" class="btn btn-light btn-lg">
                            <svg
                                viewBox="0 0 16 16"
                                fill="currentColor"
                                class="btn-ico"
                                aria-hidden="true"
                            >
                                <path
                                    d="M0 2.2 6.5 1.3v6.2H0V2.2Zm0 11.6 6.5.9V8.5H0v5.3ZM7.3 1.2 16 0v7.5H7.3V1.2Zm0 13.6L16 16V8.5H7.3v6.3Z"
                                />
                            </svg>
                            {{ t.cta.win }}
                        </Link>
                    </div>
                </div>
            </section>
        </main>

        <footer class="footer">
            <div class="footer-inner">
                <div class="footer-brand">
                    <span class="brand-mark sm">
                        <svg viewBox="0 0 32 32" fill="none" aria-hidden="true">
                            <circle
                                cx="16"
                                cy="16"
                                r="13.5"
                                stroke="currentColor"
                                stroke-width="2.5"
                            />
                            <circle cx="16" cy="16" r="6.5" fill="url(#prism)" />
                        </svg>
                    </span>
                    <span class="brand-name">ChromaVale</span>
                </div>
                <p class="footer-line">{{ t.footer.line }}</p>
                <p class="footer-copy">
                    &copy; {{ new Date().getFullYear() }} ChromaVale
                </p>
            </div>
        </footer>
    </div>
</template>

<style scoped>
/* ---------- Foundations ---------- */
.page {
    --ink: #16161a;
    --ink-soft: #3f3f46;
    --muted: #71717a;
    --line: #ececef;
    --line-strong: #e0e0e4;
    --paper: #ffffff;
    --paper-2: #fbfbfc;
    --accent: #5b4be0;
    --accent-dark: #3f31bd;

    position: relative;
    min-height: 100vh;
    background: var(--paper);
    color: var(--ink);
    font-family:
        'Instrument Sans', ui-sans-serif, system-ui, sans-serif;
    -webkit-font-smoothing: antialiased;
    overflow-x: clip;
}

.halo {
    position: absolute;
    inset: 0 0 auto 0;
    height: 620px;
    background:
        radial-gradient(
            900px 380px at 50% -120px,
            rgba(91, 75, 224, 0.06),
            transparent 70%
        );
    pointer-events: none;
}

main,
.nav,
.footer-inner {
    margin-inline: auto;
    max-width: 1120px;
    padding-inline: 24px;
}

/* ---------- Buttons (tactile) ---------- */
.btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font-weight: 600;
    letter-spacing: -0.01em;
    border-radius: 14px;
    padding: 0.7rem 1.15rem;
    font-size: 0.95rem;
    line-height: 1;
    cursor: pointer;
    user-select: none;
    text-decoration: none;
    white-space: nowrap;
    border: none;
    transition:
        transform 0.11s cubic-bezier(0.2, 0.8, 0.2, 1),
        box-shadow 0.11s cubic-bezier(0.2, 0.8, 0.2, 1),
        background 0.15s ease,
        filter 0.15s ease;
    will-change: transform;
}
.btn:active {
    transform: translateY(var(--press, 4px));
}
.btn-ico {
    width: 1.05em;
    height: 1.05em;
}

.btn-sm {
    padding: 0.55rem 0.95rem;
    font-size: 0.875rem;
    border-radius: 12px;
    --press: 3px;
}
.btn-lg {
    padding: 0.95rem 1.5rem;
    font-size: 1rem;
    --press: 5px;
}
.btn-block {
    width: 100%;
}

.btn-ink {
    background: var(--ink);
    color: #fff;
    box-shadow: 0 var(--press, 4px) 0 0 #000000;
}
.btn-ink:hover {
    background: #26262e;
}
.btn-ink:active {
    box-shadow: 0 0 0 0 #000000;
}

.btn-accent {
    background: var(--accent);
    color: #fff;
    box-shadow: 0 var(--press, 4px) 0 0 var(--accent-dark);
}
.btn-accent:hover {
    filter: brightness(1.06);
}
.btn-accent:active {
    box-shadow: 0 0 0 0 var(--accent-dark);
}

.btn-soft {
    background: #fff;
    color: var(--ink);
    box-shadow:
        0 var(--press, 4px) 0 0 var(--line-strong),
        inset 0 0 0 1px var(--line-strong);
}
.btn-soft:hover {
    background: var(--paper-2);
}
.btn-soft:active {
    box-shadow: inset 0 0 0 1px var(--line-strong);
}

.btn-light {
    background: #fff;
    color: var(--ink);
    box-shadow: 0 var(--press, 4px) 0 0 rgba(0, 0, 0, 0.55);
}
.btn-light:hover {
    background: #f4f4f6;
}
.btn-light:active {
    box-shadow: 0 0 0 0 rgba(0, 0, 0, 0.55);
}

/* ---------- Nav ---------- */
.nav-wrap {
    position: sticky;
    top: 0;
    z-index: 40;
    background: rgba(255, 255, 255, 0.72);
    backdrop-filter: saturate(1.4) blur(14px);
    border-bottom: 1px solid transparent;
    transition: border-color 0.2s ease;
}
.nav-wrap:hover {
    border-bottom-color: var(--line);
}
.nav {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 72px;
    gap: 1.5rem;
}
.brand {
    display: inline-flex;
    align-items: center;
    gap: 0.6rem;
    text-decoration: none;
    color: var(--ink);
}
.brand-mark {
    display: grid;
    place-items: center;
    width: 30px;
    height: 30px;
    color: var(--ink);
}
.brand-mark svg {
    width: 100%;
    height: 100%;
}
.brand-mark.sm {
    width: 22px;
    height: 22px;
}
.brand-name {
    font-weight: 600;
    font-size: 1.1rem;
    letter-spacing: -0.02em;
}
.nav-links {
    display: none;
    gap: 2rem;
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--ink-soft);
}
.nav-links a {
    text-decoration: none;
    color: inherit;
    transition: color 0.15s ease;
}
.nav-links a:hover {
    color: var(--ink);
}
.nav-actions {
    display: flex;
    align-items: center;
    gap: 0.9rem;
}
.text-link {
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--ink-soft);
    text-decoration: none;
}
.text-link:hover {
    color: var(--ink);
}

/* ---------- Language switcher ---------- */
.lang {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    padding: 3px;
    border-radius: 11px;
    background: var(--paper-2);
    border: 1px solid var(--line);
}
.lang-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.28rem 0.5rem;
    border: none;
    background: transparent;
    border-radius: 8px;
    cursor: pointer;
    font: inherit;
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--muted);
    transition:
        background 0.15s ease,
        color 0.15s ease,
        box-shadow 0.15s ease;
}
.lang-btn:hover {
    color: var(--ink);
}
.lang-btn.active {
    color: var(--ink);
    background: #fff;
    box-shadow: 0 1px 2px rgba(16, 16, 24, 0.1);
}
.lang-btn svg {
    width: 18px;
    height: 12.6px;
    border-radius: 2px;
    display: block;
    box-shadow: inset 0 0 0 0.5px rgba(0, 0, 0, 0.12);
}

.text-link {
    display: none;
}

@media (min-width: 880px) {
    .nav-links {
        display: flex;
    }
    .text-link {
        display: inline;
    }
}

/* ---------- Hero ---------- */
.hero {
    display: grid;
    gap: 3rem;
    padding-top: 4rem;
    padding-bottom: 4.5rem;
}
.platforms {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.76rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.13em;
    color: var(--muted);
}
.plat-ico {
    width: 14px;
    height: 14px;
    color: var(--ink-soft);
}
.platforms span {
    margin-left: 0.15rem;
}
.display {
    margin-top: 1.4rem;
    font-size: clamp(2.6rem, 6vw, 4.1rem);
    line-height: 1.02;
    letter-spacing: -0.035em;
    font-weight: 600;
}
.ink-grad {
    background: linear-gradient(
        100deg,
        #ef6f5e,
        #f2b04a 26%,
        #5bd6a0 52%,
        #4da6ff 76%,
        #9b6be5
    );
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
}
.lede {
    margin-top: 1.5rem;
    max-width: 33rem;
    font-size: 1.12rem;
    line-height: 1.6;
    color: var(--ink-soft);
}
.hero-cta {
    margin-top: 2rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.85rem;
}
.hero-fine {
    margin-top: 1.25rem;
    font-size: 0.85rem;
    color: var(--muted);
}

@media (min-width: 940px) {
    .hero {
        grid-template-columns: 1.05fr 0.95fr;
        align-items: center;
        gap: 3.5rem;
        padding-top: 5.5rem;
        padding-bottom: 6rem;
    }
}

/* ---------- App window mockup ---------- */
.hero-visual {
    perspective: 1600px;
}
.window {
    border-radius: 20px;
    background: #fff;
    border: 1px solid var(--line-strong);
    box-shadow:
        0 1px 1px rgba(16, 16, 24, 0.04),
        0 18px 40px -18px rgba(16, 16, 24, 0.22),
        0 50px 90px -40px rgba(91, 75, 224, 0.18);
    overflow: hidden;
}
.window-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.85rem 1.1rem;
    border-bottom: 1px solid var(--line);
    background: var(--paper-2);
}
.window-title {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 600;
    font-size: 0.92rem;
    letter-spacing: -0.01em;
}
.window-tag {
    font-size: 0.74rem;
    font-weight: 600;
    color: var(--accent);
    background: rgba(91, 75, 224, 0.09);
    padding: 0.25rem 0.6rem;
    border-radius: 999px;
}
.window-body {
    display: grid;
    grid-template-columns: 0.85fr 1fr;
    gap: 1rem;
    padding: 1.1rem;
}
.preview {
    position: relative;
    border-radius: 14px;
    overflow: hidden;
    border: 1px solid var(--line);
    min-height: 200px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
}
.preview-sky {
    position: absolute;
    inset: 0;
    background: linear-gradient(
        160deg,
        #ffe7c2,
        #ffd3a3 38%,
        #f4b98a 70%,
        #e7a07e
    );
}
.preview-grid {
    position: relative;
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 5px;
    padding: 0.5rem;
}
.preview-grid span {
    aspect-ratio: 1;
    border-radius: 5px;
    background: var(--c);
    filter: saturate(0.86) sepia(0.12);
}
.preview-label {
    position: relative;
    padding: 0.4rem 0.6rem 0.55rem;
    font-size: 0.72rem;
    font-weight: 600;
    color: #7c5a3a;
}
.controls {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
}
.chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
}
.chip {
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--ink-soft);
    background: var(--paper-2);
    border: 1px solid var(--line);
    padding: 0.28rem 0.6rem;
    border-radius: 999px;
}
.chip-on {
    color: #fff;
    background: var(--ink);
    border-color: var(--ink);
}
.slider-head {
    display: flex;
    justify-content: space-between;
    font-size: 0.76rem;
    font-weight: 600;
    margin-bottom: 0.4rem;
}
.slider-val {
    color: var(--muted);
}
.track {
    height: 8px;
    border-radius: 999px;
    background: var(--line);
}
.fill {
    position: relative;
    height: 100%;
    border-radius: 999px;
}
.fill.warm {
    background: linear-gradient(90deg, #f2b04a, #ef6f5e);
}
.fill.ink {
    background: var(--ink);
}
.knob {
    position: absolute;
    right: -6px;
    top: 50%;
    transform: translateY(-50%);
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #fff;
    border: 1px solid var(--line-strong);
    box-shadow: 0 2px 5px rgba(16, 16, 24, 0.18);
}
.channels {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 0.15rem;
}
.ch {
    display: flex;
    align-items: center;
    gap: 0.55rem;
}
.ch-dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    flex: none;
}
.ch-dot.r {
    background: #ef6f5e;
}
.ch-dot.g {
    background: #5bd6a0;
}
.ch-dot.b {
    background: #4da6ff;
}
.ch-track {
    flex: 1;
    height: 6px;
    border-radius: 999px;
    background: var(--line);
    overflow: hidden;
}
.ch-fill {
    height: 100%;
    border-radius: 999px;
}
.ch-fill.r {
    background: #ef6f5e;
}
.ch-fill.g {
    background: #5bd6a0;
}
.ch-fill.b {
    background: #4da6ff;
}

/* ---------- Stats ---------- */
.stats {
    display: grid;
    gap: 1.5rem;
    padding: 2.5rem 0;
    border-top: 1px solid var(--line);
    border-bottom: 1px solid var(--line);
}
.stat {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}
.stat-num {
    font-size: 2.2rem;
    font-weight: 600;
    letter-spacing: -0.03em;
}
.stat-label {
    font-size: 0.95rem;
    line-height: 1.5;
    color: var(--muted);
    max-width: 24rem;
}
@media (min-width: 800px) {
    .stats {
        grid-template-columns: repeat(3, 1fr);
        gap: 2.5rem;
    }
}

/* ---------- Sections ---------- */
.section {
    padding: 5rem 0;
}
.section-head {
    max-width: 40rem;
    margin-bottom: 3rem;
}
.kicker {
    display: inline-block;
    font-size: 0.78rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--muted);
    margin-bottom: 0.9rem;
}
.kicker.grad {
    background: linear-gradient(100deg, #ef6f5e, #5bd6a0 50%, #4da6ff);
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
}
.heading {
    font-size: clamp(1.9rem, 3.6vw, 2.7rem);
    line-height: 1.08;
    letter-spacing: -0.03em;
    font-weight: 600;
}
.sub {
    margin-top: 1.1rem;
    font-size: 1.08rem;
    line-height: 1.6;
    color: var(--ink-soft);
}

/* ---------- Feature grid ---------- */
.grid {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
}
.card {
    padding: 1.6rem;
    border-radius: 18px;
    background: var(--paper-2);
    border: 1px solid var(--line);
    transition:
        transform 0.18s ease,
        border-color 0.18s ease,
        box-shadow 0.18s ease;
}
.card:hover {
    transform: translateY(-3px);
    border-color: var(--line-strong);
    box-shadow: 0 14px 30px -20px rgba(16, 16, 24, 0.25);
}
.card-ico {
    display: grid;
    place-items: center;
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: #fff;
    border: 1px solid var(--line);
    color: var(--ink);
    margin-bottom: 1.1rem;
}
.card-ico svg {
    width: 22px;
    height: 22px;
}
.card-title {
    font-size: 1.1rem;
    font-weight: 600;
    letter-spacing: -0.01em;
    margin-bottom: 0.5rem;
}
.card-body {
    font-size: 0.95rem;
    line-height: 1.55;
    color: var(--muted);
}
@media (min-width: 720px) {
    .grid {
        grid-template-columns: repeat(2, 1fr);
    }
}
@media (min-width: 1000px) {
    .grid {
        grid-template-columns: repeat(3, 1fr);
    }
}

/* ---------- Spotlight ---------- */
.spotlight {
    padding: 1rem 0 1.5rem;
}
.spotlight-inner {
    display: grid;
    gap: 3rem;
    padding: 3.5rem;
    border-radius: 28px;
    background:
        radial-gradient(
            120% 120% at 100% 0%,
            rgba(91, 75, 224, 0.05),
            transparent 55%
        ),
        var(--paper-2);
    border: 1px solid var(--line);
}
.check-list {
    list-style: none;
    margin: 1.6rem 0 2rem;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
}
.check-list li {
    display: flex;
    align-items: center;
    gap: 0.7rem;
    font-size: 1rem;
    color: var(--ink-soft);
    font-weight: 500;
}
.check {
    flex: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: rgba(91, 75, 224, 0.1);
    position: relative;
}
.check::after {
    content: '';
    position: absolute;
    left: 6px;
    top: 5px;
    width: 5px;
    height: 9px;
    border: solid var(--accent);
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
}
.demo {
    display: grid;
    gap: 1rem;
    align-content: center;
}
.demo-card {
    border-radius: 18px;
    background: #fff;
    border: 1px solid var(--line);
    padding: 1.25rem;
}
.demo-card-on {
    border-color: rgba(91, 75, 224, 0.3);
    box-shadow: 0 16px 40px -26px rgba(91, 75, 224, 0.5);
}
.demo-head {
    margin-bottom: 0.9rem;
}
.demo-x {
    font-size: 0.78rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--muted);
}
.demo-x.on {
    color: var(--accent);
}
.ramp {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 6px;
    height: 56px;
}
.ramp span {
    border-radius: 8px;
    background: var(--c);
}
.ramp-flat span {
    filter: saturate(0.7);
}
.demo-note {
    margin-top: 0.8rem;
    font-size: 0.86rem;
    color: var(--muted);
}
.demo-arrow {
    display: grid;
    place-items: center;
    color: var(--accent);
}
.demo-arrow svg {
    width: 24px;
    height: 24px;
    transform: rotate(90deg);
}

@media (min-width: 920px) {
    .spotlight-inner {
        grid-template-columns: 1fr 1fr;
        align-items: center;
        gap: 4rem;
        padding: 4rem;
    }
    .demo {
        grid-template-columns: 1fr auto 1fr;
        align-items: center;
    }
    .demo-arrow svg {
        transform: none;
    }
}

/* ---------- Comparison table ---------- */
.table {
    border: 1px solid var(--line);
    border-radius: 20px;
    overflow: hidden;
}
.table-row {
    display: grid;
    grid-template-columns: 1.3fr 1fr 1fr 1.2fr;
    align-items: center;
    gap: 0.75rem;
    padding: 1.05rem 1.25rem;
    border-top: 1px solid var(--line);
    font-size: 0.92rem;
}
.table-row:first-child {
    border-top: none;
}
.table-head {
    background: var(--paper-2);
    font-weight: 600;
    color: var(--ink-soft);
}
.row-label {
    font-weight: 600;
    color: var(--ink);
}
.col {
    color: var(--muted);
    line-height: 1.4;
}
.col-chroma {
    color: var(--ink);
    font-weight: 500;
}
.table-head .col-chroma {
    color: var(--accent);
    font-weight: 700;
}
.mark {
    display: inline-grid;
    place-items: center;
    width: 22px;
    height: 22px;
    border-radius: 50%;
}
.mark.yes {
    background: var(--accent);
    color: #fff;
}
.mark.yes svg {
    width: 14px;
    height: 14px;
}
.mark.no {
    color: #c3c3c9;
    font-size: 1.1rem;
}
@media (max-width: 720px) {
    .table-row {
        grid-template-columns: 1fr;
        gap: 0.35rem;
    }
    .table-head {
        display: none;
    }
    .row-label {
        margin-bottom: 0.3rem;
        font-size: 1rem;
    }
}

/* ---------- Pricing ---------- */
.pricing {
    display: grid;
    gap: 1.25rem;
    grid-template-columns: 1fr;
    max-width: 56rem;
    margin-inline: auto;
}
.plan {
    position: relative;
    padding: 2rem;
    border-radius: 22px;
    background: #fff;
    border: 1px solid var(--line);
    display: flex;
    flex-direction: column;
}
.plan-pro {
    border-color: rgba(91, 75, 224, 0.35);
    box-shadow:
        0 1px 1px rgba(16, 16, 24, 0.03),
        0 30px 60px -40px rgba(91, 75, 224, 0.5);
}
.plan-badge {
    position: absolute;
    top: -0.7rem;
    right: 1.6rem;
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: #fff;
    background: var(--accent);
    padding: 0.32rem 0.7rem;
    border-radius: 999px;
}
.plan-name {
    font-size: 1.3rem;
    font-weight: 600;
    letter-spacing: -0.02em;
}
.plan-desc {
    margin-top: 0.3rem;
    font-size: 0.92rem;
    color: var(--muted);
}
.plan-price {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
    margin: 1.5rem 0;
}
.amount {
    font-size: 2.6rem;
    font-weight: 600;
    letter-spacing: -0.03em;
}
.per {
    font-size: 0.9rem;
    color: var(--muted);
}
.plan-list {
    list-style: none;
    margin: 0 0 1.75rem;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.8rem;
    flex: 1;
}
.plan-list li {
    display: flex;
    align-items: flex-start;
    gap: 0.65rem;
    font-size: 0.95rem;
    line-height: 1.4;
    color: var(--ink-soft);
}
.plan-list .check {
    margin-top: 1px;
}
.check.on {
    background: var(--accent);
}
.check.on::after {
    border-color: #fff;
}
.plan-fine {
    margin-top: 0.9rem;
    text-align: center;
    font-size: 0.82rem;
    color: var(--muted);
}
@media (min-width: 800px) {
    .pricing {
        grid-template-columns: 1fr 1fr;
        align-items: start;
    }
}

/* ---------- Final CTA ---------- */
.cta {
    padding: 3rem 0 6rem;
}
.cta-card {
    text-align: center;
    padding: 4rem 2rem;
    border-radius: 28px;
    background:
        radial-gradient(
            120% 140% at 50% 0%,
            #2a2a33,
            #131317 70%
        );
    color: #fff;
}
.cta-title {
    font-size: clamp(1.8rem, 4vw, 2.6rem);
    line-height: 1.1;
    letter-spacing: -0.03em;
    font-weight: 600;
    max-width: 22ch;
    margin-inline: auto;
}
.cta-sub {
    margin-top: 1rem;
    font-size: 1.05rem;
    color: rgba(255, 255, 255, 0.66);
}
.cta-actions {
    margin-top: 2rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.85rem;
    justify-content: center;
}

/* ---------- Footer ---------- */
.footer {
    border-top: 1px solid var(--line);
    padding: 3rem 0;
}
.footer-inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
}
.footer-brand {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--ink);
}
.footer-line {
    font-size: 0.95rem;
    color: var(--muted);
}
.footer-copy {
    font-size: 0.85rem;
    color: #a1a1aa;
}
</style>
