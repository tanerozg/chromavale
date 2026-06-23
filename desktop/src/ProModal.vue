<script setup lang="ts">
import { ref } from 'vue';
import { fetch as tauriFetch } from '@tauri-apps/plugin-http';
import { open as openUrl } from '@tauri-apps/plugin-shell';

const props = defineProps<{ deviceId: string }>();
const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'activated', result: { key: string; email: string }): void;
}>();

const SITE = 'https://chromavale.com';

const input = ref('');
const error = ref('');
const busy = ref(false);

const proFeatures = [
    'Personal color-blind calibration and correction',
    'Per-app automatic profiles',
    'Night warmth schedule',
    'Launch with Windows',
];

async function activate() {
    error.value = '';
    const key = input.value.trim().toUpperCase();
    if (!key) {
        error.value = 'Enter your license key.';
        return;
    }
    busy.value = true;
    try {
        const res = await tauriFetch(`${SITE}/api/license/activate`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ key, device_id: props.deviceId }),
        });
        const data = await res.json();
        if (res.ok && data.valid) {
            emit('activated', { key, email: data.email ?? '' });
            return;
        }
        error.value =
            data.reason === 'device_limit'
                ? 'This license has reached its device limit.'
                : data.reason === 'inactive'
                  ? 'This license is no longer active.'
                  : 'License key not found.';
    } catch {
        error.value = 'Could not reach the license server. Check your connection.';
    } finally {
        busy.value = false;
    }
}

function buy() {
    openUrl(`${SITE}/#pricing`);
}
</script>

<template>
    <div class="overlay" @click.self="emit('close')">
        <div class="modal">
            <button class="close" @click="emit('close')" aria-label="Close">
                &times;
            </button>
            <span class="badge">ChromaVale Pro</span>
            <h2 class="title">Unlock the full toolkit</h2>
            <ul class="features">
                <li v-for="f in proFeatures" :key="f">
                    <span class="check"></span>{{ f }}
                </li>
            </ul>

            <label class="field-label">Have a license key?</label>
            <div class="field">
                <input
                    v-model="input"
                    type="text"
                    placeholder="CV-XXXX-XXXX-XXXX-XXXX"
                    spellcheck="false"
                    autocomplete="off"
                    @keyup.enter="activate"
                />
                <button class="btn ink" :disabled="busy" @click="activate">
                    {{ busy ? '...' : 'Activate' }}
                </button>
            </div>
            <p v-if="error" class="error">{{ error }}</p>

            <div class="divider"><span>or</span></div>

            <button class="btn buy" @click="buy">Buy Pro on chromavale.com</button>
            <p class="fine">One-time purchase. Works on macOS and Windows.</p>
        </div>
    </div>
</template>

<style scoped>
.overlay {
    position: fixed;
    inset: 0;
    z-index: 60;
    background: rgba(8, 8, 11, 0.66);
    backdrop-filter: blur(3px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
}
.modal {
    position: relative;
    width: 100%;
    max-width: 340px;
    background: #15151a;
    border: 1px solid #2a2a31;
    border-radius: 18px;
    padding: 24px;
    color: #f4f4f6;
    font-family: 'Segoe UI', ui-sans-serif, system-ui, sans-serif;
}
.close {
    position: absolute;
    top: 12px;
    right: 14px;
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
.badge {
    display: inline-block;
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #c2b8ff;
    background: rgba(107, 91, 240, 0.18);
    padding: 0.25rem 0.6rem;
    border-radius: 999px;
}
.title {
    margin-top: 0.8rem;
    font-size: 1.3rem;
    font-weight: 600;
    letter-spacing: -0.02em;
}
.features {
    list-style: none;
    margin: 1.1rem 0 1.4rem;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
}
.features li {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    font-size: 0.88rem;
    color: #cfcfd6;
}
.check {
    flex: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: rgba(107, 91, 240, 0.16);
    position: relative;
}
.check::after {
    content: '';
    position: absolute;
    left: 6px;
    top: 4px;
    width: 4px;
    height: 8px;
    border: solid #6b5bf0;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
}
.field-label {
    font-size: 0.78rem;
    font-weight: 600;
    color: #9a9aa3;
}
.field {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;
}
.field input {
    flex: 1;
    min-width: 0;
    font: inherit;
    font-size: 0.85rem;
    letter-spacing: 0.04em;
    padding: 0.6rem 0.7rem;
    border-radius: 10px;
    border: 1px solid #2a2a31;
    background: #1c1c22;
    color: #fff;
    outline: none;
}
.field input:focus {
    border-color: #6b5bf0;
}
.btn {
    font: inherit;
    font-weight: 600;
    font-size: 0.88rem;
    border: none;
    border-radius: 10px;
    padding: 0.6rem 0.9rem;
    cursor: pointer;
}
.btn.ink {
    background: #6b5bf0;
    color: #fff;
    flex: none;
}
.btn.ink:disabled {
    opacity: 0.6;
    cursor: default;
}
.error {
    margin-top: 0.6rem;
    font-size: 0.8rem;
    color: #ef7d8a;
}
.divider {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin: 1.2rem 0;
    color: #6a6a73;
    font-size: 0.74rem;
}
.divider::before,
.divider::after {
    content: '';
    flex: 1;
    height: 1px;
    background: #2a2a31;
}
.btn.buy {
    width: 100%;
    background: #fff;
    color: #16161a;
}
.btn.buy:hover {
    background: #f0f0f2;
}
.fine {
    margin-top: 0.8rem;
    text-align: center;
    font-size: 0.76rem;
    color: #7a7a82;
}
</style>
