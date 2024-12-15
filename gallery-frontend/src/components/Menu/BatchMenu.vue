<!-- NavBarAppBarEditBarMenuNormal.vue -->
<template>
  <v-menu>
    <template #activator="{ props: MenuBtn }">
      <v-btn v-bind="MenuBtn" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <!-- Conditional Set as Cover -->
      <SetAsCover v-if="shouldShowSetAsCover" />

      <v-divider v-if="shouldShowSetAsCover"></v-divider>

      <!-- Archive and Favorite Actions -->
      <Archive :index-list="editModeList" />
      <Favorite :index-list="editModeList" />
      <BatchEditTags />
      <BatchEditAlbums />

      <v-divider></v-divider>

      <!-- Download Action -->
      <Download :index-list="editModeList" />

      <v-divider></v-divider>

      <!-- Delete or Permanently Delete Actions -->
      <Delete :index-list="editModeList" v-if="!isInTrashedPath" />
      <PermanentlyDelete :index-list="editModeList" v-else prepend-icon="mdi-trash-can-outline" />

      <v-divider></v-divider>

      <!-- Regenerate Preview Action -->
      <RegeneratePreview :index-list="editModeList" />
    </v-list>
  </v-menu>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'

// Importing menu item components
import SetAsCover from './Item/SetAsCover.vue'
import Archive from './Item/Archive.vue'
import Favorite from './Item/Favorite.vue'
import BatchEditTags from './Item/BatchEditTags.vue'
import BatchEditAlbums from './Item/BatchEditAlbums.vue'
import Download from './Item/Download.vue'
import Delete from './Item/Delete.vue'
import RegeneratePreview from './Item/RegeneratePreview.vue'
import PermanentlyDelete from './Item/PermanentlyDelete.vue'

// Utility function to extract isolation ID from the route
import { getIsolationIdByRoute } from '@/script/common/functions'

// Initialize route and store
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const collectionStore = useCollectionStore(isolationId)

// Computed property for the edit mode collection as an array
const editModeList = computed(() => Array.from(collectionStore.editModeCollection))

// Computed property to determine if SetAsCover should be shown
const shouldShowSetAsCover = computed(
  () => route.meta.isReadPage && collectionStore.editModeCollection.size === 1
)

// Computed property to check if the current path is within '/trashed'
const isInTrashedPath = computed(() => route.path.startsWith('/trashed'))
</script>

<style scoped>
/* Add any component-specific styles here */
</style>
