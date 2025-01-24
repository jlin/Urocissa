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
import { getIsolationIdByRoute } from '@/script/common/functions'

const loading = defineModel<boolean>()

const route = useRoute()
const router = useRouter()
const searchQuery = ref('')

const isolationId = getIsolationIdByRoute(route)

watchEffect(() => {
  searchQuery.value = route.query.search as string
})

const createEmptyAlbum = async () => {
  loading.value = true
  const newAlbumId = await createAlbum([], isolationId)
  if (typeof newAlbumId === 'string') {
    await navigateToAlbum(newAlbumId, router)
  }
  loading.value = false
}
</script>
