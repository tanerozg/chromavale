<script setup lang="ts">
import { Head, Link, useForm } from '@inertiajs/vue3';
import { ref, onMounted, computed } from 'vue';

type Platform = 'mac' | 'windows';
const detected = ref<Platform | null>(null);
const submitted = ref(false);

const form = useForm<{ email: string; platform: Platform | null }>({
    email: '',
    platform: null,
});

onMounted(() => {
    const ua = navigator.userAgent.toLowerCase();
    const platform = (
        (navigator as Navigator & { userAgentData?: { platform?: string } })
            .userAgentData?.platform || navigator.platform || ''
    ).toLowerCase();

    if (platform.includes('mac') || ua.includes('mac os')) {
        detected.value = 'mac';
    } else if (platform.includes('win') || ua.includes('windows')) {
        detected.value = 'windows';
    }
    form.platform = detected.value;
});

const detectedLabel = computed(() =>
    detected.value === 'mac'
        ? 'macOS'
        : detected.value === 'windows'
          ? 'Windows'
          : 'your device',
);

function choose(platform: Platform) {
    form.platform = platform;
    const field = document.getElementById('wl-email');
    field?.focus();
    field?.scrollIntoView({ behavior: 'smooth', block: 'center' });
}

function submit() {
    form.post('/waitlist', {
        preserveScroll: true,
        onSuccess: () => {
            submitted.value = true;
            form.reset('email');
        },
    });
}

const platforms = [
    {
        key: 'mac' as Platform,
        name: 'macOS',
        req: 'macOS 12 Monterey or later. Apple silicon and Intel.',
        size: 'Universal build',
    },
    {
        key: 'windows' as Platform,
        name: 'Windows',
        req: 'Windows 10 and 11, 64-bit.',
        size: 'x64 installer',
    },
];
</script>

