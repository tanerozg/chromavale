<script setup lang="ts">
import { reactive, ref, watch, onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { load } from '@tauri-apps/plugin-store';
import {
    enable as enableAutostart,
    disable as disableAutostart,
    isEnabled as isAutostartEnabled,
} from '@tauri-apps/plugin-autostart';
import Calibration from './Calibration.vue';

type Settings = {
    temperature: number; // Kelvin
    brightness: number; // 0.5 .. 1.5
    red: number; // 0 .. 2 (1 = neutral)
    green: number;
    blue: number;
    gamma: number; // 0.5 .. 2 (1 = neutral)
};

const NEUTRAL: Settings = {
    temperature: 6500,
    brightness: 1,
    red: 1,
    green: 1,
    blue: 1,
    gamma: 1,
};

const settings = reactive<Settings>({ ...NEUTRAL });
const enabled = ref(true);
const activePreset = ref('Neutral');
const status = ref('');
const error = ref('');

type FilterKind =
    | 'none'
    | 'protan'
    | 'deutan'
    | 'tritan'
    | 'grayscale'
    | 'grayscale-inverted'
    | 'inverted';
const filterKind = ref<FilterKind>('none');
const intensity = ref(1);
const colorBoost = ref(1);
const filterOptions: { key: FilterKind; label: string }[] = [
    { key: 'none', label: 'Off' },
    { key: 'deutan', label: 'Red-green (deuteranopia)' },
    { key: 'protan', label: 'Red-green (protanopia)' },
    { key: 'tritan', label: 'Blue-yellow (tritanopia)' },
    { key: 'grayscale', label: 'Grayscale' },
    { key: 'grayscale-inverted', label: 'Grayscale inverted' },
    { key: 'inverted', label: 'Inverted' },
];

const presets: { name: string; values: Partial<Settings> }[] = [
    { name: 'Neutral', values: { ...NEUTRAL } },
    {
        name: 'Comfort',
        values: { temperature: 4200, brightness: 1, red: 1, green: 1, blue: 1, gamma: 1 },
    },
    {
        name: 'Reading',
        values: { temperature: 4600, brightness: 0.97, red: 1, green: 0.99, blue: 0.94, gamma: 1.02 },
    },
    {
        name: 'Night',
        values: { temperature: 3200, brightness: 0.85, red: 1, green: 0.92, blue: 0.72, gamma: 1 },
    },
    {
        name: 'Vivid',
        values: { temperature: 6500, brightness: 1.05, red: 1, green: 1, blue: 1, gamma: 0.92 },
    },
];

const sliders: {
    key: keyof Settings;
    label: string;
    min: number;
    max: number;
    step: number;
    format: (v: number) => string;
}[] = [
    {
        key: 'temperature',
        label: 'Temperature',
        min: 2500,
        max: 9300,
        step: 50,
        format: (v) => `${Math.round(v)} K`,
    },
    {
        key: 'brightness',
        label: 'Brightness',
        min: 0.5,
        max: 1.2,
        step: 0.01,
        format: (v) => `${Math.round(v * 100)}%`,
    },
    {
        key: 'gamma',
        label: 'Gamma',
        min: 0.6,
        max: 1.6,
        step: 0.01,
        format: (v) => v.toFixed(2),
    },
];

const channels: { key: keyof Settings; label: string; cls: string }[] = [
    { key: 'red', label: 'Red', cls: 'r' },
    { key: 'green', label: 'Green', cls: 'g' },
    { key: 'blue', label: 'Blue', cls: 'b' },
];

let applyTimer: ReturnType<typeof setTimeout> | null = null;

async function apply() {
    if (!enabled.value) return;
    try {
        await invoke('apply_color', { settings: { ...settings } });
        error.value = '';
        status.value = 'Applied to your screen';
    } catch (e) {
        error.value = String(e);
        status.value = '';
    }
}

function scheduleApply() {
    if (applyTimer) clearTimeout(applyTimer);
    applyTimer = setTimeout(apply, 40);
}

async function reset() {
    try {
        await invoke('reset_color');
        error.value = '';
        status.value = 'Screen restored';
    } catch (e) {
        error.value = String(e);
    }
}

let filterTimer: ReturnType<typeof setTimeout> | null = null;

async function applyFilter() {
    if (!enabled.value) return;
    try {
        if (filterKind.value === 'none' && colorBoost.value === 1) {
            await invoke('clear_filter');
        } else {
            await invoke('apply_filter', {
                kind: filterKind.value,
                strength: intensity.value,
                colorBoost: colorBoost.value,
            });
        }
        error.value = '';
    } catch (e) {
        error.value = String(e);
    }
}

function scheduleFilter() {
    if (filterTimer) clearTimeout(filterTimer);
    filterTimer = setTimeout(applyFilter, 40);
}

watch([filterKind, intensity, colorBoost], scheduleFilter);

// --- Persistence: remember everything across launches ---
type SavedState = {
    settings: Settings;
    enabled: boolean;
    filterKind: FilterKind;
    intensity: number;
    colorBoost: number;
};

let store: Awaited<ReturnType<typeof load>> | null = null;
let restored = false;
let persistTimer: ReturnType<typeof setTimeout> | null = null;

async function persist() {
    if (!store || !restored) return;
    const state: SavedState = {
        settings: { ...settings },
        enabled: enabled.value,
        filterKind: filterKind.value,
        intensity: intensity.value,
        colorBoost: colorBoost.value,
    };
    try {
        await store.set('state', state);
        await store.save();
    } catch {
        // Ignore persistence errors; they should never break the UI.
    }
}

function schedulePersist() {
    if (persistTimer) clearTimeout(persistTimer);
    persistTimer = setTimeout(persist, 300);
}

watch([settings, enabled, filterKind, intensity, colorBoost], schedulePersist, {
    deep: true,
});

// --- Autostart ---
const autostart = ref(false);

async function toggleAutostart() {
    try {
        if (autostart.value) {
            await disableAutostart();
            autostart.value = false;
        } else {
            await enableAutostart();
            autostart.value = true;
        }
    } catch (e) {
        error.value = String(e);
    }
}

// --- Calibration ---
const calibrating = ref(false);

async function onCalibrated(result: { kind: FilterKind; intensity: number }) {
    filterKind.value = result.kind;
    intensity.value = result.intensity;
    enabled.value = true;
    calibrating.value = false;
    await apply();
    await applyFilter();
    status.value = 'Calibration applied';
}

function choosePreset(name: string) {
    const preset = presets.find((p) => p.name === name);
    if (!preset) return;
    activePreset.value = name;
    Object.assign(settings, NEUTRAL, preset.values);
}

async function toggleEnabled() {
    enabled.value = !enabled.value;
    if (enabled.value) {
        await apply();
        await applyFilter();
    } else {
        await reset();
        try {
            await invoke('clear_filter');
        } catch (e) {
            error.value = String(e);
        }
    }
}

watch(
    settings,
    () => {
        activePreset.value =
            presets.find((p) =>
                (Object.keys(p.values) as (keyof Settings)[]).every(
                    (k) => p.values[k] === settings[k],
                ),
            )?.name ?? 'Custom';
        scheduleApply();
    },
    { deep: true },
);

let unlistenToggle: UnlistenFn | null = null;

onMounted(async () => {
    // Restore saved state from the last session.
    try {
        store = await load('settings.json');
        const saved = await store.get<SavedState>('state');
        if (saved) {
            Object.assign(settings, saved.settings);
            enabled.value = saved.enabled;
            filterKind.value = saved.filterKind;
            intensity.value = saved.intensity;
            colorBoost.value = saved.colorBoost;
        }
    } catch {
        // No saved state yet, or store unavailable: start from defaults.
    }
    restored = true;

    await apply();
    await applyFilter();

    try {
        autostart.value = await isAutostartEnabled();
    } catch {
        // Autostart not available on this platform.
    }

    unlistenToggle = await listen('toggle-power', () => {
        toggleEnabled();
    });
});
onBeforeUnmount(() => {
    if (applyTimer) clearTimeout(applyTimer);
    if (filterTimer) clearTimeout(filterTimer);
    if (persistTimer) clearTimeout(persistTimer);
    if (unlistenToggle) unlistenToggle();
});
</script>

<template>
    <div class="app">
        <header class="top">
            <div class="brand">
                <span class="mark">
                    <svg viewBox="0 0 32 32" fill="none" aria-hidden="true">
                        <circle
                            cx="16"
                            cy="16"
                            r="13.5"
                            stroke="currentColor"
                            stroke-width="2.5"
                        />
                        <circle cx="16" cy="16" r="6.5" fill="url(#g)" />
                        <defs>
                            <linearGradient
                                id="g"
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
                <span class="name">ChromaVale</span>
            </div>
            <button
                class="power"
                :class="{ on: enabled }"
                @click="toggleEnabled"
                title="Toggle ChromaVale (Ctrl+Alt+C)"
            >
                <span class="dot"></span>
                {{ enabled ? 'On' : 'Off' }}
            </button>
        </header>

        <p class="hotkey-hint">
            Toggle anywhere with
            <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>C</kbd>
        </p>

        <section class="presets">
            <button
                v-for="p in presets"
                :key="p.name"
                class="preset"
                :class="{ active: activePreset === p.name }"
                @click="choosePreset(p.name)"
            >
                {{ p.name }}
            </button>
        </section>

        <section class="panel" :class="{ disabled: !enabled }">
            <div v-for="s in sliders" :key="s.key" class="ctrl">
                <div class="ctrl-head">
                    <label>{{ s.label }}</label>
                    <span class="val">{{ s.format(settings[s.key]) }}</span>
                </div>
                <input
                    v-model.number="settings[s.key]"
                    type="range"
                    :min="s.min"
                    :max="s.max"
                    :step="s.step"
                    :disabled="!enabled"
                />
            </div>

            <div class="channels-label">Per channel</div>
            <div v-for="c in channels" :key="c.key" class="ctrl">
                <div class="ctrl-head">
                    <label>
                        <span class="cdot" :class="c.cls"></span>{{ c.label }}
                    </label>
                    <span class="val">
                        {{ Math.round(settings[c.key] * 100) }}%
                    </span>
                </div>
                <input
                    v-model.number="settings[c.key]"
                    type="range"
                    min="0.5"
                    max="1.2"
                    step="0.01"
                    :class="['range', c.cls]"
                    :disabled="!enabled"
                />
            </div>
        </section>

        <section class="panel cb">
            <div class="cb-head">
                <h2>Screen filter</h2>
                <span class="cb-tag">Daltonization</span>
            </div>
            <button class="cal-btn" @click="calibrating = true">
                <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                    <circle cx="12" cy="12" r="8" stroke="currentColor" stroke-width="1.7" />
                    <circle cx="12" cy="12" r="3" fill="currentColor" />
                </svg>
                Find my filter (test)
            </button>
            <div class="filter-list">
                <button
                    v-for="o in filterOptions"
                    :key="o.key"
                    class="filter-opt"
                    :class="{ active: filterKind === o.key }"
                    @click="filterKind = o.key"
                >
                    <span class="radio"></span>
                    {{ o.label }}
                </button>
            </div>
            <div class="ctrl" :class="{ disabled: filterKind === 'none' }">
                <div class="ctrl-head">
                    <label>Intensity</label>
                    <span class="val">{{ Math.round(intensity * 100) }}%</span>
                </div>
                <input
                    v-model.number="intensity"
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    :disabled="filterKind === 'none'"
                />
            </div>
            <div class="ctrl">
                <div class="ctrl-head">
                    <label>Color boost</label>
                    <span class="val">{{ Math.round(colorBoost * 100) }}%</span>
                </div>
                <input
                    v-model.number="colorBoost"
                    type="range"
                    min="1"
                    max="2"
                    step="0.01"
                />
            </div>
            <p class="cb-note">
                Color-blind modes remap colors you confuse into tones you can
                tell apart. Color boost makes every color more vivid.
            </p>
        </section>

        <div class="option-row">
            <span>Start with Windows</span>
            <button
                class="mini-toggle"
                :class="{ on: autostart }"
                role="switch"
                :aria-checked="autostart"
                @click="toggleAutostart"
            >
                <span class="knob2"></span>
            </button>
        </div>

        <footer class="foot">
            <button class="btn ghost" @click="reset">Reset screen</button>
            <span v-if="error" class="msg err">{{ error }}</span>
            <span v-else class="msg">{{ status }}</span>
        </footer>

        <Calibration
            v-if="calibrating"
            @complete="onCalibrated"
            @cancel="calibrating = false"
        />
    </div>
</template>

<style scoped>
.app {
    --ink: #f4f4f6;
    --muted: #9a9aa3;
    --line: #26262c;
    --panel: #17171b;
    --accent: #6b5bf0;
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    padding: 18px;
    gap: 16px;
    background: #0f0f12;
    color: var(--ink);
    font-family: 'Segoe UI', ui-sans-serif, system-ui, sans-serif;
    -webkit-font-smoothing: antialiased;
    user-select: none;
}
.top {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.brand {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
}
.mark {
    width: 26px;
    height: 26px;
    color: #fff;
    display: grid;
    place-items: center;
}
.mark svg {
    width: 100%;
    height: 100%;
}
.name {
    font-weight: 600;
    font-size: 1.05rem;
    letter-spacing: -0.02em;
}
.power {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    border: 1px solid var(--line);
    background: var(--panel);
    color: var(--muted);
    border-radius: 999px;
    padding: 0.35rem 0.75rem;
    font: inherit;
    font-size: 0.82rem;
    font-weight: 600;
    cursor: pointer;
}
.power.on {
    color: #fff;
    border-color: rgba(107, 91, 240, 0.5);
}
.power .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #54545c;
}
.power.on .dot {
    background: #5bd6a0;
}

.hotkey-hint {
    margin-top: -6px;
    font-size: 0.74rem;
    color: var(--muted);
}
.hotkey-hint kbd {
    font-family: inherit;
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--ink);
    background: #1c1c22;
    border: 1px solid var(--line);
    border-radius: 5px;
    padding: 0.05rem 0.3rem;
}

