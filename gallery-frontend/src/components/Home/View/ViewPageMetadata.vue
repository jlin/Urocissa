<template>
  <v-col
    id="metadata-col"
    v-if="metadata"
    class="h-100 metadata-css"
    cols="auto"
    :style="{ backgroundColor: 'white' }"
  >
    <v-row v-if="metadata.database" no-gutters class="position-relative">
      <v-toolbar
        color="white"
        :style="{
          backgroundColor: '#212121'
        }"
      >
        <!-- Icon button with increased size -->
        <v-btn icon @click="toggleInfo">
          <v-icon>mdi-close</v-icon>
        </v-btn>
        <!-- Wrapped Info text with increased font size -->
        <v-toolbar-title class="text-h5">Info</v-toolbar-title>
      </v-toolbar>
      <v-col class="h-100 w-100" cols="auto">
        <v-list bg-color="white" class="pa-0" height="100%" lines="two">
          <!-- Metadata Items -->
          <ItemSize :database="metadata.database" />
          <ItemPath :database="metadata.database" />
          <ItemDate :database="metadata.database" />
          <ItemExif
            v-if="
              metadata.database.exif_vec.Make !== undefined ||
              metadata.database.exif_vec.Model !== undefined
            "
            :database="metadata.database"
          />
          <!-- Tags Section -->
          <v-divider></v-divider>
          <ItemTag
            :isolation-id="props.isolationId"
            :index="props.index"
            :tags="metadata.database.tag"
          />

          <!-- Albums Section -->
          <ItemAlbum
            :isolation-id="props.isolationId"
            :index="props.index"
            :albums="metadata.database.album"
          />
        </v-list>
      </v-col>
    </v-row>
    <v-row v-if="metadata.album" no-gutters class="position-relative">
      <v-toolbar
        color="white"
        :style="{
          backgroundColor: '#212121'
        }"
      >
        <!-- Icon button with increased size -->
        <v-btn icon @click="toggleInfo">
          <v-icon>mdi-close</v-icon>
        </v-btn>
        <!-- Wrapped Info text with increased font size -->
        <v-toolbar-title class="text-h5">Info</v-toolbar-title>
      </v-toolbar>
      <v-col class="h-100 w-100" cols="auto">
        <v-list bg-color="white" class="pa-0" height="100%" lines="two">
          <!-- Metadata Items -->
          <ItemTitle :title="metadata.album.title" />
          <ItemCount :album="metadata.album" />
          <!-- Tags Section -->
          <v-divider></v-divider>
          <ItemTag
            :isolation-id="props.isolationId"
            :index="props.index"
            :tags="metadata.album.tag"
          />
        </v-list>
      </v-col>
    </v-row>
  </v-col>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { useInfoStore } from '@/store/infoStore'

import { AbstractData, IsolationId } from '@/script/common/types'

import ItemExif from './Item/ItemExif.vue'
import ItemSize from './Item/ItemSize.vue'
import ItemPath from './Item/ItemPath.vue'
import ItemDate from './Item/ItemDate.vue'
import ItemTag from './Item/ItemTag.vue'
import ItemAlbum from './Item/ItemAlbum.vue'
import ItemTitle from './Item/ItemTitle.vue'
import ItemCount from './Item/ItemCount.vue'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  metadata: AbstractData
}>()

const infoStore = useInfoStore('mainId')

function toggleInfo() {
  infoStore.showInfo = !infoStore.showInfo
}

watch(
  () => props.hash,
  () => {
    console.log(props.metadata)
  }
)
</script>
<style scoped>
.metadata-css {
  width: 360px;
}

@media (max-width: 720px) {
  .metadata-css {
    width: 100%;
  }
}
</style>
