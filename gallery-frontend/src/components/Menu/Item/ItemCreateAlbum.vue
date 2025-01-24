<template>
  <v-list-item value="create-album" @click="createEmptyAlbum()">
    <template #prepend>
      <v-icon v-if="!waiting" icon="mdi-book-plus"></v-icon>
      <v-icon v-else><v-progress-circular size="24" indeterminate></v-progress-circular></v-icon>
    </template>
    <v-list-item-title class="wrap">{{ 'Create Album' }}</v-list-item-title>
  </v-list-item>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { createAlbum } from '@/script/common/createAlbums'
import { navigateToAlbum } from '@/script/navigator'
import { getIsolationIdByRoute } from '@/script/common/functions'

const route = useRoute()
const router = useRouter()
const searchQuery = ref('')
const waiting = ref(false)

const isolationId = getIsolationIdByRoute(route)

watchEffect(() => {
  searchQuery.value = route.query.search as string
})

const createEmptyAlbum = async () => {
  waiting.value = true
  const newAlbumId = await createAlbum([], isolationId)
  if (typeof newAlbumId === 'string') {
    await navigateToAlbum(newAlbumId, router)
  }
  waiting.value = false
}
</script>
