<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';

type FilterKind = 'protan' | 'deutan' | 'tritan';
type Axis = 'rg' | 'by';

const emit = defineEmits<{
    (e: 'complete', result: { kind: FilterKind; intensity: number }): void;
    (e: 'cancel'): void;
}>();

type RGB = [number, number, number];

// Iso-luminant directions (Rec. 601): moving along these changes hue but not
// brightness, so brightness can't be used as a shortcut to spot the target.
const BASE: RGB = [128, 120, 120];
const DIR: Record<Axis, RGB> = {
    rg: [0.587, -0.299, 0], // red vs green
    by: [-0.1287, -0.1287, 1.0], // blue vs yellow
};
// Magnitudes from clearly visible to subtle (for normal color vision).
const MAGS: Record<Axis, number[]> = {
    rg: [70, 48, 32, 20, 12],
    by: [90, 62, 42, 28, 17],
};

const GRID = 16; // 4x4

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
            const target: RGB = [
                BASE[0] + sign * m * dir[0],
                BASE[1] + sign * m * dir[1],
                BASE[2] + sign * m * dir[2],
            ];
            plates.push({
                axis,
                base: BASE,
                target,
                targetIndex: Math.floor(Math.random() * GRID),
            });
        });
    });
    // Shuffle so axes alternate unpredictably.
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
    if (current.value < plates.value.length - 1) {
        current.value++;
    } else {
        finishTest();
    }
}

function pickDot(index: number) {
    answer(index === plate.value.targetIndex);
}
function cantTell() {
    answer(false);
}

const resultKind = ref<FilterKind>('deutan');
const resultIntensity = ref(0.7);
const resultLabel = ref('');
const isNormal = ref(false);

function severityFromErrors(errors: number): number {
    return Math.min(1, 0.25 + errors * 0.15);
}

function finishTest() {
    const normal = errorsRG <= 1 && errorsBY <= 1;
    if (normal) {
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
        // Red-green: confirm protan vs deutan with a quick preview.
        resultIntensity.value = severityFromErrors(errorsRG);
        phase.value = 'refine';
    }
}

// Daltonization previews for the protan/deutan refinement (SVG feColorMatrix).
const protanMatrix =
    '1 0 0 0 0  -0.2549 1.2549 0 0 0  0.3031 -0.5451 1.242 0 0  0 0 0 1 0';
const deutanMatrix =
    '1 0 0 0 0  -0.4375 1.4375 0 0 0  0.2625 -0.5625 1.3 0 0  0 0 0 1 0';

// Colors that are easy to confuse with red-green deficiency.
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

const severityWord = computed(() => {
    const v = resultIntensity.value;
    if (v < 0.55) return 'mild';
    if (v < 0.8) return 'moderate';
    return 'strong';
});

function applyResult() {
    if (isNormal.value) {
        emit('cancel');
        return;
    }
    emit('complete', {
        kind: resultKind.value,
        intensity: resultIntensity.value,
    });
}

onMounted(() => {
    // Nothing to preload; the intro shows first.
});
</script>

