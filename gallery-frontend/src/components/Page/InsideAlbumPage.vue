<template>
  <HomePage />
</template>

<script setup lang="ts">
import HomePage from '@/components/Home/mainHome.vue'
defineProps<{ id: string; hash?: string }>()
</script>
