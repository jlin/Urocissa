<template>
  <v-overlay
    :model-value="true"
    :height="'100%'"
    :width="'100%'"
    class="d-flex"
    id="view-page"
    persistent
  >
    <v-container
      fluid
      class="pa-0 h-100 overflow-hidden position-relative"
      :style="{ backgroundColor: 'black' }"
    >
      <v-row no-gutters class="w-100 h-100 flex-nowrap">
        <ViewPageDisplay v-if="metadata" :metadata="metadata" />
        <MetadataCol v-if="metadata" :metadata="metadata" />
      </v-row>
    </v-container>
  </v-overlay>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useDataStore } from '@/store/dataStore'
import ViewPageDisplay from './ViewPageDisplay.vue'
import MetadataCol from '@/components/Home/View/ViewPageMetadata.vue'

const index = computed(() => {
  return dataStore.hashMapData.get(route.params.hash as string)!
})

const dataStore = useDataStore()
const route = useRoute()
const metadata = computed(() => {
  return dataStore.data.get(index.value)?.database!
})
</script>
