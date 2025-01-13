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
import { editAlbums } from '@/worker/toDataWorker'
import { ref } from 'vue'

const collectionStore = useCollectionStore('tempId')
const prefetchStore = usePrefetchStore('tempId')
const modalStore = useModalStore('mainId')
const rerenderStore = useRerenderStore('mainId')
const props = defineProps<{
  album: Album
}>()

const waiting = ref(false)

const submit = async () => {
  waiting.value = true
  const hashArray = Array.from(collectionStore.editModeCollection)
  const timestamp = prefetchStore.timestamp
  if (timestamp !== null) {
    await editAlbums(hashArray, [props.album.id], [], timestamp)
    modalStore.showHomeTempModal = false
    waiting.value = false
    rerenderStore.rerenderHome()
  }
}
</script>
