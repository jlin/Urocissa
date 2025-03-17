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
  <EditBar v-else />
</template>

<script setup lang="ts">
import { ref, type Ref, watchEffect } from 'vue'

import { useCollectionStore } from '@/store/collectionStore'
import EditBar from '@/components/NavBar/EditBar.vue'
import InfoBar from '@/components/NavBar/InfoBar.vue'

import { useUploadStore } from '@/store/uploadStore'

const uploadStore = useUploadStore('mainId')
const collectionStore = useCollectionStore('mainId')

const fileInput: Ref<HTMLInputElement | null> = ref(null)

watchEffect(() => {
  uploadStore.uploadButton = fileInput.value
})
</script>
