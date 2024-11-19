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
  <NavBarAppBarEditBar v-else />
  <NavBarProgessBar />
</template>
<script setup lang="ts">
import { useCollectionStore } from '@/store/collectionStore'
import { leaveRead } from '@/script/navigator'
import { useRoute } from 'vue-router'
import NavBarAppBarEditBar from '../NavBar/NavBarAppBarEditBar.vue'
import NavBarProgessBar from '../NavBar/NavBarProgessBar.vue'

const props = defineProps<{
  isolationId: string
  title: string | undefined
}>()
const route = useRoute()
const collectionStore = useCollectionStore(props.isolationId)
</script>
