<!-- NavBarAppBarEditBarMenuNormal.vue -->
<template>
  <v-menu>
    <template #activator="{ props }">
      <v-btn v-bind="props" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <SetAsCover v-if="route.meta.isReadPage && collectionStore.editModeCollection.size === 1" />

      <v-divider
        v-if="route.meta.isReadPage && collectionStore.editModeCollection.size === 1"
      ></v-divider>

      <Archive :index-list="Array.from(collectionStore.editModeCollection)" />
      <Favorite :index-list="Array.from(collectionStore.editModeCollection)" />
      <BatchEditTags />
      <BatchEditAlbums />

      <v-divider></v-divider>

      <Download />

      <v-divider></v-divider>

      <Delete
        :index-list="Array.from(collectionStore.editModeCollection)"
        v-if="!route.path.startsWith('/trashed')"
      />
      <PermanentlyDelete
        :index-list="Array.from(collectionStore.editModeCollection)"
        v-else
        prepend-icon="mdi-trash-can-outline"
      />

      <v-divider></v-divider>

      <RegeneratePreview />
    </v-list>
  </v-menu>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { getIsolationIdByRoute } from '@/script/common/functions'
import SetAsCover from './Item/SetAsCover.vue'
import Archive from './Item/Archive.vue'
import Favorite from './Item/Favorite.vue'
import BatchEditTags from './Item/BatchEditTags.vue'
import BatchEditAlbums from './Item/BatchEditAlbums.vue'
import Download from './Item/Download.vue'
import Delete from './Item/Delete.vue'
import RegeneratePreview from './Item/RegeneratePreview.vue'
import PermanentlyDelete from './Item/PermanentlyDelete.vue'

const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const collectionStore = useCollectionStore(isolationId)
</script>
