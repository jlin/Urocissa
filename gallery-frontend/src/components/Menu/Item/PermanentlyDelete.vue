<template>
  <v-list-item @click="deleteData">
    <v-list-item-title class="wrap">Permanently Delete</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { getIsolationIdByRoute } from '@/script/common/functions'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const collectionStore = useCollectionStore(isolationId)

const deleteData = () => {
  const indexArray = Array.from(collectionStore.editModeCollection)
  deleteDataInWorker(indexArray, isolationId)
}
</script>
