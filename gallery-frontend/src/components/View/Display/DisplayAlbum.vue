<template>
  <!-- Root is a single v-col so parent v-row receives a valid column child -->
  <div class="h-100 d-flex align-center justify-center">
    <router-view v-slot="{ Component }">
      <component :is="Component" />
    </router-view>
    <div
      :class="[
        'd-flex',
        'align-center',
        'justify-center',
        colWidth < colHeight ? 'flex-column' : 'flex-row'
      ]"
    >
      <v-card class="album-card">
        <img
          v-if="imgStore.imgOriginal.get(index)"
          id="album-img"
          :key="index"
          rounded="xl"
          aspect-ratio="1"
          cover
          :src="imgStore.imgOriginal.get(index)"
          class="album-card__img"
        />
      </v-card>
      <v-card class="album-card album-card__side d-flex flex-column" outlined style="padding: 16px">
        <v-card-item>
          <v-text-field
            v-model="titleModel"
            variant="underlined"
            @blur="editTitle(props.album, titleModel)"
            :placeholder="titleModel === '' ? 'Add Title' : undefined"
          ></v-text-field>
        </v-card-item>
        <v-list>
          <v-list-item>
            <v-list-item-title v-if="album.startTime">
              {{ `${dater(album.startTime)} ~ ${dater(album.endTime!)}` }}
            </v-list-item-title>
            <v-list-item-subtitle>
              {{ `${album.itemCount} item${album.itemCount === 1 ? '' : 's'}` }}
              •
              {{ filesize(album.itemSize) }}
            </v-list-item-subtitle>
          </v-list-item>
        </v-list>

        <!-- Use this div to take up remaining space -->
        <div class="flex-grow-1"></div>

        <v-card-actions class="justify-end" v-if="route.meta.level === 2">
          <v-btn
            color="teal-accent-4"
            variant="flat"
            class="ma-2 button button-submit"
            :to="route.meta.getChildPage(route, undefined)"
            @click="
              () => {
                albumStore.leaveAlbumPath = route.fullPath
              }
            "
          >
            Enter Album
          </v-btn>
        </v-card-actions>
      </v-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useImgStore } from '@/store/imgStore'
import { useAlbumStore } from '@/store/albumStore'
import { filesize } from 'filesize'
import { useRoute } from 'vue-router'
import { dater } from '@utils/dater'
import { Album } from '@type/types'
import { ref, watch } from 'vue'
import { editTitle } from '@utils/createAlbums'

const titleModel = ref('')

const route = useRoute()

const albumStore = useAlbumStore('mainId')
const imgStore = useImgStore('mainId')

const props = defineProps<{
  index: number
  album: Album
  colWidth: number
  colHeight: number
}>()

watch(
  () => props.album.title,
  () => {
    titleModel.value = props.album.title ?? ''
  },
  { immediate: true }
)
</script>
<style scoped>
.v-text-field :deep(input) {
  font-size: 2.125rem;
  font-weight: 400;
  line-height: 1.175;
  letter-spacing: 0.0073529412em;
}

/* Album card responsive sizing using media queries instead of inline JS styles */
.album-card {
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  border: 8px solid white;
  max-width: 500px;
  max-height: 500px;
  width: min(80vw, 500px);
  height: min(80vw, 500px);
}

.album-card__img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.album-card__side {
  border: none; /* separate side card doesn't need the photo border */
  max-width: 500px;
  max-height: 500px;
  width: min(80vw, 500px);
  height: min(80vw, 500px);
}

/* On wider viewports use roughly half the column width (similar to previous colWidth/2 behavior) */
@media (min-width: 600px) {
  .album-card,
  .album-card__side {
    width: min(40vw, 500px);
    height: min(40vw, 500px);
  }
}

@media (min-width: 1200px) {
  .album-card,
  .album-card__side {
    width: min(30vw, 500px);
    height: min(30vw, 500px);
  }
}
</style>
