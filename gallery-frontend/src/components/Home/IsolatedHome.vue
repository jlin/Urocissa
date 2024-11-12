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
    test1234
  </v-overlay>
</template>
<script setup lang="ts">
import { useModalStore } from '@/store/modalStore'

const modalStore = useModalStore()
</script>
