<template>
  <HomePage />
</template>

<script setup lang="ts">
import HomePage from '@/components/Home/Home.vue'
defineProps<{ id: string; hash?: string }>()
</script>
