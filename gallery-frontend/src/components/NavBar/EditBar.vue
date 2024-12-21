<!-- NavBarAppBarEditBar.vue -->
<template>
  <v-toolbar
    :style="{
      backgroundColor: '#212121'
    }"
  >
    <LeaveEdit />
    <v-card
      variant="flat"
      class="w-100"
      :title="`${collectionStore.editModeCollection.size} items`"
    >
    </v-card>
    <v-spacer></v-spacer>
    <SelectAll v-if="prefetchStore.dataLength !== collectionStore.editModeCollection.size" />
    <SelectClear v-else />
    <SelectInverse />
    <BatchMenu />
  </v-toolbar>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import BatchMenu from '@/components/Menu/BatchMenu.vue'
import { useRoute } from 'vue-router'
import { getIsolationIdByRoute } from '@/script/common/functions'
import LeaveEdit from '../Menu/Botton/LeaveEdit.vue'
import SelectAll from '../Menu/Botton/SelectAll.vue'
import SelectClear from '../Menu/Botton/SelectClear.vue'
import SelectInverse from '../Menu/Botton/SelectInverse.vue'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)

const collectionStore = useCollectionStore(isolationId)
const prefetchStore = usePrefetchStore(isolationId)
</script>
