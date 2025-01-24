<template>
  <v-list-item prepend-icon="mdi-book-plus" value="create-album" @click="createEmptyAlbum()">
    <v-list-item-title class="wrap">{{ 'Create Album' }}</v-list-item-title>
  </v-list-item>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { createAlbum } from '@/script/common/createAlbums'
import { navigateToAlbum } from '@/script/navigator'

const route = useRoute()
const router = useRouter()
const searchQuery = ref('')

watchEffect(() => {
  searchQuery.value = route.query.search as string
})

const waiting = ref(false)

const createEmptyAlbum = async () => {
  waiting.value = true
  const newAlbumId = await createAlbum([])
  if (typeof newAlbumId === 'string') {
    await navigateToAlbum(newAlbumId, router)
  }
  waiting.value = false
}
</script>
