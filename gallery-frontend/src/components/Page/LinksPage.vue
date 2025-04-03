<template>
  <NavBar />
  <v-container
    v-if="albumStore.fetched"
    id="table-container"
    class="pa-1 bg-grey-darken-3 d-flex align-start"
    :style="{
      height: `calc(100% - ${navBarHeight}px)`
    }"
    fluid
  >
    <v-row justify="center" class="ma-0">
      <v-col cols="12" sm="12" md="10" lg="8" class="d-flex justify-center">
        <v-card tile flat class="overflow-y-auto">
          <v-table hover>
            <thead ref="tableRef">
              <tr>
                <th>album</th>
                <th>link</th>
              </tr>
            </thead>
            <tbody>
              <template v-for="album in albumStore.albums.values()" :key="album.albumId">
                <tr v-for="[key, share] in album.shareList" :key="album.albumId + '-' + key">
                  <td>{{ album.albumId }}</td>
                  <td>{{ share.url }}</td>
                </tr>
              </template>
            </tbody>
          </v-table>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useInitializedStore } from '@/store/initializedStore'
import { onMounted } from 'vue'
import { onBeforeUnmount } from 'vue'
import { navBarHeight } from '@/type/constants'
import NavBar from '@/components/NavBar/NavBar.vue'
import { useAlbumStore } from '@/store/albumStore'
const initializedStore = useInitializedStore('mainId')
const albumStore = useAlbumStore('mainId')

const dynamicWidth = ref<number>(0)
const tableRef = ref<HTMLElement | null>(null)
const updateDynamicWidth = () => {
  const tableWidth = tableRef.value?.offsetWidth ?? 0
  dynamicWidth.value = tableWidth <= 300 ? 300 : tableWidth
}

watch(
  () => initializedStore.initialized,
  () => {
    if (initializedStore.initialized) {
      updateDynamicWidth()
    }
  }
)

onMounted(async () => {
  if (!albumStore.fetched) {
    await albumStore.fetchAlbums()
  }
  initializedStore.initialized = true
})

onBeforeUnmount(() => {
  initializedStore.initialized = false
})
</script>

<style scoped>
#table-container {
  display: flex;
  justify-content: center;
  position: relative;
  padding: 4px;
  padding-top: 4px;
  background-color: #3d3d3d;
  overflow-y: scroll;
  height: 100dvh;
  width: 100%;
}

#metadata {
  height: 100%;
  width: 300px;
}
</style>
