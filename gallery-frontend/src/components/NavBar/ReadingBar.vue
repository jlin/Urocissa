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
    <v-btn icon="mdi mdi-arrow-left" :to="leaveRead(route)"></v-btn>
    <v-card elevation="0" class="w-100">
      <v-card-title>
        <v-text-field
          v-model="titleModel"
          variant="plain"
          @blur="editTitle"
          :placeholder="titleModel === '' ? 'Untitled' : undefined"
        ></v-text-field
      ></v-card-title>
    </v-card>
    <v-spacer></v-spacer>
    <v-btn icon="mdi-image-plus" @click="modalStore.showHomeTempModal = true"> </v-btn>
  </v-toolbar>
  <EditBar v-else />
  <ProgessBar isolation-id="subId" />
  <HomeTemp v-if="modalStore.showHomeTempModal" :album="props.album"> </HomeTemp>
</template>
<script setup lang="ts">
import { useCollectionStore } from '@/store/collectionStore'
import { leaveRead } from '@/script/navigator'
import { useRoute } from 'vue-router'
import EditBar from '@/components/NavBar/EditBar.vue'
import ProgessBar from '@/components/NavBar/ProgessBar.vue'
import HomeTemp from '@/components/Home/Page/HomeTemp.vue'
import { Album } from '@/script/common/types'
import { useModalStore } from '@/store/modalStore'
import { ref, watch } from 'vue'
import { useAlbumStore } from '@/store/albumStore'
import axios from 'axios'
import { useDataStore } from '@/store/dataStore'

const props = defineProps<{
  album: Album
}>()

const route = useRoute()

const modalStore = useModalStore('mainId')
const collectionStore = useCollectionStore('subId')

const titleModel = ref('')

watch(
  () => props.album.title,
  () => {
    titleModel.value = props.album.title ?? ''
  },
  { immediate: true }
)

const dataStore = useDataStore('mainId')
const albumStore = useAlbumStore('mainId')

async function editTitle() {
  if ((props.album.title ?? '') !== titleModel.value) {
    const id = props.album.id
    const title = titleModel.value === '' ? null : titleModel.value
    await axios.post('/post/set_album_title', {
      albumId: id,
      title: title
    })
    const albumInfo = albumStore.albums.get(id)
    const index = dataStore.hashMapData.get(props.album.id)
    if (index !== undefined) {
      const album = dataStore.data.get(index)?.album
      if (albumInfo && album) {
        albumInfo.albumName = title
        album.title = title
      } else {
        console.error(`Cannot find album with id ${id}`)
      }
    }
  }
}
</script>

<style scoped>
.v-text-field :deep(input) {
  font-size: 22px;
  font-weight: 400;
  line-height: 1.175;
  letter-spacing: 0.0073529412em;
  margin-bottom: -8px;
}
</style>
