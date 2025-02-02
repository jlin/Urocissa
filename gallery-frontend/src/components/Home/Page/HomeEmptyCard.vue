<template>
  <v-container class="d-flex align-center justify-center" fluid>
    <v-hover v-slot="{ isHovering, props: hoverProps }">
      <v-card
        v-if="hasHoveringEffect"
        class="pa-4 text-center mx-auto hover-cursor"
        :style="{
          border: isHovering && hasHoveringEffect ? '2px solid #BDBDBD' : '2px solid transparent'
        }"
        :elevation="isHovering && hasHoveringEffect ? 12 : 2"
        max-width="600"
        rounded="lg"
        width="100%"
        v-bind="hoverProps"
        @click="clickEmptyCard()"
      >
        <v-icon class="mb-5" color="grey" size="100"> mdi-image-plus </v-icon>
        <v-card-item>
          <v-card-subtitle>
            Wow, so empty!<br />
            {{ computedMessage }}
          </v-card-subtitle>
        </v-card-item>
      </v-card>
      <v-card
        v-else
        class="pa-4 text-center mx-auto"
        :style="{
          border: '2px solid transparent'
        }"
        :elevation="2"
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
import { IsolationId } from '@/script/common/types'
import { useCollectionStore } from '@/store/collectionStore'
import { useModalStore } from '@/store/modalStore'
import { useUploadStore } from '@/store/uploadStore'
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const props = defineProps<{
  isolationId: IsolationId
}>()

const route = useRoute()

const uploadStore = useUploadStore(props.isolationId)
const collectionStore = useCollectionStore(props.isolationId)
const modalStore = useModalStore('mainId')

const hasHoveringEffect = computed(() => {
  const path = route.path

  if (path.startsWith('/favorite')) {
    return false
  } else if (path.startsWith('/archived')) {
    return false
  } else if (path.startsWith('/trashed')) {
    return false
  } else if (route.meta.isReadPage) {
    return collectionStore.editModeOn ? false : true // Inside the component for adding photos
  } else if (path.startsWith('/albums')) {
    return true
  } else if (path.startsWith('/all')) {
    return true
  } else if (path.startsWith('/view')) {
    return false
  } else {
    return true
  }
})

const computedMessage = computed(() => {
  const path = route.path

  if (path.startsWith('/favorite')) {
    return 'Add your favorite photos and videos here!'
  } else if (path.startsWith('/archived')) {
    return 'Archived photos won’t appear on the home page.'
  } else if (path.startsWith('/trashed')) {
    return 'Deleted photos and videos appear here.'
  } else if (path.startsWith('/all')) {
    return 'Upload some photos here!'
  } else if (route.meta.isReadPage) {
    return collectionStore.editModeOn
      ? 'All photos are already added!' // Inside the component for adding photos
      : 'Add photos to this album!'
  } else if (path.startsWith('/albums')) {
    return 'Create some albums here!'
  } else {
    return 'Upload some photos here!'
  }
})
const clickEmptyCard = () => {
  const path = route.path

  if (route.meta.isReadPage) {
    modalStore.showHomeTempModal = true
  } else if (path.startsWith('/albums')) {
    modalStore.showCreateAlbumsModal = true
  } else {
    uploadStore.triggerFileInput()
  }
}
</script>
<style scoped>
.hover-cursor {
  cursor: pointer !important;
}
</style>
