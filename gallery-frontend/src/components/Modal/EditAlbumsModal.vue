<template>
  <v-dialog
    v-model="modalStore.showEditAlbumsModal"
    variant="flat"
    persistent
    id="edit-album-overlay"
  >
    <v-card class="mx-auto w-100" max-width="400" variant="elevated" retain-focus>
      <v-card-title> Edit Albums </v-card-title>
      <v-container>
        <v-combobox
          v-model="changedAlbumsArray"
          chips
          multiple
          item-title="albumName"
          item-value="albumId"
          :items="albumList!"
          label="Albums"
          closable-chips
        ></v-combobox>
      </v-container>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="teal-accent-4"
          variant="outlined"
          class="ma-2 button button-submit"
          @click="modalStore.showEditAlbumsModal = false"
        >
          Cancel
        </v-btn>
        <v-btn
          color="teal-accent-4"
          variant="outlined"
          class="ma-2 button button-submit"
          @click="change()"
        >
          Submit
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
/**
 * This modal is used for editing the album of a single photo on the single photo view page.
 */
import { useModalStore } from '@/store/modalStore'
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useDataStore } from '@/store/dataStore'
import { useAlbumStore } from '@/store/albumStore'
import { editAlbumsInWorker } from '@/script/inWorker/editAlbumsInWorker'
import { AlbumInfo } from '@/script/common/types'

const modalStore = useModalStore()
const storeData = useDataStore()
const route = useRoute()
const changedAlbumsArray = ref<AlbumInfo[]>([])
const albumStore = useAlbumStore()
const albumList = computed(() => {
  return albumStore.albums
})

onMounted(() => {
  const data = storeData.data.get(storeData.hashMapData.get(route.params.hash as string)!)!
  if (data.database) {
    changedAlbumsArray.value = data.database.album.map((albumId) => ({
      albumId,
      albumName:
        albumStore.albums.find((album) => album.albumId === albumId)?.albumName ?? 'Unknown Album'
    }))
  } else if (data.album) {
    console.error('This should not happen')
  }
})

const defaultAlbums = computed(() => {
  const data = storeData.data.get(storeData.hashMapData.get(route.params.hash as string)!)!
  if (data.database) {
    const result = data.database.album.map((albumId) => ({
      albumId,
      albumName:
        albumStore.albums.find((album) => album.albumId === albumId)?.albumName ?? 'Unknown Album'
    }))
    return result
  } else {
    console.error('This should not happen')
  }
})

const change = () => {
  const hashArray: number[] = [storeData.hashMapData.get(route.params.hash as string)!]
  const addAlbumsArrayComputed = changedAlbumsArray.value.filter(
    (album) => !defaultAlbums.value?.map((album) => album.albumId)!.includes(album.albumId)
  )
  const removeAlbumsArrayComputed = defaultAlbums.value!.filter(
    (album) => !changedAlbumsArray.value?.map((album) => album.albumId).includes(album.albumId)
  )
  console.log(' hashArray is', hashArray)
  console.log('addAlbumsArrayComputed is', addAlbumsArrayComputed)
  console.log(' removeAlbumsArrayComputed) is', removeAlbumsArrayComputed)

  editAlbumsInWorker(
    hashArray,
    addAlbumsArrayComputed.map((album) => album.albumId),
    removeAlbumsArrayComputed.map((album) => album.albumId)
  )
}
</script>

<style scoped></style>
