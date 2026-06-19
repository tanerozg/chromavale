<script setup lang="ts">
import { Head, Link } from '@inertiajs/vue3';
import { reactive, ref, computed } from 'vue';

/* ---- Engine parameters ---- */
const params = reactive({
    temp: 0, // -100 (cool) .. 100 (warm)
    sat: 100, // 0 .. 200
    contrast: 100, // 50 .. 150
    bright: 100, // 50 .. 150
    r: 100, // 0 .. 200
    g: 100,
    b: 100,
});

const sliders = [
    { key: 'temp', label: 'Temperature', min: -100, max: 100, suffix: '' },
    { key: 'sat', label: 'Saturation', min: 0, max: 200, suffix: '%' },
    { key: 'contrast', label: 'Contrast', min: 50, max: 150, suffix: '%' },
    { key: 'bright', label: 'Brightness', min: 50, max: 150, suffix: '%' },
] as const;

const channels = [
    { key: 'r', label: 'Red', cls: 'r' },
    { key: 'g', label: 'Green', cls: 'g' },
    { key: 'b', label: 'Blue', cls: 'b' },
] as const;

type ParamKey = keyof typeof params;

/* ---- Engine -> SVG feComponentTransfer / saturate ---- */
const satValue = computed(() => (params.sat / 100).toFixed(3));

const warm = computed(() => params.temp / 100);
const effR = computed(() => (params.r / 100) * (1 + warm.value * 0.25));
const effG = computed(() => params.g / 100);
const effB = computed(() => (params.b / 100) * (1 - warm.value * 0.25));

const contrastF = computed(() => params.contrast / 100);
const brightF = computed(() => params.bright / 100);
const intercept = computed(() =>
    (0.5 * (1 - contrastF.value) + (brightF.value - 1) * 0.5).toFixed(4),
);
const slopeR = computed(() => (effR.value * contrastF.value).toFixed(4));
const slopeG = computed(() => (effG.value * contrastF.value).toFixed(4));
const slopeB = computed(() => (effB.value * contrastF.value).toFixed(4));

/* ---- Color blindness ---- */
type Cb = 'none' | 'protan' | 'deutan' | 'tritan';
const cbType = ref<Cb>('none');
const correction = ref(true);

const cbOptions = [
    { key: 'none', label: 'Normal vision' },
    { key: 'protan', label: 'Protanopia' },
    { key: 'deutan', label: 'Deuteranopia' },
    { key: 'tritan', label: 'Tritanopia' },
] as const;

// Standard color-blindness simulation matrices.
const simMatrices: Record<Exclude<Cb, 'none'>, string> = {
    protan:
        '0.567 0.433 0 0 0  0.558 0.442 0 0 0  0 0.242 0.758 0 0  0 0 0 1 0',
    deutan:
        '0.625 0.375 0 0 0  0.70 0.30 0 0 0  0 0.30 0.70 0 0  0 0 0 1 0',
    tritan:
        '0.95 0.05 0 0 0  0 0.433 0.567 0 0  0 0.475 0.525 0 0  0 0 0 1 0',
};

// Daltonization correction matrices (redistribute lost contrast into
// channels the viewer can still perceive).
const fixMatrices: Record<Exclude<Cb, 'none'>, string> = {
    protan:
        '1 0 0 0 0  -0.2549 1.2549 0 0 0  0.3031 -0.5451 1.242 0 0  0 0 0 1 0',
    deutan:
        '1 0 0 0 0  -0.4375 1.4375 0 0 0  0.2625 -0.5625 1.3 0 0  0 0 0 1 0',
    tritan:
        '1.05 -0.3825 0.3325 0 0  0 1.2345 -0.2345 0 0  0 0 1 0 0  0 0 0 1 0',
};

const simMatrix = computed(() =>
    cbType.value === 'none' ? '' : simMatrices[cbType.value],
);
const fixMatrix = computed(() =>
    cbType.value === 'none' ? '' : fixMatrices[cbType.value],
);
const showCorrection = computed(
    () => cbType.value !== 'none' && correction.value,
);

