<script setup lang="ts">
import type { HTMLAttributes } from 'vue';

defineOptions({
    inheritAttrs: false,
});

type Props = {
    className?: HTMLAttributes['class'];
};

defineProps<Props>();
</script>

<template>
    <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 32 32"
        :class="className"
        v-bind="$attrs"
    >
        <circle
            cx="16"
            cy="16"
            r="13"
            fill="none"
            stroke="currentColor"
            stroke-width="2.6"
        />
        <circle cx="16" cy="16" r="6.2" fill="currentColor" />
    </svg>
</template>
