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
      <v-card-title> {{ props.album.title }}</v-card-title>
    </v-card>
    <v-spacer></v-spacer>
    <v-btn icon="mdi-plus" @click="modalStore.showHomeTempModal = true"> </v-btn>
  </v-toolbar>
  <EditBar v-else />
  <ProgessBar isolation-id="subId" />
  <HomeTemp v-if="modalStore.showHomeTempModal" :album="props.album"> </HomeTemp>
</template>
<script setup lang="ts">
import { useCollectionStore } from '@/store/collectionStore'
import { leaveRead } from '@/script/navigator'
import { useRoute } from 'vue-router'
import EditBar from './EditBar.vue'
import ProgessBar from './ProgessBar.vue'
import HomeTemp from '../Home/HomeTemp.vue'
import { Album } from '@/script/common/types'
import { useModalStore } from '@/store/modalStore'
const modalStore = useModalStore('mainId')

const props = defineProps<{
  album: Album
}>()
const route = useRoute()
const collectionStore = useCollectionStore('subId')
</script>