/* ---- Presets ---- */
function set(values: Partial<typeof params>) {
    Object.assign(params, values);
}
const presets = [
    {
        name: 'Comfort',
        apply: () =>
            set({ temp: 45, sat: 96, contrast: 100, bright: 100, r: 100, g: 100, b: 100 }),
    },
    {
        name: 'Reading',
        apply: () =>
            set({ temp: 30, sat: 88, contrast: 104, bright: 98, r: 100, g: 100, b: 96 }),
    },
    {
        name: 'Night',
        apply: () =>
            set({ temp: 82, sat: 80, contrast: 96, bright: 88, r: 100, g: 92, b: 68 }),
    },
    {
        name: 'Vivid',
        apply: () =>
            set({ temp: 0, sat: 146, contrast: 108, bright: 102, r: 100, g: 100, b: 100 }),
    },
    {
        name: 'Grayscale',
        apply: () =>
            set({ temp: 0, sat: 0, contrast: 100, bright: 100, r: 100, g: 100, b: 100 }),
    },
];
function reset() {
    set({ temp: 0, sat: 100, contrast: 100, bright: 100, r: 100, g: 100, b: 100 });
}

function fmt(key: ParamKey, suffix: string) {
    const v = params[key];
    if (key === 'temp') return v === 0 ? 'Neutral' : v > 0 ? `Warm +${v}` : `Cool ${v}`;
    return `${v}${suffix}`;
}
</script>

