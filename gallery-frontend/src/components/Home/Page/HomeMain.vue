<template>
  <Home isolation-id="mainId" :basic-string="props.basicString">
    <template #reading-bar>
      <NavBar />
    </template>
  </Home>
</template>
<script setup lang="ts">
import Home from './Home.vue'
import NavBar from '@/components/NavBar/NavBar.vue'

const props = defineProps<{
  basicString: string | null
}>()
</script>