.presets {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
}
.preset {
    border: 1px solid var(--line);
    background: var(--panel);
    color: var(--muted);
    border-radius: 10px;
    padding: 0.4rem 0.7rem;
    font: inherit;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s, background 0.15s;
}
.preset:hover {
    color: var(--ink);
}
.preset.active {
    color: #fff;
    background: #20202a;
    border-color: rgba(107, 91, 240, 0.55);
}

.panel {
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 16px;
    padding: 16px;
    transition: opacity 0.15s;
}
.panel.disabled {
    opacity: 0.5;
}
.ctrl {
    margin-bottom: 12px;
}
.ctrl:last-child {
    margin-bottom: 0;
}
.ctrl-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 6px;
}
.ctrl-head label {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    font-size: 0.85rem;
    font-weight: 600;
}
.val {
    font-size: 0.8rem;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
}
.cdot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
}
.cdot.r {
    background: #ef6f5e;
}
.cdot.g {
    background: #5bd6a0;
}
.cdot.b {
    background: #4da6ff;
}
.channels-label {
    margin: 16px 0 10px;
    padding-top: 14px;
    border-top: 1px solid var(--line);
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--muted);
}

input[type='range'] {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    border-radius: 999px;
    background: #2a2a31;
    outline: none;
    accent-color: var(--accent);
    cursor: pointer;
}
input[type='range']::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.5);
    cursor: pointer;
}
input[type='range']:disabled {
    cursor: default;
}
.range.r {
    accent-color: #ef6f5e;
}
.range.g {
    accent-color: #5bd6a0;
}
.range.b {
    accent-color: #4da6ff;
}

