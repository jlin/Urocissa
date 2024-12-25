<template>
  <v-toolbar
    :style="{
      backgroundColor: '#212121'
    }"
    ><v-btn icon="mdi-close" @click="modalStore.showHomeTempModal = false"></v-btn>
    <v-card
      variant="flat"
      class="w-100"
      :title="`Add ${collectionStore.editModeCollection.size} items to ${album.title}`"
    >
    </v-card>
    <v-spacer></v-spacer>
    <SelectInverse isolation-id="tempId" />
    <SelectAll
      isolation-id="tempId"
      v-if="
        prefetchStore.dataLength === 0 ||
        prefetchStore.dataLength !== collectionStore.editModeCollection.size
      "
    />
    <SelectClear v-else isolation-id="tempId" />
    <v-btn color="teal-accent-4" variant="flat" class="ma-2 button button-submit" @click="submit">
      Complete
    </v-btn>
  </v-toolbar>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import SelectAll from '../Menu/Botton/BtnSelectAll.vue'
import SelectClear from '../Menu/Botton/BtnSelectClear.vue'
import SelectInverse from '../Menu/Botton/BtnSelectInverse.vue'
import { Album } from '@/script/common/types'
import { editAlbumsInWorker } from '@/script/inWorker/editAlbumsInWorker'
import { useModalStore } from '@/store/modalStore'
import { watchEffect } from 'vue'

const collectionStore = useCollectionStore('tempId')
const prefetchStore = usePrefetchStore('tempId')
const modalStore = useModalStore('mainId')
const props = defineProps<{
  album: Album
}>()

const submit = () => {
  const hashArray = Array.from(collectionStore.editModeCollection)
  editAlbumsInWorker(hashArray, [props.album.id], [], 'tempId')
  modalStore.showHomeTempModal = false
}

watchEffect(() => {
  console.log('prefetchStore.dataLength is', prefetchStore.dataLength)
  console.log('collectionStore.editModeCollection.size is', collectionStore.editModeCollection.size)

  console.log(
    'prefetchStore.dataLength !== collectionStore.editModeCollection.size is',
    prefetchStore.dataLength !== collectionStore.editModeCollection.size
  )
})
</script>
