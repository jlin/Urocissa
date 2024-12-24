<template>
  <v-toolbar
    :style="{
      backgroundColor: '#212121'
    }"
  >
    <v-card
      variant="flat"
      class="w-100"
      :title="`${collectionStore.editModeCollection.size} items added to ${album.title}`"
    >
    </v-card>
    <v-spacer></v-spacer>
    <SelectInverse isolation-id="tempId" />
    <SelectAll
      isolation-id="tempId"
      v-if="prefetchStore.dataLength !== collectionStore.editModeCollection.size"
    />
    <SelectClear v-else isolation-id="tempId" />
    <BatchMenu />
  </v-toolbar>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import BatchMenu from '@/components/Menu/BatchMenu.vue'
import SelectAll from '../Menu/Botton/BtnSelectAll.vue'
import SelectClear from '../Menu/Botton/BtnSelectClear.vue'
import SelectInverse from '../Menu/Botton/BtnSelectInverse.vue'
import { Album } from '@/script/common/types'

defineProps<{
  album: Album
}>()

const collectionStore = useCollectionStore('tempId')
const prefetchStore = usePrefetchStore('tempId')
</script>
