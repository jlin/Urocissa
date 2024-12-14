<!-- NavBarAppBarEditBar.vue -->
<template>
  <v-toolbar
    :style="{
      backgroundColor: '#212121'
    }"
  >
    <v-btn icon="mdi-close" @click="leaveEdit"></v-btn>
    <v-card
      variant="flat"
      class="w-100"
      :title="`${collectionStore.editModeCollection.size} items`"
    >
    </v-card>
    <v-spacer></v-spacer>
    <v-btn
      v-if="prefetchStore.dataLength !== collectionStore.editModeCollection.size"
      icon="mdi-select-all"
      @click="selectAll"
    ></v-btn>
    <v-btn v-else icon="mdi-select-remove" @click="selectRemove"></v-btn>
    <v-btn icon="mdi-select-inverse" @click="selectInverse"></v-btn>
    <!-- Use the new Menu Component -->
    <BatchMenu />
  </v-toolbar>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import BatchMenu from '@/components/Menu/BatchMenu.vue'
import { useRoute } from 'vue-router'
import { getIsolationIdByRoute } from '@/script/common/functions'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)

const collectionStore = useCollectionStore(isolationId)
const prefetchStore = usePrefetchStore(isolationId)

// Methods
const leaveEdit = () => {
  collectionStore.editModeCollection.clear()
  collectionStore.editModeOn = false
}

const selectAll = () => {
  for (let i = 0; i < prefetchStore.dataLength; i++) {
    collectionStore.editModeCollection.add(i)
  }
}

const selectRemove = () => {
  collectionStore.editModeCollection.clear()
}

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

<style scoped>
/* Add any component-specific styles here */
</style>
