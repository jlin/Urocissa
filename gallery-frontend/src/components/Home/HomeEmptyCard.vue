<template>
  <v-container class="d-flex align-center justify-center" fluid>
    <v-hover v-slot="{ isHovering, props }">
      <v-card
        class="pa-4 text-center mx-auto hover-cursor"
        :style="{
          border: isHovering ? '2px solid #BDBDBD' : '2px solid transparent'
        }"
        :elevation="isHovering ? 12 : 2"
        max-width="600"
        rounded="lg"
        width="100%"
        v-bind="props"
      >
        <v-icon class="mb-5" color="grey" size="100"> mdi-image-plus </v-icon>
        <v-card-item>
          <v-card-subtitle>
            Wow, so empty!<br />
            {{ computedMessage }}
          </v-card-subtitle>
        </v-card-item>
      </v-card>
    </v-hover>
  </v-container>
</template>
<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
const route = useRoute()

const computedMessage = computed(() => {
  const path = route.path

  if (path.startsWith('/favorite')) {
    return 'Quickly find your favorite photos and videos here!'
  } else if (path.startsWith('/archived')) {
    return 'Archived photos won’t appear on the home page.'
  } else if (path.startsWith('/trashed')) {
    return 'Deleted photos and videos will only appear here.'
  } else if (path.startsWith('/albums')) {
    return 'The album feature is still under development!'
  } else if (path.startsWith('/all')) {
    return 'Try uploading some photos here!'
  } else if (path.startsWith('/album-') && path.includes('/')) {
    return 'Add some photos to this album!'
  } else {
    return 'Try uploading some photos here!'
  }
})
</script>
<style scoped>
.hover-cursor {
  cursor: pointer !important;
}
</style>
