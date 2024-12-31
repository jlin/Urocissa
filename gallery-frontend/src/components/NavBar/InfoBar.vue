<template>
  <v-toolbar
    :style="{
      backgroundColor: '#212121'
    }"
  >
    <v-btn v-if="!route.meta.isReadPage" @click="showDrawer = !showDrawer" icon="mdi-menu"> </v-btn>
    <v-btn
      v-else
      icon="mdi mdi-arrow-left"
      :to="albumStore.leaveAlbumPath ? albumStore.leaveAlbumPath : '/'"
    ></v-btn>
    <v-card
      v-if="
        route.meta.isReadPage && !route.meta.isViewPage && typeof route.params.hash === 'string'
      "
      elevation="0"
      class="w-50"
    >
      <v-card-title class="text-truncate">
        {{ albumStore.albums.get(route.params.hash) }}
      </v-card-title>
    </v-card>
    <v-card
      elevation="0"
      :style="{
        width: `${route.meta.isReadPage ? '50%' : '100%'}`
      }"
    >
      <v-card-text class="pa-0">
        <v-text-field
          id="nav-search-input"
          rounded
          class="ma-0"
          v-model="searchQuery"
          bg-color="grey-darken-2"
          @click:prepend-inner="handleSearch"
          @click:clear="handleSearch"
          @keyup.enter="handleSearch"
          clearable
          persistent-clear
          variant="solo"
          flat
          prepend-inner-icon="mdi-magnify"
          single-line
          hide-details
          style="margin-right: 10px"
        >
          <template #label>
            <span class="text-caption">Search</span>
          </template>
        </v-text-field>
      </v-card-text>
    </v-card>

    <v-menu v-if="!route.meta.isReadPage">
      <template #activator="{ props }">
        <v-btn v-bind="props" icon="mdi-plus"></v-btn>
      </template>
      <v-list>
        <v-list-item prepend-icon="mdi-upload" value="upload" @click="uploadStore.triggerFileInput">
          <v-list-item-title class="wrap">{{ 'Upload' }}</v-list-item-title>
        </v-list-item>
        <v-list-item prepend-icon="mdi-book-plus" value="create-album" @click="triggerModal()">
          <v-list-item-title class="wrap">{{ 'Create Album' }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-toolbar>
</template>

<script setup lang="ts">
import { inject, ref, watchEffect } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useFilterStore } from '@/store/filterStore'
import { useUploadStore } from '@/store/uploadStore'
import { useModalStore } from '@/store/modalStore'
import { useAlbumStore } from '@/store/albumStore'

const showDrawer = inject('showDrawer')

const albumStore = useAlbumStore('mainId')
const modalStore = useModalStore('mainId')
const uploadStore = useUploadStore('mainId')
const route = useRoute()
const router = useRouter()
const searchQuery = ref('')
const filterStore = useFilterStore('mainId')

const triggerModal = () => {
  modalStore.showCreateAlbumsModal = true
}

const handleSearch = async () => {
  filterStore.filterString = searchQuery.value
  await router.replace({
    path: route.path,
    query: { search: searchQuery.value }
  })
}

watchEffect(() => {
  searchQuery.value = route.query.search as string
})
</script>
