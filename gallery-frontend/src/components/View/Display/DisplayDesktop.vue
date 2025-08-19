<template>
  <div class="h-100 w-100">
    <v-card
      width="100"
      v-if="previousHash !== undefined"
      color="transparent"
      class="navigate-left d-flex align-center justify-center h-50"
      style="position: absolute; left: 0; top: 50%; transform: translateY(-50%); z-index: 1"
      :to="previousPage"
      replace
    >
      <v-icon>mdi-arrow-left</v-icon>
    </v-card>
    <v-card
      width="100"
      v-if="nextHash !== undefined"
      color="transparent"
      class="navigate-right d-flex align-center justify-center h-50"
      style="position: absolute; right: 0; top: 50%; transform: translateY(-50%); z-index: 1"
      :to="nextPage"
      replace
    >
      <v-icon>mdi-arrow-right</v-icon>
    </v-card>

    <!-- Navigation overlays (not grid children) -->
    <v-card
      width="100"
      v-if="!configStore.isMobile && previousHash !== undefined"
      color="transparent"
      class="navigate-left d-flex align-center justify-center h-50"
      style="position: absolute; left: 0; top: 50%; transform: translateY(-50%); z-index: 1"
      :to="previousPage"
      replace
    >
      <v-icon>mdi-arrow-left</v-icon>
    </v-card>
    <v-card
      width="100"
      v-if="!configStore.isMobile && nextHash !== undefined"
      color="transparent"
      class="navigate-right d-flex align-center justify-center h-50"
      style="position: absolute; right: 0; top: 50%; transform: translateY(-50%); z-index: 1"
      :to="nextPage"
      replace
    >
      <v-icon>mdi-arrow-right</v-icon>
    </v-card>

    <div class="h-100 w-100">
      <ViewPageDisplayDatabase
        v-if="abstractData && !configStore.disableImg"
        :index="index"
        :hash="hash"
        :abstract-data="abstractData"
        :col-width="colWidth ?? 0"
        :col-height="colHeight ?? 0"
        :isolation-id="isolationId"
      />
      <ViewPageDisplayAlbum
        v-if="abstractData && abstractData.album && !configStore.disableImg"
        :index="index"
        :album="abstractData.album"
        :col-width="colWidth ?? 0"
        :col-height="colHeight ?? 0"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useDataStore } from '@/store/dataStore'
import { useConfigStore } from '@/store/configStore'
import ViewPageDisplayDatabase from './DisplayDatabase.vue'
import ViewPageDisplayAlbum from './DisplayAlbum.vue'
import { AbstractData, IsolationId } from '@type/types'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  abstractData: AbstractData | undefined
  colWidth: number | undefined
  colHeight: number | undefined
}>()

const configStore = useConfigStore(props.isolationId)
const dataStore = useDataStore(props.isolationId)
const route = useRoute()

const nextHash = computed(() => {
  const nextData = dataStore.data.get(props.index + 1)
  if (nextData?.database) return nextData.database.hash
  if (nextData?.album) return nextData.album.id
  return undefined
})

const previousHash = computed(() => {
  const previousData = dataStore.data.get(props.index - 1)
  if (previousData?.database) return previousData.database.hash
  if (previousData?.album) return previousData.album.id
  return undefined
})

const nextPage = computed(() => {
  if (nextHash.value === undefined) return undefined
  if (route.meta.level === 2) {
    const updatedParams = { ...route.params, hash: nextHash.value }
    return { ...route, params: updatedParams }
  } else if (route.meta.level === 4) {
    const updatedParams = { ...route.params, subhash: nextHash.value }
    return { ...route, params: updatedParams }
  }
  return undefined
})

const previousPage = computed(() => {
  if (previousHash.value === undefined) return undefined
  if (route.meta.level === 2) {
    const updatedParams = { ...route.params, hash: previousHash.value }
    return { ...route, params: updatedParams }
  } else if (route.meta.level === 4) {
    const updatedParams = { ...route.params, subhash: previousHash.value }
    return { ...route, params: updatedParams }
  }
  return undefined
})
</script>

<style scoped>
/* No extra styles; positioning is inline to match original layout */
</style>
