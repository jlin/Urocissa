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
      <v-card-title> {{ `select elements into ${props.album.title}` }}</v-card-title>
    </v-card>
    <v-spacer></v-spacer>
  </v-toolbar>
  <EditBar v-else />
  <ProgessBar isolation-id="subId" />
</template>
<script setup lang="ts">
import { useCollectionStore } from '@/store/collectionStore'
import { leaveRead } from '@/script/navigator'
import { useRoute } from 'vue-router'

import EditBar from './EditBar.vue'
import ProgessBar from './ProgessBar.vue'
import { Album } from '@/script/common/types'

const props = defineProps<{
  album: Album
}>()
const route = useRoute()
const collectionStore = useCollectionStore('subId')
</script>
