<script setup lang="ts">
import { Head, Link } from '@inertiajs/vue3';
import { ref, computed } from 'vue';

type FilterKind = 'protan' | 'deutan' | 'tritan';
type Axis = 'rg' | 'by';
type RGB = [number, number, number];

const BASE: RGB = [150, 142, 142];
const DIR: Record<Axis, RGB> = {
    rg: [0.587, -0.299, 0],
    by: [-0.1287, -0.1287, 1.0],
};
const MAGS: Record<Axis, number[]> = {
    rg: [70, 48, 32, 20, 12],
    by: [90, 62, 42, 28, 17],
};
const GRID = 16;

interface Plate {
    axis: Axis;
    base: RGB;
    target: RGB;
    targetIndex: number;
}

function clampByte(v: number): number {
    return Math.max(0, Math.min(255, Math.round(v)));
}
function cssColor(c: RGB): string {
    return `rgb(${clampByte(c[0])}, ${clampByte(c[1])}, ${clampByte(c[2])})`;
}

function buildPlates(): Plate[] {
    const plates: Plate[] = [];
    (['rg', 'by'] as Axis[]).forEach((axis) => {
        MAGS[axis].forEach((m) => {
            const sign = Math.random() < 0.5 ? 1 : -1;
            const dir = DIR[axis];
            plates.push({
                axis,
                base: BASE,
                target: [
                    BASE[0] + sign * m * dir[0],
                    BASE[1] + sign * m * dir[1],
                    BASE[2] + sign * m * dir[2],
                ],
                targetIndex: Math.floor(Math.random() * GRID),
            });
        });
    });
    for (let i = plates.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [plates[i], plates[j]] = [plates[j], plates[i]];
    }
    return plates;
}

type Phase = 'intro' | 'test' | 'refine' | 'result';
const phase = ref<Phase>('intro');
const plates = ref<Plate[]>([]);
const current = ref(0);
let errorsRG = 0;
let errorsBY = 0;

const plate = computed(() => plates.value[current.value]);
const progress = computed(() =>
    plates.value.length
        ? Math.round((current.value / plates.value.length) * 100)
        : 0,
);

function start() {
    plates.value = buildPlates();
    current.value = 0;
    errorsRG = 0;
    errorsBY = 0;
    phase.value = 'test';
}

function answer(correct: boolean) {
    if (!correct) {
        if (plate.value.axis === 'rg') errorsRG++;
        else errorsBY++;
    }
    if (current.value < plates.value.length - 1) current.value++;
    else finishTest();
}
function pickDot(index: number) {
    answer(index === plate.value.targetIndex);
}

const resultKind = ref<FilterKind>('deutan');
const resultIntensity = ref(0.7);
const resultLabel = ref('');
const isNormal = ref(false);

function severityFromErrors(errors: number): number {
    return Math.min(1, 0.25 + errors * 0.15);
}
const severityWord = computed(() => {
    const v = resultIntensity.value;
    if (v < 0.55) return 'mild';
    if (v < 0.8) return 'moderate';
    return 'strong';
});

function finishTest() {
    if (errorsRG <= 1 && errorsBY <= 1) {
        isNormal.value = true;
        resultLabel.value = 'Your color vision looks typical';
        phase.value = 'result';
        return;
    }
    isNormal.value = false;
    if (errorsBY > errorsRG) {
        resultKind.value = 'tritan';
        resultIntensity.value = severityFromErrors(errorsBY);
        resultLabel.value = 'Blue-yellow (tritan) tendency';
        phase.value = 'result';
    } else {
        resultIntensity.value = severityFromErrors(errorsRG);
        phase.value = 'refine';
    }
}

const protanMatrix =
    '1 0 0 0 0  -0.2549 1.2549 0 0 0  0.3031 -0.5451 1.242 0 0  0 0 0 1 0';
const deutanMatrix =
    '1 0 0 0 0  -0.4375 1.4375 0 0 0  0.2625 -0.5625 1.3 0 0  0 0 0 1 0';
const refineSwatches = [
    '#c0392b',
    '#9b8f1e',
    '#7a8b2f',
    '#b5651d',
    '#6b8e23',
    '#a0522d',
];
function chooseRedGreen(kind: FilterKind) {
    resultKind.value = kind;
    resultLabel.value =
        kind === 'protan'
            ? 'Red-green (protan) tendency'
            : 'Red-green (deuteran) tendency';
    phase.value = 'result';
}
</script>

