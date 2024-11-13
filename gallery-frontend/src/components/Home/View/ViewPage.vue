<template>
  <v-overlay
    v-if="index !== undefined"
    :model-value="true"
    :height="'100%'"
    :width="'100%'"
    class="d-flex"
    id="view-page"
    persistent
    transition="false"
  >
    <v-container
      fluid
      class="pa-0 h-100 overflow-hidden position-relative"
      :style="{ backgroundColor: 'black' }"
    >
      <v-row no-gutters class="w-100 h-100 flex-nowrap">
        <ViewPageDisplay :metadata="abstractData" :index="index" />
        <MetadataCol v-if="abstractData" :metadata="abstractData" />
      </v-row>
    </v-container>
  </v-overlay>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useDataStore } from '@/store/dataStore'
import ViewPageDisplay from '@/components/Home/View/ViewPageDisplay/ViewPageDisplay.vue'
import MetadataCol from '@/components/Home/View/ViewPageMetadata.vue'

const props = defineProps<{
  isolationId: string
}>()

const dataStore = useDataStore(props.isolationId)

const route = useRoute()
const hash = computed(() => {
  if (props.isolationId === '') {
    return route.params.hash as string
  } else {
    return route.params.subhash as string
  }
})

const index = computed(() => {
  return dataStore.hashMapData.get(hash.value)
})

const abstractData = computed(() => {
  if (index.value !== undefined) {
    return dataStore.data.get(index.value)
  }
})
</script>
<style scoped>
.v-container::-webkit-scrollbar {
  display: none;
  /* Hide scrollbar */
}
</style>
