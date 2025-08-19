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
    <NavigationOverlays
      :previous-hash="previousHash"
      :next-hash="nextHash"
      :previous-page="previousPage"
      :next-page="nextPage"
      :show="!configStore.isMobile"
    />
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
import { useConfigStore } from '@/store/configStore'
import ViewPageDisplayDatabase from './DisplayDatabase.vue'
import ViewPageDisplayAlbum from './DisplayAlbum.vue'
import NavigationOverlays from './NavigationOverlays.vue'
import { AbstractData, IsolationId } from '@type/types'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  abstractData: AbstractData | undefined
  colWidth: number | undefined
  colHeight: number | undefined
  previousHash: string | undefined
  nextHash: string | undefined
  previousPage: Record<string, unknown> | undefined
  nextPage: Record<string, unknown> | undefined
}>()

const configStore = useConfigStore(props.isolationId)
</script>
