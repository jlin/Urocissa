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
      <Restore :index-list="editModeList" v-if="isInTrashedPath" />
      <PermanentlyDelete :index-list="editModeList" v-if="isInTrashedPath" />

      <v-divider></v-divider>

      <!-- Regenerate Action -->
      <RegenerateMetadata :index-list="editModeList" />
    </v-list>
  </v-menu>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'

// Importing menu item components
import SetAsCover from './Item/ItemSetAsCover.vue'
import Archive from './Item/ItemArchive.vue'
import Favorite from './Item/ItemFavorite.vue'
import BatchEditTags from './Item/ItemBatchEditTags.vue'
import BatchEditAlbums from './Item/ItemBatchEditAlbums.vue'
import Download from './Item/ItemDownload.vue'
import Delete from './Item/ItemDelete.vue'
import PermanentlyDelete from './Item/ItemPermanentlyDelete.vue'

// Utility function to extract isolation ID from the route
import { getIsolationIdByRoute } from '@/script/common/functions'
import RegenerateMetadata from './Item/ItemRegenerateMetadata.vue'
import Restore from './Item/ItemRestore.vue'

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