/* Color blindness section */
.cb {
    margin-top: 12px;
}
.cb-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
}
.cb-head h2 {
    font-size: 0.92rem;
    font-weight: 600;
}
.cb-tag {
    font-size: 0.66rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #b9aaff;
    background: rgba(107, 91, 240, 0.16);
    padding: 0.2rem 0.5rem;
    border-radius: 999px;
}
.cal-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
    margin-bottom: 14px;
    padding: 0.65rem 1rem;
    border-radius: 11px;
    border: 1px solid rgba(107, 91, 240, 0.5);
    background: rgba(107, 91, 240, 0.14);
    color: #c2b8ff;
    font: inherit;
    font-size: 0.86rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
}
.cal-btn:hover {
    background: rgba(107, 91, 240, 0.22);
}
.cal-btn svg {
    width: 16px;
    height: 16px;
}

.filter-list {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    margin-bottom: 14px;
}
.filter-opt {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    border: 1px solid transparent;
    background: transparent;
    color: var(--muted);
    border-radius: 9px;
    padding: 0.5rem 0.6rem;
    font: inherit;
    font-size: 0.84rem;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition: color 0.15s, background 0.15s;
}
.filter-opt:hover {
    color: var(--ink);
    background: #1c1c22;
}
.filter-opt.active {
    color: #fff;
    background: #1c1c22;
    border-color: rgba(107, 91, 240, 0.45);
}
.radio {
    flex: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 2px solid #43434c;
    position: relative;
    transition: border-color 0.15s;
}
.filter-opt.active .radio {
    border-color: var(--accent);
}
.filter-opt.active .radio::after {
    content: '';
    position: absolute;
    inset: 2px;
    border-radius: 50%;
    background: var(--accent);
}
.ctrl.disabled {
    opacity: 0.5;
}
.cb-note {
    margin-top: 10px;
    font-size: 0.78rem;
    line-height: 1.45;
    color: var(--muted);
}

.option-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 12px;
    padding: 0.7rem 0.9rem;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 12px;
    font-size: 0.86rem;
    font-weight: 600;
}
.mini-toggle {
    position: relative;
    width: 40px;
    height: 23px;
    border-radius: 999px;
    border: none;
    background: #2a2a31;
    cursor: pointer;
    transition: background 0.18s;
    flex: none;
}
.mini-toggle.on {
    background: var(--accent);
}
.knob2 {
    position: absolute;
    top: 2.5px;
    left: 2.5px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
    transition: transform 0.18s cubic-bezier(0.3, 0.8, 0.3, 1);
}
.mini-toggle.on .knob2 {
    transform: translateX(17px);
}

.foot {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-top: auto;
}
.btn {
    font: inherit;
    font-weight: 600;
    font-size: 0.85rem;
    border-radius: 11px;
    padding: 0.6rem 1rem;
    cursor: pointer;
    border: 1px solid var(--line);
}
.btn.ghost {
    background: var(--panel);
    color: var(--ink);
}
.btn.ghost:hover {
    background: #20202a;
}
.msg {
    font-size: 0.8rem;
    color: var(--muted);
}
.msg.err {
    color: #ef7d8a;
}
</style>