<template>
    <Head title="Download ChromaVale - Free for Mac and Windows" />

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
                <span class="kicker">Download</span>
                <h1 class="title">Get ChromaVale, free.</h1>
                <p class="lede">
                    ChromaVale is launching soon for macOS and Windows. Join the
                    list and we will email you the moment your build is ready to
                    download. No spam, just the release.
                </p>
            </div>

            <!-- Waitlist signup -->
            <div class="signup">
                <div v-if="!submitted" class="signup-inner">
                    <div class="signup-copy">
                        <span class="badge">
                            <span class="badge-dot"></span>
                            Launching soon
                        </span>
                        <h2 class="signup-title">
                            Be first to tune your screen.
                        </h2>
                        <p class="signup-sub">
                            We detected
                            <strong>{{ detectedLabel }}</strong>. We will send
                            the right build for you.
                        </p>
                    </div>
                    <form class="signup-form" @submit.prevent="submit">
                        <div class="field">
                            <input
                                id="wl-email"
                                v-model="form.email"
                                type="email"
                                inputmode="email"
                                autocomplete="email"
                                placeholder="you@example.com"
                                :class="{ 'has-error': form.errors.email }"
                                @input="form.clearErrors('email')"
                            />
                            <button
                                type="submit"
                                class="btn btn-ink"
                                :disabled="form.processing"
                            >
                                {{ form.processing ? 'Adding...' : 'Notify me' }}
                            </button>
                        </div>
                        <p v-if="form.errors.email" class="error">
                            {{ form.errors.email }}
                        </p>
                        <p v-else class="field-note">
                            By joining you agree to receive a single launch
                            email.
                        </p>
                    </form>
                </div>

                <div v-else class="success">
                    <span class="success-mark">
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
                    <h2 class="signup-title">You are on the list.</h2>
                    <p class="signup-sub">
                        We will email you the {{ detectedLabel }} build as soon
                        as ChromaVale is ready. Talk soon.
                    </p>
                    <Link href="/" class="btn btn-soft">Back to home</Link>
                </div>
            </div>

            <!-- Platform requirements -->
            <div class="platforms">
                <article
                    v-for="p in platforms"
                    :key="p.key"
                    class="plat"
                    :class="{ 'plat-on': detected === p.key }"
                >
                    <div class="plat-head">
                        <span class="plat-ico">
                            <svg
                                v-if="p.key === 'mac'"
                                viewBox="0 0 16 16"
                                fill="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    d="M11.18 8.5c-.02-1.5 1.22-2.22 1.28-2.26-.7-1.02-1.79-1.16-2.18-1.18-.93-.09-1.81.54-2.28.54-.47 0-1.2-.53-1.97-.51-1.01.01-1.94.59-2.46 1.49-1.05 1.82-.27 4.51.75 5.99.5.72 1.1 1.53 1.88 1.5.75-.03 1.04-.49 1.95-.49.91 0 1.17.49 1.97.47.81-.01 1.33-.74 1.83-1.46.58-.84.82-1.65.83-1.69-.02-.01-1.6-.61-1.62-2.43Zm-1.5-4.47c.41-.5.69-1.2.61-1.9-.59.02-1.31.39-1.74.89-.38.44-.72 1.15-.63 1.83.66.05 1.34-.33 1.76-.82Z"
                                />
                            </svg>
                            <svg
                                v-else
                                viewBox="0 0 16 16"
                                fill="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    d="M0 2.2 6.5 1.3v6.2H0V2.2Zm0 11.6 6.5.9V8.5H0v5.3ZM7.3 1.2 16 0v7.5H7.3V1.2Zm0 13.6L16 16V8.5H7.3v6.3Z"
                                />
                            </svg>
                        </span>
                        <div>
                            <h3 class="plat-name">
                                {{ p.name }}
                                <span v-if="detected === p.key" class="here">
                                    Your device
                                </span>
                            </h3>
                            <p class="plat-size">{{ p.size }}</p>
                        </div>
                    </div>
                    <p class="plat-req">{{ p.req }}</p>
                    <button class="btn btn-soft btn-block" @click="choose(p.key)">
                        Get notified for {{ p.name }}
                    </button>
                </article>
            </div>

            <p class="foot-note">
                Already using ChromaVale on one device? Your Pro license works on
                both macOS and Windows.
            </p>
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

    min-height: 100vh;
    background: #fff;
    color: var(--ink);
    font-family: 'Instrument Sans', ui-sans-serif, system-ui, sans-serif;
    -webkit-font-smoothing: antialiased;
}

/* Buttons */
.btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font: inherit;
    font-weight: 600;
    letter-spacing: -0.01em;
    border: none;
    border-radius: 13px;
    padding: 0.75rem 1.2rem;
    font-size: 0.95rem;
    cursor: pointer;
    text-decoration: none;
    white-space: nowrap;
    transition:
        transform 0.11s cubic-bezier(0.2, 0.8, 0.2, 1),
        box-shadow 0.11s,
        background 0.15s;
    --press: 4px;
}
.btn:active {
    transform: translateY(var(--press));
}
.btn-block {
    width: 100%;
}
.btn-ink {
    background: var(--ink);
    color: #fff;
    box-shadow: 0 var(--press) 0 0 #000;
}
.btn-ink:hover {
    background: #26262e;
}
.btn-ink:active {
    box-shadow: 0 0 0 0 #000;
}
.btn-ink:disabled {
    opacity: 0.7;
    cursor: default;
    transform: none;
}
.btn-soft {
    background: #fff;
    color: var(--ink);
    box-shadow:
        0 var(--press) 0 0 var(--line-strong),
        inset 0 0 0 1px var(--line-strong);
}
.btn-soft:hover {
    background: var(--paper-2);
}
.btn-soft:active {
    box-shadow: inset 0 0 0 1px var(--line-strong);
}

/* Bar */
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
    max-width: 800px;
    margin-inline: auto;
    padding: 3.5rem 24px 5rem;
}
.intro {
    text-align: center;
    max-width: 38rem;
    margin: 0 auto 2.5rem;
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
    font-size: clamp(2rem, 5vw, 2.9rem);
    line-height: 1.05;
    letter-spacing: -0.03em;
    font-weight: 600;
}
.lede {
    margin-top: 1rem;
    font-size: 1.08rem;
    line-height: 1.6;
    color: var(--ink-soft);
}

