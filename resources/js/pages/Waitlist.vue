<script setup lang="ts">
import { Head } from '@inertiajs/vue3';

type Subscriber = {
    id: number;
    email: string;
    platform: 'mac' | 'windows' | null;
    created_at: string;
};

defineProps<{
    subscribers: Subscriber[];
    stats: {
        total: number;
        mac: number;
        windows: number;
        unknown: number;
    };
}>();

defineOptions({
    layout: {
        breadcrumbs: [
            {
                title: 'Waitlist',
                href: '/admin/waitlist',
            },
        ],
    },
});

function platformLabel(platform: Subscriber['platform']) {
    if (platform === 'mac') return 'macOS';
    if (platform === 'windows') return 'Windows';
    return 'Unknown';
}

function formatDate(value: string) {
    return new Date(value).toLocaleString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
    });
}
</script>

<template>
    <Head title="Waitlist" />

    <div class="flex h-full flex-1 flex-col gap-4 p-4">
        <!-- Stats -->
        <div class="grid auto-rows-min gap-4 sm:grid-cols-3">
            <div class="rounded-xl border border-border bg-card p-5">
                <p
                    class="text-xs font-semibold tracking-wider text-muted-foreground uppercase"
                >
                    Total signups
                </p>
                <p class="mt-2 text-3xl font-semibold tracking-tight">
                    {{ stats.total }}
                </p>
            </div>
            <div class="rounded-xl border border-border bg-card p-5">
                <p
                    class="text-xs font-semibold tracking-wider text-muted-foreground uppercase"
                >
                    macOS
                </p>
                <p class="mt-2 text-3xl font-semibold tracking-tight">
                    {{ stats.mac }}
                </p>
            </div>
            <div class="rounded-xl border border-border bg-card p-5">
                <p
                    class="text-xs font-semibold tracking-wider text-muted-foreground uppercase"
                >
                    Windows
                </p>
                <p class="mt-2 text-3xl font-semibold tracking-tight">
                    {{ stats.windows }}
                </p>
            </div>
        </div>

        <!-- Table -->
        <div
            class="flex-1 overflow-hidden rounded-xl border border-border bg-card"
        >
            <div
                class="flex items-center justify-between border-b border-border px-5 py-4"
            >
                <h2 class="font-semibold">Subscribers</h2>
                <span class="text-xs text-muted-foreground">
                    Showing latest {{ subscribers.length }}
                </span>
            </div>

            <div v-if="subscribers.length" class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead>
                        <tr
                            class="border-b border-border text-left text-xs font-semibold tracking-wider text-muted-foreground uppercase"
                        >
                            <th class="px-5 py-3 font-semibold">Email</th>
                            <th class="px-5 py-3 font-semibold">Platform</th>
                            <th class="px-5 py-3 font-semibold">Joined</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr
                            v-for="s in subscribers"
                            :key="s.id"
                            class="border-b border-border/60 last:border-0 hover:bg-accent/40"
                        >
                            <td class="px-5 py-3 font-medium">{{ s.email }}</td>
                            <td class="px-5 py-3">
                                <span
                                    class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium"
                                    :class="
                                        s.platform
                                            ? 'bg-accent text-accent-foreground'
                                            : 'bg-muted text-muted-foreground'
                                    "
                                >
                                    {{ platformLabel(s.platform) }}
                                </span>
                            </td>
                            <td class="px-5 py-3 text-muted-foreground">
                                {{ formatDate(s.created_at) }}
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div
                v-else
                class="flex flex-col items-center justify-center gap-2 px-5 py-16 text-center"
            >
                <p class="font-medium">No signups yet</p>
                <p class="max-w-sm text-sm text-muted-foreground">
                    When visitors join the waitlist from the download page,
                    they will appear here.
                </p>
            </div>
        </div>
    </div>
</template>
