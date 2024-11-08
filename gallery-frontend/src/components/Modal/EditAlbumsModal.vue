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
          item-title="label"
          item-value="value"
          :items="albumList!"
          label="Tags"
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
          :loading="!tagStore.fetched"
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
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { useTagStore } from '@/store/tagStore'
const modalStore = useModalStore()
const storeData = useDataStore()
const route = useRoute()
const changedAlbumsArray = ref<string[]>([])
const tagStore = useTagStore()
const albumList = computed(() => {
  // todo!
})

onMounted(() => {
  const data = storeData.data.get(storeData.hashMapData.get(route.params.hash as string)!)!
  if (data.database) {
    changedAlbumsArray.value = data.database.album
  } else if (data.album) {
    console.error('This should not happen')
  }
})

const defaultAlbums = computed(() => {
  const data = storeData.data.get(storeData.hashMapData.get(route.params.hash as string)!)!
  if (data.database) {
    return data.database.album
  } else {
    console.error('This should not happen')
  }
})

const change = () => {
  const hashArray: number[] = [storeData.hashMapData.get(route.params.hash as string)!]
  const addAlbumsArrayComputed = changedAlbumsArray.value.filter(
    (album) => !defaultAlbums.value!.includes(album)
  )
  const removeAlbumsArrayComputed = defaultAlbums.value!.filter(
    (album) => !changedAlbumsArray.value.includes(album)
  )
  editTagsInWorker(hashArray, addAlbumsArrayComputed, removeAlbumsArrayComputed)
}
</script>

<style scoped></style>