/* Signup */
.signup {
    border-radius: 24px;
    border: 1px solid var(--line);
    background:
        radial-gradient(
            130% 130% at 100% 0%,
            rgba(91, 75, 224, 0.06),
            transparent 55%
        ),
        var(--paper-2);
    padding: 2.25rem;
    margin-bottom: 2rem;
}
.signup-inner {
    display: grid;
    gap: 1.5rem;
}
.badge {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--accent);
    background: rgba(91, 75, 224, 0.09);
    padding: 0.35rem 0.7rem;
    border-radius: 999px;
}
.badge-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
}
.signup-title {
    margin-top: 1rem;
    font-size: 1.5rem;
    font-weight: 600;
    letter-spacing: -0.02em;
}
.signup-sub {
    margin-top: 0.5rem;
    font-size: 1rem;
    line-height: 1.55;
    color: var(--muted);
}
.signup-sub strong {
    color: var(--ink);
    font-weight: 600;
}
.field {
    display: flex;
    gap: 0.6rem;
}
.field input {
    flex: 1;
    min-width: 0;
    font: inherit;
    font-size: 0.98rem;
    padding: 0.75rem 1rem;
    border-radius: 13px;
    border: 1px solid var(--line-strong);
    background: #fff;
    color: var(--ink);
    outline: none;
    transition:
        border-color 0.15s ease,
        box-shadow 0.15s ease;
}
.field input::placeholder {
    color: #a1a1aa;
}
.field input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px rgba(91, 75, 224, 0.16);
}
.field input.has-error {
    border-color: #e2526a;
}
.error {
    margin-top: 0.6rem;
    font-size: 0.86rem;
    color: #cf3b56;
}
.field-note {
    margin-top: 0.6rem;
    font-size: 0.82rem;
    color: var(--muted);
}

@media (min-width: 720px) {
    .signup-inner {
        grid-template-columns: 1fr 1fr;
        align-items: center;
        gap: 2.5rem;
    }
}

/* Success */
.success {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
}
.success-mark {
    display: grid;
    place-items: center;
    width: 52px;
    height: 52px;
    border-radius: 50%;
    background: rgba(91, 75, 224, 0.12);
    color: var(--accent);
    margin-bottom: 0.5rem;
}
.success-mark svg {
    width: 26px;
    height: 26px;
}
.success .btn {
    margin-top: 1.5rem;
}

/* Platforms */
.platforms {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
}
.plat {
    padding: 1.5rem;
    border-radius: 18px;
    border: 1px solid var(--line);
    background: #fff;
    display: flex;
    flex-direction: column;
}
.plat-on {
    border-color: rgba(91, 75, 224, 0.35);
    box-shadow: 0 16px 40px -28px rgba(91, 75, 224, 0.5);
}
.plat-head {
    display: flex;
    align-items: center;
    gap: 0.85rem;
    margin-bottom: 1rem;
}
.plat-ico {
    display: grid;
    place-items: center;
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: var(--paper-2);
    border: 1px solid var(--line);
    color: var(--ink);
    flex: none;
}
.plat-ico svg {
    width: 20px;
    height: 20px;
}
.plat-name {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 1.1rem;
    font-weight: 600;
    letter-spacing: -0.01em;
}
.here {
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--accent);
    background: rgba(91, 75, 224, 0.1);
    padding: 0.18rem 0.45rem;
    border-radius: 999px;
}
.plat-size {
    margin-top: 0.15rem;
    font-size: 0.84rem;
    color: var(--muted);
}
.plat-req {
    font-size: 0.92rem;
    line-height: 1.5;
    color: var(--ink-soft);
    margin-bottom: 1.25rem;
    flex: 1;
}
.foot-note {
    margin-top: 2rem;
    text-align: center;
    font-size: 0.88rem;
    color: var(--muted);
}

@media (min-width: 680px) {
    .platforms {
        grid-template-columns: 1fr 1fr;
    }
}
</style>
