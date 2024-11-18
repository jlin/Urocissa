<template>
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
  <v-toolbar
    v-if="collectionStore.editModeOn"
    :style="{
      backgroundColor: '#212121'
    }"
  >
    <v-btn
      icon="mdi-close"
      @click="
        () => {
          collectionStore.editModeCollection.clear()
          collectionStore.editModeOn = false
        }
      "
    ></v-btn
    ><v-card elevation="0">
      <v-card-title> {{ `${collectionStore.editModeCollection.size} items` }} </v-card-title>
    </v-card>
  </v-toolbar>
</template>
<script setup lang="ts">
import { useCollectionStore } from '@/store/collectionStore'
import { leaveRead } from '@/script/navigator'
import { useRoute } from 'vue-router'
const props = defineProps<{
  isolationId: string
  title: string | undefined
}>()
const route = useRoute()
const collectionStore = useCollectionStore(props.isolationId)
</script>
