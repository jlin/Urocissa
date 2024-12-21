<template>
  <v-btn icon="mdi-checkbox-intermediate-variant" @click="selectInverse"></v-btn>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import { useRoute } from 'vue-router'
import { getIsolationIdByRoute } from '@/script/common/functions'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)

const collectionStore = useCollectionStore(isolationId)
const prefetchStore = usePrefetchStore(isolationId)

const selectInverse = () => {
  for (let i = 0; i < prefetchStore.dataLength; i++) {
    if (collectionStore.editModeCollection.has(i)) {
      collectionStore.editModeCollection.delete(i)
    } else {
      collectionStore.editModeCollection.add(i)
    }
  }
}
</script>
