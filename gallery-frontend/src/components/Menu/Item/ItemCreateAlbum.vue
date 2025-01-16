<template>
  <v-list-item prepend-icon="mdi-book-plus" value="create-album" @click="createAlbum()">
    <v-list-item-title class="wrap">{{ 'Create Album' }}</v-list-item-title>
  </v-list-item>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useModalStore } from '@/store/modalStore'
import axios from 'axios'
import { useMessageStore } from '@/store/messageStore'
import { navigateToAlbum } from '@/script/navigator'

const modalStore = useModalStore('mainId')
const messageStore = useMessageStore('mainId')

const route = useRoute()
const router = useRouter()
const searchQuery = ref('')

watchEffect(() => {
  searchQuery.value = route.query.search as string
})

const waiting = ref(false)

const createAlbum = async () => {
  try {
    waiting.value = true
    const createAlbumData = {
      title: null,
      elements: []
    }

    const response = await axios.post<string>('/post/create_album', createAlbumData, {
      headers: {
        'Content-Type': 'application/json'
      }
    })

    messageStore.message = 'Album created successfully.'
    messageStore.warn = false
    messageStore.showMessage = true

    modalStore.showCreateAlbumsModal = false
    const newAlbumId = response.data
    waiting.value = false
    await navigateToAlbum(newAlbumId, router)
  } catch (error) {
    console.error('Error creating album:', error)
    messageStore.message = 'Failed to create album.'
    messageStore.warn = true
    messageStore.showMessage = true
  }
}
</script>
