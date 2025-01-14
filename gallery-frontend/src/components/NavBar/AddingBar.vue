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
    <v-btn
      :loading="waiting"
      color="teal-accent-4"
      variant="flat"
      class="ma-2 button button-submit"
      @click="submit"
    >
      Complete
    </v-btn>
  </v-toolbar>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import SelectAll from '@/components/Menu/Botton/BtnSelectAll.vue'
import SelectClear from '@/components/Menu/Botton/BtnSelectClear.vue'
import SelectInverse from '@/components/Menu/Botton/BtnSelectInverse.vue'
import { Album } from '@/script/common/types'
import { useModalStore } from '@/store/modalStore'
import { useRerenderStore } from '@/store/rerenderStore'
import { ref } from 'vue'
import axios from 'axios'
import { refreshAlbumMetadata } from '@/worker/utils'
import { useRoute } from 'vue-router'

const collectionStore = useCollectionStore('tempId')
const prefetchStore = usePrefetchStore('tempId')
const modalStore = useModalStore('mainId')
const rerenderStore = useRerenderStore('mainId')
const props = defineProps<{
  album: Album
}>()
const route = useRoute()

const waiting = ref(false)

const submit = async () => {
  waiting.value = true
  const hashArray = Array.from(collectionStore.editModeCollection)
  const timestamp = prefetchStore.timestamp
  if (timestamp !== null) {
    await axios.put('/put/edit_album', {
      idArray: hashArray,
      addAlbumsArray: [props.album.id],
      removeAlbumsArray: [],
      timestamp: timestamp
    })

    console.log('Successfully edited albums.')

    modalStore.showHomeTempModal = false
    waiting.value = false

    const albumId = route.params.hash

    if (typeof albumId !== 'string') {
      return
    }

    refreshAlbumMetadata(albumId)
    rerenderStore.rerenderHomeIsolated()
  }
}
</script>