<template>
    <Head title="Try ChromaVale - Live color demo" />

    <!-- Live color pipeline. Applied to the scene via filter: url(#cv) -->
    <svg class="cv-defs" aria-hidden="true">
        <filter id="cv" color-interpolation-filters="sRGB">
            <feColorMatrix type="saturate" :values="satValue" />
            <feComponentTransfer>
                <feFuncR type="linear" :slope="slopeR" :intercept="intercept" />
                <feFuncG type="linear" :slope="slopeG" :intercept="intercept" />
                <feFuncB type="linear" :slope="slopeB" :intercept="intercept" />
            </feComponentTransfer>
            <feColorMatrix
                v-if="showCorrection"
                type="matrix"
                :values="fixMatrix"
            />
            <feColorMatrix
                v-if="cbType !== 'none'"
                type="matrix"
                :values="simMatrix"
            />
        </filter>
    </svg>

    <div class="page">
        <header class="bar">
            <Link href="/" class="brand">
                <span class="brand-mark">
                    <svg viewBox="0 0 32 32" fill="none" aria-hidden="true">
                        <circle
                            cx="16"
                            cy="16"
                            r="13.5"
                            stroke="currentColor"
                            stroke-width="2.5"
                        />
                        <circle cx="16" cy="16" r="6.5" fill="url(#prism)" />
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
            </Link>
            <Link href="/" class="back">
                <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                    <path
                        d="M12 5l-5 5 5 5"
                        stroke="currentColor"
                        stroke-width="1.7"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    />
                </svg>
                Back to site
            </Link>
        </header>

        <main class="wrap">
            <div class="intro">
                <span class="kicker">Live browser demo</span>
                <h1 class="title">See your screen, the way you want it.</h1>
                <p class="lede">
                    Every slider recolors the scene in real time, the same way
                    ChromaVale recolors your whole screen. Switch on a
                    color-blindness type to experience it, then let the
                    correction pull the colors back apart.
                </p>
            </div>

            <div class="studio">
                <!-- Scene -->
                <div class="stage">
                    <div class="scene" style="filter: url(#cv)">
                        <div class="scene-sky"></div>
                        <div class="scene-sun"></div>
                        <div class="scene-hills">
                            <span class="hill h1"></span>
                            <span class="hill h2"></span>
                            <span class="hill h3"></span>
                        </div>
                        <div class="scene-card">
                            <div class="sc-row">
                                <span class="sc-title">Palette</span>
                                <span class="sc-pill">Live</span>
                            </div>
                            <div class="swatches">
                                <span style="--c: #e23b3b">Red</span>
                                <span style="--c: #ef7d2e">Orange</span>
                                <span style="--c: #e9c233">Yellow</span>
                                <span style="--c: #36a85a">Green</span>
                                <span style="--c: #1fa3a3">Teal</span>
                                <span style="--c: #2f6fe0">Blue</span>
                                <span style="--c: #8a4fd6">Violet</span>
                                <span style="--c: #d23f8c">Magenta</span>
                            </div>
                            <p class="sc-text">
                                Red and green sit side by side. Tell them apart?
                            </p>
                        </div>
                    </div>
                    <div class="stage-foot">
                        <span class="dot-key r"></span>R {{ params.r }}%
                        <span class="dot-key g"></span>G {{ params.g }}%
                        <span class="dot-key b"></span>B {{ params.b }}%
                    </div>
                </div>

                <!-- Controls -->
                <aside class="panel">
                    <div class="block">
                        <div class="block-head">
                            <h2>Color engine</h2>
                            <button class="link-btn" @click="reset">
                                Reset
                            </button>
                        </div>

                        <div class="presets">
                            <button
                                v-for="p in presets"
                                :key="p.name"
                                class="preset"
                                @click="p.apply"
                            >
                                {{ p.name }}
                            </button>
                        </div>

                        <div
                            v-for="s in sliders"
                            :key="s.key"
                            class="ctrl"
                        >
                            <div class="ctrl-head">
                                <label :for="s.key">{{ s.label }}</label>
                                <span class="ctrl-val">
                                    {{ fmt(s.key, s.suffix) }}
                                </span>
                            </div>
                            <input
                                :id="s.key"
                                v-model.number="params[s.key]"
                                type="range"
                                :min="s.min"
                                :max="s.max"
                            />
                        </div>

                        <div class="channels-head">Per channel</div>
                        <div
                            v-for="c in channels"
                            :key="c.key"
                            class="ctrl"
                        >
                            <div class="ctrl-head">
                                <label :for="c.key">
                                    <span class="ch-dot" :class="c.cls"></span>
                                    {{ c.label }}
                                </label>
                                <span class="ctrl-val">
                                    {{ params[c.key] }}%
                                </span>
                            </div>
                            <input
                                :id="c.key"
                                v-model.number="params[c.key]"
                                type="range"
                                min="0"
                                max="200"
                                :class="['range', c.cls]"
                            />
                        </div>
                    </div>

                    <div class="block">
                        <div class="block-head">
                            <h2>Color blindness</h2>
                        </div>
                        <div class="seg">
                            <button
                                v-for="o in cbOptions"
                                :key="o.key"
                                class="seg-btn"
                                :class="{ active: cbType === o.key }"
                                @click="cbType = o.key"
                            >
                                {{ o.label }}
                            </button>
                        </div>

                        <label
                            class="toggle"
                            :class="{ disabled: cbType === 'none' }"
                        >
                            <input
                                v-model="correction"
                                type="checkbox"
                                :disabled="cbType === 'none'"
                            />
                            <span class="switch"></span>
                            <span class="toggle-text">
                                Apply ChromaVale correction
                            </span>
                        </label>
                        <p class="hint">
                            {{
                                cbType === 'none'
                                    ? 'Pick a type to simulate how those colors collapse, then compare with correction on.'
                                    : correction
                                      ? 'Correction is on. The confusing tones are pushed back apart.'
                                      : 'Simulation only. Notice how the swatches blur together.'
                            }}
                        </p>
                    </div>

                    <Link href="/#pricing" class="cta">Download ChromaVale</Link>
                    <p class="disclaimer">
                        A browser demo for illustration. The desktop app works
                        across your entire screen and every app.
                    </p>
                </aside>
            </div>
        </main>
    </div>
</template>

<style scoped>
.page {
    --ink: #16161a;
    --ink-soft: #3f3f46;
    --muted: #71717a;
    --line: #ececef;
    --line-strong: #e0e0e4;
    --paper-2: #fbfbfc;
    --accent: #5b4be0;
    --accent-dark: #3f31bd;

    min-height: 100vh;
    background: #fff;
    color: var(--ink);
    font-family: 'Instrument Sans', ui-sans-serif, system-ui, sans-serif;
    -webkit-font-smoothing: antialiased;
}
.cv-defs {
    position: absolute;
    width: 0;
    height: 0;
}

/* Top bar */
.bar {
    position: sticky;
    top: 0;
    z-index: 20;
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 68px;
    padding-inline: 24px;
    background: rgba(255, 255, 255, 0.78);
    backdrop-filter: saturate(1.4) blur(14px);
    border-bottom: 1px solid var(--line);
}
.brand {
    display: inline-flex;
    align-items: center;
    gap: 0.55rem;
    text-decoration: none;
    color: var(--ink);
}
.brand-mark {
    width: 28px;
    height: 28px;
    display: grid;
    place-items: center;
}
.brand-mark svg {
    width: 100%;
    height: 100%;
}
.brand-name {
    font-weight: 600;
    font-size: 1.05rem;
    letter-spacing: -0.02em;
}
.back {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--ink-soft);
    text-decoration: none;
}
.back:hover {
    color: var(--ink);
}
.back svg {
    width: 18px;
    height: 18px;
}

