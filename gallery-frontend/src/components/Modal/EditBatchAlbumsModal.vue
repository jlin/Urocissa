<template>
  <v-dialog
    v-model="modalStore.showBatchEditAlbumsModal"
    variant="flat"
    persistent
    id="batch-edit-album-overlay"
  >
    <v-card class="mx-auto w-100" max-width="400" variant="elevated">
      <v-card-title class="text-h5"> Edit Albums </v-card-title>
      <v-form v-model="formIsValid" @submit.prevent="submit" validate-on="input">
        <v-container>
          <v-combobox
            v-model="addAlbumsArray"
            chips
            multiple
            label="Add to Albumss"
            item-title="albumName"
            :rules="[addAlbumsRule]"
            :items="[...albumStore.albums.values()]"
            item-value="albumId"
            return-object
            id="test"
          >
            <template #prepend-item>
              <v-list-item value="">
                <template #prepend>
                  <v-list-item-action>
                    <v-btn color="transparent" icon="mdi-plus" density="comfortable" flat></v-btn>
                  </v-list-item-action>
                  <v-list-item-title class="wrap" @click="createNonEmptyAlbum()"
                    >Create New Album</v-list-item-title
                  >
                </template>
              </v-list-item>
            </template>
          </v-combobox>
        </v-container>
        <v-container>
          <v-combobox
            v-model="removeAlbumsArray"
            chips
            multiple
            label="Remove from Albums"
            item-title="albumName"
            :rules="[removeAlbumsRule]"
            :items="[...albumStore.albums.values()]"
            return-objects
          ></v-combobox>
        </v-container>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="teal-accent-4"
            variant="outlined"
            class="ma-2 button button-submit"
            @click="modalStore.showBatchEditAlbumsModal = false"
          >
            Cancel
          </v-btn>
          <v-btn
            color="teal-accent-4"
            variant="outlined"
            class="ma-2 button button-submit"
            :disabled="!formIsValid"
            type="submit"
          >
            Submit
          </v-btn>
        </v-card-actions>
      </v-form>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
/**
 * This modal is used for editing the albums of multiple photos on the home page.
 */
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useModalStore } from '@/store/modalStore'
import { useCollectionStore } from '@/store/collectionStore'
import { useAlbumStore } from '@/store/albumStore'
import { AlbumInfo } from '@/script/common/types'
import { editAlbumsInWorker } from '@/script/inWorker/editAlbumsInWorker'
import { getIsolationIdByRoute } from '@/script/common/functions'
import { createAlbum } from '@/script/common/createAlbums'
import { navigateToAlbum } from '@/script/navigator'
const route = useRoute()
const router = useRouter()
const isolationId = getIsolationIdByRoute(route)

const formIsValid = ref(false)

const collectionStore = useCollectionStore(isolationId)
const albumStore = useAlbumStore(isolationId)
const modalStore = useModalStore('mainId')

const addAlbumsArray = ref<AlbumInfo[]>([])
const removeAlbumsArray = ref<AlbumInfo[]>([])

// Rule for Add Albums to ensure no album is added that's already in Remove Albums
const addAlbumsRule = (inputArray: AlbumInfo[]) => {
  return (
    inputArray.every(
      (album) =>
        !removeAlbumsArray.value.map((removeAlbum) => removeAlbum.albumId).includes(album.albumId)
    ) || 'Some albums are already selected in Remove Albums'
  )
}

// Rule for Remove Albums to ensure no album is added that's already in Add Albums
const removeAlbumsRule = (inputArray: AlbumInfo[]) =>
  inputArray.every(
    (album) => !addAlbumsArray.value.map((addAlbum) => addAlbum.albumId).includes(album.albumId)
  ) || 'Some albums are already selected in Add Albums'

const submit = () => {
  const hashArray = Array.from(collectionStore.editModeCollection)
  editAlbumsInWorker(
    hashArray,
    addAlbumsArray.value.map((album) => album.albumId),
    removeAlbumsArray.value.map((album) => album.albumId),
    isolationId
  )
  modalStore.showBatchEditAlbumsModal = false
}

const createNonEmptyAlbum = async () => {
  const newAlbumId = await createAlbum([...collectionStore.editModeCollection], isolationId)
  if (typeof newAlbumId === 'string') {
    await navigateToAlbum(newAlbumId, router)
    modalStore.showBatchEditAlbumsModal = false
    collectionStore.editModeOn = false
  }
}
</script>
