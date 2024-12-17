<template>
  <v-toolbar
    flat
    height="2"
    class="no-select"
    :style="{
      backgroundColor: '#212121'
    }"
  >
  </v-toolbar>
  <InfoBar v-if="!collectionStore.editModeOn" />
  <!-- If collectionStore.editModeOn === true then show the Editbar -->
  <EditBar v-else />
</template>

<script setup lang="ts">
import { ref, type Ref, watchEffect } from 'vue'
import { useRoute } from 'vue-router'

import { useCollectionStore } from '@/store/collectionStore'
import EditBar from '@/components/NavBar/EditBar.vue'
import InfoBar from '@/components/NavBar/InfoBar.vue'

import { useUploadStore } from '@/store/uploadStore'

const uploadStore = useUploadStore('mainId')
const collectionStore = useCollectionStore('mainId')
const route = useRoute()

const searchQuery = ref('')
const fileInput: Ref<HTMLInputElement | null> = ref(null)

watchEffect(() => {
  uploadStore.uploadButton = fileInput.value
})

watchEffect(() => {
  searchQuery.value = route.query.search as string
})
</script>
