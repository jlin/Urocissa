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
    <SelectInverse :isolation-id="isolationId" />
    <SelectAll
      v-if="
        prefetchStore.dataLength === 0 ||
        prefetchStore.dataLength !== collectionStore.editModeCollection.size
      "
      :isolation-id="isolationId"
    />
    <SelectClear v-else :isolation-id="isolationId" />
    <BatchMenu />
  </v-toolbar>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import BatchMenu from '@/components/Menu/BatchMenu.vue'
import { useRoute } from 'vue-router'
import { getIsolationIdByRoute } from '@/script/common/functions'
import LeaveEdit from '../Menu/Botton/BtnLeaveEdit.vue'
import SelectAll from '../Menu/Botton/BtnSelectAll.vue'
import SelectClear from '../Menu/Botton/BtnSelectClear.vue'
import SelectInverse from '../Menu/Botton/BtnSelectInverse.vue'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)

const collectionStore = useCollectionStore(isolationId)
const prefetchStore = usePrefetchStore(isolationId)
</script>