<template>
    <div class="cal">
        <button class="close" @click="emit('cancel')" aria-label="Close">
            &times;
        </button>

        <!-- Intro -->
        <div v-if="phase === 'intro'" class="screen">
            <h1 class="title">Find the filter that fits your eyes</h1>
            <p class="lead">
                A short test. On each card, one dot has a slightly different
                color. Click it. If none stand out, choose
                <strong>I can't tell</strong>. There are no wrong answers, it
                just tunes ChromaVale to you.
            </p>
            <div class="actions">
                <button class="btn primary" @click="start">Start test</button>
                <button class="btn ghost" @click="emit('cancel')">
                    Maybe later
                </button>
            </div>
        </div>

        <!-- Test -->
        <div v-else-if="phase === 'test'" class="screen">
            <div class="bar">
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
            <button class="btn ghost wide" @click="cantTell">
                I can't tell
            </button>
        </div>

        <!-- Protan vs deutan refinement -->
        <div v-else-if="phase === 'refine'" class="screen">
            <h2 class="subtitle">One more thing</h2>
            <p class="lead">
                Which set of colors looks easier to tell apart?
            </p>
            <div class="ab">
                <button class="ab-card" @click="chooseRedGreen('protan')">
                    <div class="ab-swatches" style="filter: url(#cal-protan)">
                        <span
                            v-for="(c, idx) in refineSwatches"
                            :key="'p' + idx"
                            :style="{ background: c }"
                        ></span>
                    </div>
                    <span class="ab-label">A</span>
                </button>
                <button class="ab-card" @click="chooseRedGreen('deutan')">
                    <div class="ab-swatches" style="filter: url(#cal-deutan)">
                        <span
                            v-for="(c, idx) in refineSwatches"
                            :key="'d' + idx"
                            :style="{ background: c }"
                        ></span>
                    </div>
                    <span class="ab-label">B</span>
                </button>
            </div>
            <button class="btn ghost wide" @click="chooseRedGreen('deutan')">
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
        </div>

        <!-- Result -->
        <div v-else class="screen">
            <span class="result-mark">
                <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                    <path
                        d="M5 12.5l4.5 4.5L19 7"
                        stroke="currentColor"
                        stroke-width="2.2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    />
                </svg>
            </span>
            <h2 class="subtitle">{{ resultLabel }}</h2>
            <p v-if="!isNormal" class="lead">
                We estimate a <strong>{{ severityWord }}</strong> level and have
                prepared a correction tuned to you. You can fine-tune it any
                time in the panel.
            </p>
            <p v-else class="lead">
                No correction needed. You can still use the color engine and
                comfort filters.
            </p>
            <div class="actions">
                <button class="btn primary" @click="applyResult">
                    {{ isNormal ? 'Done' : 'Apply correction' }}
                </button>
                <button v-if="!isNormal" class="btn ghost" @click="start">
                    Retake
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
.cal {
    position: fixed;
    inset: 0;
    z-index: 50;
    background: #0f0f12;
    color: #f4f4f6;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    font-family: 'Segoe UI', ui-sans-serif, system-ui, sans-serif;
}
.close {
    position: absolute;
    top: 14px;
    right: 16px;
    border: none;
    background: transparent;
    color: #9a9aa3;
    font-size: 1.4rem;
    line-height: 1;
    cursor: pointer;
}
.close:hover {
    color: #fff;
}
.screen {
    width: 100%;
    max-width: 340px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
}
.title {
    font-size: 1.4rem;
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.15;
}
.subtitle {
    font-size: 1.2rem;
    font-weight: 600;
    letter-spacing: -0.01em;
}
.lead {
    margin-top: 0.9rem;
    font-size: 0.92rem;
    line-height: 1.55;
    color: #b6b6bf;
}
.lead strong {
    color: #fff;
}
.actions {
    margin-top: 1.6rem;
    display: flex;
    gap: 0.6rem;
}
.btn {
    font: inherit;
    font-weight: 600;
    font-size: 0.92rem;
    border-radius: 12px;
    padding: 0.7rem 1.2rem;
    border: 1px solid transparent;
    cursor: pointer;
}
.btn.primary {
    background: #6b5bf0;
    color: #fff;
    box-shadow: 0 4px 0 0 #3f31bd;
    transition: transform 0.1s, box-shadow 0.1s, filter 0.15s;
}
.btn.primary:hover {
    filter: brightness(1.07);
}
.btn.primary:active {
    transform: translateY(4px);
    box-shadow: 0 0 0 0 #3f31bd;
}
.btn.ghost {
    background: #17171b;
    color: #f4f4f6;
    border-color: #26262c;
}
.btn.ghost:hover {
    background: #20202a;
}
.btn.wide {
    margin-top: 1.2rem;
    width: 100%;
}
.bar {
    width: 100%;
    height: 6px;
    border-radius: 999px;
    background: #2a2a31;
    overflow: hidden;
    margin-bottom: 1.1rem;
}
.bar-fill {
    height: 100%;
    background: #6b5bf0;
    transition: width 0.2s;
}
.hint {
    font-size: 0.9rem;
    font-weight: 600;
    margin-bottom: 1rem;
}
.grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
    width: 100%;
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
    gap: 0.8rem;
    width: 100%;
    margin-top: 1.2rem;
}
.ab-card {
    border: 1px solid #26262c;
    background: #17171b;
    border-radius: 14px;
    padding: 0.9rem;
    cursor: pointer;
    transition: border-color 0.15s, transform 0.08s;
}
.ab-card:hover {
    border-color: #6b5bf0;
}
.ab-card:active {
    transform: translateY(2px);
}
.ab-swatches {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 5px;
}
.ab-swatches span {
    aspect-ratio: 1;
    border-radius: 6px;
}
.ab-label {
    display: block;
    margin-top: 0.6rem;
    font-weight: 700;
    color: #b6b6bf;
}
.result-mark {
    display: grid;
    place-items: center;
    width: 52px;
    height: 52px;
    border-radius: 50%;
    background: rgba(107, 91, 240, 0.16);
    color: #b9aaff;
    margin-bottom: 0.6rem;
}
.result-mark svg {
    width: 26px;
    height: 26px;
}
</style>
