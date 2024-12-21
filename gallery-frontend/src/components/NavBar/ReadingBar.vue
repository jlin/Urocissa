<template>
  <v-toolbar
    flat
    height="2"
    class="no-select"
    :style="{
      backgroundColor: '#212121'
    }"
  >
  </v-toolbar>
  <v-toolbar
    v-if="!collectionStore.editModeOn"
    class="position-relative"
    :style="{
      backgroundColor: '#212121'
    }"
  >
    <v-btn icon="mdi mdi-arrow-left" :to="leaveRead(route)"></v-btn
    ><v-card elevation="0">
      <v-card-title> {{ props.title }}</v-card-title>
    </v-card>
  </v-toolbar>
  <EditBar v-else />
  <ProgessBar />
</template>
<script setup lang="ts">
import { useCollectionStore } from '@/store/collectionStore'
import { leaveRead } from '@/script/navigator'
import { useRoute } from 'vue-router'
import EditBar from './EditBar.vue'
import ProgessBar from './ProgessBar.vue'
import { IsolationId } from '@/script/common/types'

const props = defineProps<{
  isolationId: IsolationId
  title: string | null
}>()
const route = useRoute()
const collectionStore = useCollectionStore(props.isolationId)
</script>
