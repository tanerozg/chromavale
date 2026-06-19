<script setup lang="ts">
import { Link } from '@inertiajs/vue3';
import { home } from '@/routes';

defineProps<{
    title?: string;
    description?: string;
}>();
</script>

<template>
    <div
        class="relative flex min-h-svh flex-col items-center justify-center gap-6 overflow-hidden bg-background p-6 md:p-10"
    >
        <!-- Subtle brand halo -->
        <div
            aria-hidden="true"
            class="pointer-events-none absolute inset-x-0 top-0 h-72 bg-[radial-gradient(600px_280px_at_50%_-80px,rgba(91,75,224,0.10),transparent_70%)]"
        ></div>

        <div class="relative w-full max-w-sm">
            <div class="flex flex-col gap-8">
                <div class="flex flex-col items-center gap-5">
                    <Link
                        :href="home()"
                        class="flex flex-col items-center gap-3"
                    >
                        <span
                            class="flex h-12 w-12 items-center justify-center text-foreground"
                        >
                            <svg
                                viewBox="0 0 32 32"
                                fill="none"
                                class="h-full w-full"
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
                                    fill="url(#authPrism)"
                                />
                                <defs>
                                    <linearGradient
                                        id="authPrism"
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
                        <span
                            class="text-lg font-semibold tracking-tight text-foreground"
                        >
                            ChromaVale
                        </span>
                    </Link>
                    <div class="space-y-2 text-center">
                        <h1 class="text-xl font-semibold tracking-tight">
                            {{ title }}
                        </h1>
                        <p class="text-center text-sm text-muted-foreground">
                            {{ description }}
                        </p>
                    </div>
                </div>
                <slot />
            </div>

            <div
                class="mx-auto mt-8 h-1 w-28 rounded-full bg-[linear-gradient(90deg,#ef6f5e,#f2b04a_26%,#5bd6a0_52%,#4da6ff_76%,#9b6be5)] opacity-80"
            ></div>
        </div>
    </div>
</template>