.wrap {
    max-width: 1180px;
    margin-inline: auto;
    padding: 2.5rem 24px 5rem;
}
.intro {
    max-width: 44rem;
    margin-bottom: 2.5rem;
}
.kicker {
    display: inline-block;
    font-size: 0.76rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.13em;
    color: var(--muted);
    margin-bottom: 0.8rem;
}
.title {
    font-size: clamp(1.9rem, 4vw, 2.8rem);
    line-height: 1.06;
    letter-spacing: -0.03em;
    font-weight: 600;
}
.lede {
    margin-top: 1rem;
    font-size: 1.08rem;
    line-height: 1.6;
    color: var(--ink-soft);
}

/* Studio layout */
.studio {
    display: grid;
    gap: 1.5rem;
}
@media (min-width: 940px) {
    .studio {
        grid-template-columns: 1fr 360px;
        align-items: start;
    }
}

/* Stage */
.stage {
    position: sticky;
    top: 92px;
}
.scene {
    position: relative;
    aspect-ratio: 4 / 3;
    border-radius: 22px;
    overflow: hidden;
    border: 1px solid var(--line-strong);
    box-shadow: 0 30px 60px -40px rgba(16, 16, 24, 0.4);
}
.scene-sky {
    position: absolute;
    inset: 0;
    background: linear-gradient(180deg, #8ec5ff 0%, #cfe8ff 45%, #ffe9c7 100%);
}
.scene-sun {
    position: absolute;
    top: 16%;
    left: 64%;
    width: 88px;
    height: 88px;
    border-radius: 50%;
    background: radial-gradient(circle, #fff6d6, #ffd24a 70%);
    box-shadow: 0 0 60px 20px rgba(255, 210, 74, 0.45);
}
.scene-hills {
    position: absolute;
    inset: 0;
}
.hill {
    position: absolute;
    bottom: 0;
    border-radius: 50% 50% 0 0;
}
.h1 {
    left: -10%;
    width: 70%;
    height: 42%;
    background: linear-gradient(180deg, #5bbf72, #3a9a57);
}
.h2 {
    right: -12%;
    width: 72%;
    height: 52%;
    background: linear-gradient(180deg, #2f9d8a, #1f7d77);
}
.h3 {
    left: 24%;
    width: 60%;
    height: 32%;
    background: linear-gradient(180deg, #7fce86, #4faf63);
}
.scene-card {
    position: absolute;
    left: 5%;
    right: 5%;
    bottom: 5%;
    padding: 1rem 1.1rem 1.15rem;
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.92);
    backdrop-filter: blur(4px);
    box-shadow: 0 10px 30px -16px rgba(16, 16, 24, 0.5);
}
.sc-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.7rem;
}
.sc-title {
    font-weight: 600;
    font-size: 0.95rem;
    color: #16161a;
}
.sc-pill {
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #5b4be0;
    background: rgba(91, 75, 224, 0.12);
    padding: 0.2rem 0.5rem;
    border-radius: 999px;
}
.swatches {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
}
.swatches span {
    aspect-ratio: 1.6;
    border-radius: 8px;
    background: var(--c);
    display: flex;
    align-items: flex-end;
    padding: 4px 6px;
    font-size: 0.6rem;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.92);
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.35);
}
.sc-text {
    margin-top: 0.7rem;
    font-size: 0.82rem;
    color: #52525b;
}
.stage-foot {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    margin-top: 0.9rem;
    font-size: 0.82rem;
    font-weight: 500;
    color: var(--muted);
}
.dot-key {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    margin-left: 0.6rem;
}
.dot-key.r,
.ch-dot.r {
    background: #e23b3b;
}
.dot-key.g,
.ch-dot.g {
    background: #36a85a;
}
.dot-key.b,
.ch-dot.b {
    background: #2f6fe0;
}

/* Panel */
.panel {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}
.block {
    padding: 1.25rem;
    border-radius: 18px;
    border: 1px solid var(--line);
    background: var(--paper-2);
}
.block-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
}
.block-head h2 {
    font-size: 1rem;
    font-weight: 600;
    letter-spacing: -0.01em;
}
.link-btn {
    border: none;
    background: none;
    cursor: pointer;
    font: inherit;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--accent);
}
.presets {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 1.25rem;
}
.preset {
    border: 1px solid var(--line-strong);
    background: #fff;
    border-radius: 10px;
    padding: 0.4rem 0.7rem;
    font: inherit;
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--ink-soft);
    cursor: pointer;
    transition:
        transform 0.1s ease,
        border-color 0.15s ease,
        color 0.15s ease;
}
.preset:hover {
    color: var(--ink);
    border-color: var(--muted);
}
.preset:active {
    transform: translateY(1px);
}

