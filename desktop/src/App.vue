<script setup lang="ts">
import { reactive, ref, watch, onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';

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
    } else {
        await reset();
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

onMounted(apply);
onBeforeUnmount(() => {
    if (applyTimer) clearTimeout(applyTimer);
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
            >
                <span class="dot"></span>
                {{ enabled ? 'On' : 'Off' }}
            </button>
        </header>

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

        <footer class="foot">
            <button class="btn ghost" @click="reset">Reset screen</button>
            <span v-if="error" class="msg err">{{ error }}</span>
            <span v-else class="msg">{{ status }}</span>
        </footer>
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
