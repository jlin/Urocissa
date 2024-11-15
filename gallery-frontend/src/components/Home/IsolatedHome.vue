<template>
  <v-overlay
    :model-value="true"
    :height="'100%'"
    :width="'100%'"
    class="d-flex"
    id="view-page"
    persistent
    transition="false"
  >
    <Home isolationId="subId" :title="props.title" />
  </v-overlay>
</template>
<script setup lang="ts">
import Home from './Home.vue'

const props = defineProps<{
  title: string
}>()
</script>
