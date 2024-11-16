<template>
  <v-dialog
    v-if="submit !== undefined"
    v-model="modalStore.showEditAlbumsModal"
    variant="flat"
    persistent
    id="edit-album-overlay"
  >
    <v-card class="mx-auto w-100" max-width="400" variant="elevated" retain-focus>
      <v-card-title> Edit Albums </v-card-title>
      <v-container>
        <!-- 
  v-model="reactiveArray":
    - Binds to the list (reactiveArray) of selected objects.
    - Choose between `return-object` or `item-value`:
      - `return-object`: reactiveArray will be the list of object.
      - `item-value`: reactiveArray will be the list of object.value
      - Rmk. In this case `item-value` you can further assign item-value="myValue" so that reactiveArray will be the list of object.myValue

  items:
    - The list of objects that can be selected by the user.

  item-title:
    - If set to "field", displays `object.field` to the user (in option filed)

  label:
    - If set to "SomeText", displays "SomeText" to the user (in text field)
-->
        <v-select
          v-model="vModelAlbumsArray"
          chips
          multiple
          item-title="albumName"
          :items="albumStore.albums"
          label="Albums"
          closable-chips
          return-object
        ></v-select>
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
          @click="submit()"
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
import { ref, onMounted, watchEffect } from 'vue'
import { useRoute } from 'vue-router'
import { useAlbumStore } from '@/store/albumStore'
import { editAlbumsInWorker } from '@/script/inWorker/editAlbumsInWorker'
import { AlbumInfo } from '@/script/common/types'
import { getHashIndexDataFromRoute } from '@/script/common/functions'

const submit = ref<(() => void) | undefined>(undefined)
const route = useRoute()

const modalStore = useModalStore('mainId')
const albumStore = useAlbumStore('mainId')

const vModelAlbumsArray = ref<AlbumInfo[]>([])

onMounted(() => {
  const useSubmit = (): undefined | (() => void) => {
    const initializeResult = getHashIndexDataFromRoute(route)
    if (initializeResult === undefined) {
      return undefined
    }

    const { index: index, data: data } = initializeResult
    if (data.database === undefined) {
      return undefined
    }

    const defaultAlbums: AlbumInfo[] = []

    for (const albumId of data.database.album) {
      const albumName = albumStore.albumMap.get(albumId)
      if (albumName === undefined) {
        // Early return if albumName is not found
        return undefined
      }
      defaultAlbums.push({ albumId, albumName })
    }

    // by default vModelAlbumsArray is empty
    // initialize vModelAlbumsArray by setting to defaultAlbums
    vModelAlbumsArray.value = defaultAlbums

    const innerSubmit = () => {
      // hash of the current photo/video
      const idArray: number[] = [index]

      // albums that should be added = albums that are not in default, but in v-model
      const addAlbumsArrayComputed = vModelAlbumsArray.value.filter(
        (album) => !defaultAlbums.map((album) => album.albumId).includes(album.albumId)
      )

      // albums that should be deleted = albums that are in default, but not in v-model
      const removeAlbumsArrayComputed = defaultAlbums.filter(
        (album) => !vModelAlbumsArray.value.map((album) => album.albumId).includes(album.albumId)
      )

      editAlbumsInWorker(
        idArray,
        addAlbumsArrayComputed.map((album) => album.albumId),
        removeAlbumsArrayComputed.map((album) => album.albumId)
      )
      modalStore.showEditAlbumsModal = false
    }
    return innerSubmit
  }
  submit.value = useSubmit()
})

watchEffect(() => {
  console.log('vModelAlbumsArray is', vModelAlbumsArray.value)
})
</script>

<style scoped></style>
