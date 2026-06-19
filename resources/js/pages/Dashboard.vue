<script setup lang="ts">
import { Head } from '@inertiajs/vue3';
import { ref, computed } from 'vue';
import { dashboard } from '@/routes';

defineOptions({
    layout: {
        breadcrumbs: [
            {
                title: 'Dashboard',
                href: dashboard(),
            },
        ],
    },
});

const profiles = [
    {
        name: 'Comfort',
        desc: 'Warm, easy on the eyes',
        bar: 'linear-gradient(90deg,#f2b04a,#ef6f5e)',
        temp: '4200 K',
        sat: '96%',
    },
    {
        name: 'Reading',
        desc: 'Soft contrast for long text',
        bar: 'linear-gradient(90deg,#e9c98a,#d9a36b)',
        temp: '4600 K',
        sat: '88%',
    },
    {
        name: 'Night',
        desc: 'Deep warm, low blue light',
        bar: 'linear-gradient(90deg,#c97b3a,#9a4f2a)',
        temp: '3200 K',
        sat: '80%',
    },
    {
        name: 'Vivid',
        desc: 'Punchy and saturated',
        bar: 'linear-gradient(90deg,#ef6f5e,#5bd6a0,#4da6ff)',
        temp: '6500 K',
        sat: '146%',
    },
    {
        name: 'Grayscale',
        desc: 'Full focus, no color',
        bar: 'linear-gradient(90deg,#9a9a9f,#cfcfd4)',
        temp: '6500 K',
        sat: '0%',
    },
];

const activeProfile = ref('Comfort');
const active = computed(
    () => profiles.find((p) => p.name === activeProfile.value) ?? profiles[0],
);
const correctionOn = ref(true);

const appFilters = [
    { app: 'Code editor', profile: 'Night', on: true },
    { app: 'Photo editor', profile: 'Off', on: false },
    { app: 'Web browser', profile: 'Comfort', on: true },
];
</script>

<template>
    <Head title="Dashboard" />

    <div class="flex h-full flex-1 flex-col gap-4 p-4">
        <!-- Status cards -->
        <div class="grid auto-rows-min gap-4 md:grid-cols-3">
            <div class="rounded-xl border border-border bg-card p-5">
                <p
                    class="text-xs font-semibold tracking-wider text-muted-foreground uppercase"
                >
                    Active profile
                </p>
                <div class="mt-3 flex items-center gap-3">
                    <span
                        class="h-8 w-8 rounded-lg ring-1 ring-black/5"
                        :style="{ background: active.bar }"
                    ></span>
                    <div>
                        <p class="text-lg leading-tight font-semibold">
                            {{ active.name }}
                        </p>
                        <p class="text-xs text-muted-foreground">
                            {{ active.temp }} · {{ active.sat }} saturation
                        </p>
                    </div>
                </div>
            </div>

            <div class="rounded-xl border border-border bg-card p-5">
                <p
                    class="text-xs font-semibold tracking-wider text-muted-foreground uppercase"
                >
                    Color-blind correction
                </p>
                <div class="mt-3 flex items-center justify-between">
                    <div>
                        <p class="text-lg leading-tight font-semibold">
                            Deuteranopia
                        </p>
                        <p class="text-xs text-muted-foreground">
                            {{ correctionOn ? 'Correction active' : 'Paused' }}
                        </p>
                    </div>
                    <button
                        type="button"
                        role="switch"
                        :aria-checked="correctionOn"
                        class="relative h-6 w-11 rounded-full transition-colors"
                        :class="correctionOn ? 'bg-primary' : 'bg-muted'"
                        @click="correctionOn = !correctionOn"
                    >
                        <span
                            class="absolute top-0.5 left-0.5 h-5 w-5 rounded-full bg-white shadow transition-transform"
                            :class="correctionOn ? 'translate-x-5' : ''"
                        ></span>
                    </button>
                </div>
            </div>

            <div class="rounded-xl border border-border bg-card p-5">
                <p
                    class="text-xs font-semibold tracking-wider text-muted-foreground uppercase"
                >
                    Next schedule
                </p>
                <div class="mt-3">
                    <p class="text-lg leading-tight font-semibold">
                        Night at 21:00
                    </p>
                    <p class="text-xs text-muted-foreground">
                        Auto-switches every evening
                    </p>
                </div>
            </div>
        </div>

        <div class="grid flex-1 gap-4 lg:grid-cols-3">
            <!-- Profiles -->
            <div
                class="rounded-xl border border-border bg-card p-5 lg:col-span-2"
            >
                <div class="mb-4 flex items-center justify-between">
                    <h2 class="font-semibold">Your profiles</h2>
                    <span class="text-xs text-muted-foreground">
                        {{ profiles.length }} saved
                    </span>
                </div>
                <div class="grid gap-3 sm:grid-cols-2">
                    <button
                        v-for="p in profiles"
                        :key="p.name"
                        type="button"
                        class="rounded-lg border p-4 text-left transition-colors"
                        :class="
                            activeProfile === p.name
                                ? 'border-primary/50 bg-accent'
                                : 'border-border hover:bg-accent/50'
                        "
                        @click="activeProfile = p.name"
                    >
                        <div class="flex items-center justify-between">
                            <span class="font-medium">{{ p.name }}</span>
                            <span
                                v-if="activeProfile === p.name"
                                class="text-xs font-semibold text-primary"
                            >
                                Active
                            </span>
                        </div>
                        <p class="mt-0.5 text-xs text-muted-foreground">
                            {{ p.desc }}
                        </p>
                        <span
                            class="mt-3 block h-2 w-full rounded-full"
                            :style="{ background: p.bar }"
                        ></span>
                    </button>
                </div>
            </div>

            <!-- Per-app filters -->
            <div class="rounded-xl border border-border bg-card p-5">
                <h2 class="mb-4 font-semibold">Per-app filters</h2>
                <ul class="flex flex-col gap-3">
                    <li
                        v-for="f in appFilters"
                        :key="f.app"
                        class="flex items-center justify-between rounded-lg border border-border px-3 py-2.5"
                    >
                        <div>
                            <p class="text-sm font-medium">{{ f.app }}</p>
                            <p class="text-xs text-muted-foreground">
                                {{ f.on ? f.profile : 'No filter' }}
                            </p>
                        </div>
                        <span
                            class="h-2.5 w-2.5 rounded-full"
                            :class="f.on ? 'bg-primary' : 'bg-muted-foreground/40'"
                        ></span>
                    </li>
                </ul>
                <p class="mt-4 text-xs text-muted-foreground">
                    Filters apply only to the listed apps. Everything else uses
                    your active profile.
                </p>
            </div>
        </div>
    </div>
</template>
