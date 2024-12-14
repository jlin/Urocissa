<template>
  <v-list-item
    prepend-icon="mdi-star-arrow-down"
    @click="quickAddTags('_favorite', Array.from(collectionStore.editModeCollection), isolationId)"
  >
    <v-list-item-title class="wrap">Favorite</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { getIsolationIdByRoute } from '@/script/common/functions'
import { quickAddTags } from '@/script/common/quickEditTags'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const collectionStore = useCollectionStore(isolationId)
</script>
