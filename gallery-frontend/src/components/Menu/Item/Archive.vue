<template>
  <v-list-item
    prepend-icon="mdi-archive-arrow-down"
    @click="quickAddTags('_archive', Array.from(collectionStore.editModeCollection), isolationId)"
  >
    <v-list-item-title class="wrap">Archive</v-list-item-title>
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
