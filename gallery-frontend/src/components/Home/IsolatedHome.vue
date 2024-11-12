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
    <Home isolation-id="`idid`" />
  </v-overlay>
</template>
<script setup lang="ts">
import Home from './Home.vue'
</script>