.ctrl {
    margin-bottom: 0.95rem;
}
.ctrl-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.4rem;
}
.ctrl-head label {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    font-size: 0.86rem;
    font-weight: 600;
}
.ctrl-val {
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
}
.ch-dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
}
.channels-head {
    margin: 1.35rem 0 0.85rem;
    font-size: 0.74rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--muted);
    padding-top: 1rem;
    border-top: 1px solid var(--line);
}

input[type='range'] {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    border-radius: 999px;
    background: var(--line-strong);
    outline: none;
    accent-color: var(--accent);
    cursor: pointer;
}
input[type='range']::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #fff;
    border: 1px solid var(--line-strong);
    box-shadow: 0 2px 5px rgba(16, 16, 24, 0.22);
    cursor: pointer;
}
input[type='range']::-moz-range-thumb {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #fff;
    border: 1px solid var(--line-strong);
    box-shadow: 0 2px 5px rgba(16, 16, 24, 0.22);
    cursor: pointer;
}
.range.r {
    accent-color: #e23b3b;
}
.range.g {
    accent-color: #36a85a;
}
.range.b {
    accent-color: #2f6fe0;
}

/* Segmented CB control */
.seg {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.4rem;
    margin-bottom: 1.1rem;
}
.seg-btn {
    border: 1px solid var(--line-strong);
    background: #fff;
    border-radius: 10px;
    padding: 0.5rem 0.6rem;
    font: inherit;
    font-size: 0.82rem;
    font-weight: 600;
    color: var(--ink-soft);
    cursor: pointer;
    transition:
        border-color 0.15s ease,
        background 0.15s ease,
        color 0.15s ease;
}
.seg-btn:hover {
    border-color: var(--muted);
}
.seg-btn.active {
    background: var(--ink);
    border-color: var(--ink);
    color: #fff;
}

.toggle {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    cursor: pointer;
    user-select: none;
}
.toggle.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}
.toggle input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}
.switch {
    position: relative;
    width: 40px;
    height: 23px;
    border-radius: 999px;
    background: var(--line-strong);
    transition: background 0.18s ease;
    flex: none;
}
.switch::after {
    content: '';
    position: absolute;
    top: 2.5px;
    left: 2.5px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 1px 3px rgba(16, 16, 24, 0.3);
    transition: transform 0.18s cubic-bezier(0.3, 0.8, 0.3, 1);
}
.toggle input:checked + .switch {
    background: var(--accent);
}
.toggle input:checked + .switch::after {
    transform: translateX(17px);
}
.toggle-text {
    font-size: 0.9rem;
    font-weight: 600;
}
.hint {
    margin-top: 0.85rem;
    font-size: 0.84rem;
    line-height: 1.5;
    color: var(--muted);
}

.cta {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.9rem 1.3rem;
    border-radius: 14px;
    background: var(--ink);
    color: #fff;
    font-weight: 600;
    font-size: 0.98rem;
    text-decoration: none;
    box-shadow: 0 5px 0 0 #000;
    transition:
        transform 0.11s cubic-bezier(0.2, 0.8, 0.2, 1),
        box-shadow 0.11s,
        background 0.15s;
}
.cta:hover {
    background: #26262e;
}
.cta:active {
    transform: translateY(5px);
    box-shadow: 0 0 0 0 #000;
}
.disclaimer {
    font-size: 0.78rem;
    line-height: 1.5;
    color: #a1a1aa;
    text-align: center;
}
</style>
