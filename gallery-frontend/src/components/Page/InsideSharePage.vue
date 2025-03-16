<template>
  <HomePage />
</template>

<script setup lang="ts">
import HomeMain from '@/components/Home/Page/HomeMain.vue'
defineProps<{ id: string; hash?: string }>()
</script>
