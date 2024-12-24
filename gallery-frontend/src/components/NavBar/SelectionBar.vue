<template>
  <AddingBar :album="album" />
  <ProgessBar isolation-id="subId" />
</template>
<script setup lang="ts">
import { useCollectionStore } from '@/store/collectionStore'

import AddingBar from './AddingBar.vue'
import ProgessBar from './ProgessBar.vue'
import { Album } from '@/script/common/types'
import { watchEffect } from 'vue'

defineProps<{
  album: Album
}>()
const collectionStore = useCollectionStore('tempId')
watchEffect(() => {
  // persist to edit mode
  if (!collectionStore.editModeOn) {
    collectionStore.editModeOn = true
  }
})
</script>
