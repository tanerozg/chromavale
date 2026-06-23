<script setup lang="ts">
import { Head, Link } from '@inertiajs/vue3';
</script>

<template>
    <Head title="Thanks for going Pro - ChromaVale" />

    <div class="page">
        <div class="card">
            <span class="mark">
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
            <h1 class="title">Thanks for going Pro</h1>
            <p class="lead">
                Your license key is on its way to your inbox. Open ChromaVale,
                click <strong>Unlock Pro</strong>, and paste it in to activate
                all features on both macOS and Windows.
            </p>
            <div class="cta">
                <Link href="/download" class="btn btn-ink">Get the app</Link>
                <Link href="/" class="btn btn-soft">Back to home</Link>
            </div>
            <p class="fine">
                The key works on up to 5 devices. Keep the email so you can
                re-activate later.
            </p>
        </div>
    </div>
</template>

<style scoped>
.page {
    --ink: #16161a;
    --ink-soft: #3f3f46;
    --muted: #71717a;
    --line-strong: #e0e0e4;
    --accent: #5b4be0;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    background: #fff;
    color: var(--ink);
    font-family: 'Instrument Sans', ui-sans-serif, system-ui, sans-serif;
    -webkit-font-smoothing: antialiased;
}
.card {
    max-width: 30rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
}
.mark {
    display: grid;
    place-items: center;
    width: 60px;
    height: 60px;
    border-radius: 50%;
    background: rgba(91, 75, 224, 0.12);
    color: var(--accent);
    margin-bottom: 1.2rem;
}
.mark svg {
    width: 30px;
    height: 30px;
}
.title {
    font-size: clamp(1.8rem, 5vw, 2.4rem);
    line-height: 1.1;
    letter-spacing: -0.03em;
    font-weight: 600;
}
.lead {
    margin-top: 1rem;
    font-size: 1.05rem;
    line-height: 1.6;
    color: var(--ink-soft);
}
.lead strong {
    color: var(--ink);
}
.cta {
    margin-top: 1.8rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    justify-content: center;
}
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.85rem 1.4rem;
    border-radius: 13px;
    font-weight: 600;
    font-size: 1rem;
    text-decoration: none;
    --press: 4px;
    transition: transform 0.11s, box-shadow 0.11s, background 0.15s;
}
.btn:active {
    transform: translateY(var(--press));
}
.btn-ink {
    background: var(--ink);
    color: #fff;
    box-shadow: 0 var(--press) 0 0 #000;
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
.fine {
    margin-top: 1.2rem;
    font-size: 0.85rem;
    color: var(--muted);
}
</style>
