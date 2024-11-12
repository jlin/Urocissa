<!-- NavBarAppBarEditBar.vue -->
<template>
  <v-app-bar>
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
    <NavBarAppBarEditBarMenuNormal v-if="!route.meta.isInsideAlbum" />
    <NavBarAppBarEditBarMenuAlbum v-else />
  </v-app-bar>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import { useRoute } from 'vue-router'
import NavBarAppBarEditBarMenuNormal from '@/components/NavBar/NavBarAppBarEditBarMenuNormal.vue'
import NavBarAppBarEditBarMenuAlbum from './NavBarAppBarEditBarMenuAlbum.vue'

const route = useRoute()
const collectionStore = useCollectionStore('')
const prefetchStore = usePrefetchStore('')

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
