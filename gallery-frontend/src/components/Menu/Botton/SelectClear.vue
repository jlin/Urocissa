<template>
  <v-btn icon="mdi-select-remove" @click="selectClear"></v-btn>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { useRoute } from 'vue-router'
import { getIsolationIdByRoute } from '@/script/common/functions'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)

const collectionStore = useCollectionStore(isolationId)

const selectClear = () => {
  collectionStore.editModeCollection.clear()
}
</script>