<template>
    <Head title="Free color blindness test - ChromaVale">
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link
            href="https://fonts.googleapis.com/css2?family=Instrument+Sans:wght@400..700&display=swap"
            rel="stylesheet"
        />
    </Head>

    <div class="page">
        <header class="bar">
            <Link href="/" class="brand">
                <span class="brand-mark">
                    <svg viewBox="0 0 32 32" fill="none" aria-hidden="true">
                        <circle cx="16" cy="16" r="13.5" stroke="currentColor" stroke-width="2.5" />
                        <circle cx="16" cy="16" r="6.5" fill="url(#prism)" />
                        <defs>
                            <linearGradient id="prism" x1="9" y1="9" x2="23" y2="23" gradientUnits="userSpaceOnUse">
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
            <Link href="/" class="back">Back to site</Link>
        </header>

        <main class="wrap">
            <!-- Intro -->
            <section v-if="phase === 'intro'" class="card center">
                <span class="kicker">Free test</span>
                <h1 class="title">Find the filter that fits your eyes</h1>
                <p class="lead">
                    A short color vision check. On each card, one dot has a
                    slightly different color. Click it. If none stand out,
                    choose <strong>I can't tell</strong>. At the end we
                    recommend the correction that fits you.
                </p>
                <button class="btn btn-ink btn-lg" @click="start">
                    Start the test
                </button>
                <p class="fine">
                    Takes under a minute. An illustration, not a medical
                    diagnosis.
                </p>
            </section>

            <!-- Test -->
            <section v-else-if="phase === 'test'" class="card center">
                <div class="bar-track">
                    <div class="bar-fill" :style="{ width: progress + '%' }"></div>
                </div>
                <p class="hint">Click the dot that looks different</p>
                <div class="grid">
                    <button
                        v-for="i in GRID"
                        :key="i"
                        class="dot"
                        :style="{
                            background:
                                i - 1 === plate.targetIndex
                                    ? cssColor(plate.target)
                                    : cssColor(plate.base),
                        }"
                        @click="pickDot(i - 1)"
                    ></button>
                </div>
                <button class="btn btn-soft btn-block" @click="answer(false)">
                    I can't tell
                </button>
            </section>

            <!-- Refine -->
            <section v-else-if="phase === 'refine'" class="card center">
                <h2 class="subtitle">One more thing</h2>
                <p class="lead">Which set of colors is easier to tell apart?</p>
                <div class="ab">
                    <button class="ab-card" @click="chooseRedGreen('protan')">
                        <div class="ab-swatches" style="filter: url(#cal-protan)">
                            <span v-for="(c, i) in refineSwatches" :key="'p' + i" :style="{ background: c }"></span>
                        </div>
                        <span class="ab-label">A</span>
                    </button>
                    <button class="ab-card" @click="chooseRedGreen('deutan')">
                        <div class="ab-swatches" style="filter: url(#cal-deutan)">
                            <span v-for="(c, i) in refineSwatches" :key="'d' + i" :style="{ background: c }"></span>
                        </div>
                        <span class="ab-label">B</span>
                    </button>
                </div>
                <button class="btn btn-soft btn-block" @click="chooseRedGreen('deutan')">
                    They look the same
                </button>
                <svg width="0" height="0" aria-hidden="true">
                    <filter id="cal-protan" color-interpolation-filters="sRGB">
                        <feColorMatrix type="matrix" :values="protanMatrix" />
                    </filter>
                    <filter id="cal-deutan" color-interpolation-filters="sRGB">
                        <feColorMatrix type="matrix" :values="deutanMatrix" />
                    </filter>
                </svg>
            </section>

            <!-- Result -->
            <section v-else class="card center">
                <span class="result-mark">
                    <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                        <path d="M5 12.5l4.5 4.5L19 7" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" />
                    </svg>
                </span>
                <h2 class="subtitle">{{ resultLabel }}</h2>
                <p v-if="!isNormal" class="lead">
                    We estimate a <strong>{{ severityWord }}</strong> level.
                    ChromaVale can apply a correction tuned to this across your
                    whole screen, on every app and game.
                </p>
                <p v-else class="lead">
                    No correction needed. ChromaVale can still tune comfort,
                    warmth and vivid gaming looks for you.
                </p>
                <div class="result-cta">
                    <Link href="/download" class="btn btn-ink btn-lg">
                        Download ChromaVale, free
                    </Link>
                    <button class="btn btn-soft btn-lg" @click="start">
                        Retake test
                    </button>
                </div>
            </section>
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
.bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 68px;
    padding-inline: 24px;
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
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--ink-soft);
    text-decoration: none;
}
.back:hover {
    color: var(--ink);
}
.wrap {
    max-width: 520px;
    margin-inline: auto;
    padding: 3rem 24px 5rem;
}
.card {
    border: 1px solid var(--line);
    border-radius: 24px;
    background:
        radial-gradient(120% 120% at 100% 0%, rgba(91, 75, 224, 0.05), transparent 55%),
        var(--paper-2);
    padding: 2.5rem 2rem;
}
.center {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
}
.kicker {
    font-size: 0.76rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.13em;
    color: var(--muted);
    margin-bottom: 0.8rem;
}
.title {
    font-size: clamp(1.6rem, 5vw, 2.1rem);
    line-height: 1.1;
    letter-spacing: -0.03em;
    font-weight: 600;
}
.subtitle {
    font-size: 1.4rem;
    font-weight: 600;
    letter-spacing: -0.02em;
}
.lead {
    margin-top: 1rem;
    font-size: 1rem;
    line-height: 1.6;
    color: var(--ink-soft);
    max-width: 30rem;
}
.lead strong {
    color: var(--ink);
}
.fine {
    margin-top: 1rem;
    font-size: 0.82rem;
    color: var(--muted);
}
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font: inherit;
    font-weight: 600;
    border: none;
    border-radius: 13px;
    padding: 0.75rem 1.2rem;
    font-size: 0.95rem;
    cursor: pointer;
    text-decoration: none;
    transition: transform 0.11s, box-shadow 0.11s, background 0.15s;
    --press: 4px;
}
.btn:active {
    transform: translateY(var(--press));
}
.btn-lg {
    padding: 0.95rem 1.5rem;
    font-size: 1rem;
    --press: 5px;
}
.btn-block {
    width: 100%;
    margin-top: 1.2rem;
}
.btn-ink {
    background: var(--ink);
    color: #fff;
    box-shadow: 0 var(--press) 0 0 #000;
    margin-top: 1.8rem;
}
.btn-ink:hover {
    background: #26262e;
}
.btn-ink:active {
    box-shadow: 0 0 0 0 #000;
}
.btn-soft {
    background: #fff;
    color: var(--ink);
    box-shadow: 0 var(--press) 0 0 var(--line-strong), inset 0 0 0 1px var(--line-strong);
}
.btn-soft:active {
    box-shadow: inset 0 0 0 1px var(--line-strong);
}
.bar-track {
    width: 100%;
    height: 6px;
    border-radius: 999px;
    background: var(--line);
    overflow: hidden;
    margin-bottom: 1.4rem;
}
.bar-fill {
    height: 100%;
    background: var(--accent);
    transition: width 0.2s;
}
.hint {
    font-size: 0.95rem;
    font-weight: 600;
    margin-bottom: 1.2rem;
}
.grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 14px;
    width: 100%;
    max-width: 320px;
}
.dot {
    aspect-ratio: 1;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    transition: transform 0.08s;
}
.dot:active {
    transform: scale(0.92);
}
.ab {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.9rem;
    width: 100%;
    margin-top: 1.4rem;
}
.ab-card {
    border: 1px solid var(--line-strong);
    background: #fff;
    border-radius: 16px;
    padding: 1rem;
    cursor: pointer;
    transition: border-color 0.15s, transform 0.08s;
}
.ab-card:hover {
    border-color: var(--accent);
}
.ab-card:active {
    transform: translateY(2px);
}
.ab-swatches {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 6px;
}
.ab-swatches span {
    aspect-ratio: 1;
    border-radius: 7px;
}
.ab-label {
    display: block;
    margin-top: 0.7rem;
    font-weight: 700;
    color: var(--muted);
}
.result-mark {
    display: grid;
    place-items: center;
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: rgba(91, 75, 224, 0.12);
    color: var(--accent);
    margin-bottom: 0.6rem;
}
.result-mark svg {
    width: 28px;
    height: 28px;
}
.result-cta {
    margin-top: 1.8rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    justify-content: center;
}
.result-cta .btn-ink {
    margin-top: 0;
}
</style>
