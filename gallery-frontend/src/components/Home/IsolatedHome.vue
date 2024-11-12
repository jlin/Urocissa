<template>
  <v-overlay
    :model-value="modalStore.showIsolatedHomeModal"
    :height="'100%'"
    :width="'100%'"
    class="d-flex"
    id="view-page"
    persistent
    transition="false"
  >
    <Home isolation-id="`idid`" />
  </v-overlay>
</template>
<script setup lang="ts">
import { useModalStore } from '@/store/modalStore'
import Home from './Home.vue'

const modalStore = useModalStore('')
</script>
