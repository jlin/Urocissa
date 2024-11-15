<template>
  <v-menu>
    <template v-slot:activator="{ props }">
      <v-btn v-bind="props" icon>
        <v-icon>mdi-dots-vertical</v-icon>
      </v-btn>
    </template>
    <v-list>
      <v-list-item
        prepend-icon="mdi-archive-arrow-down"
        v-if="collectionStore.editModeCollection.size === 1"
        @click="setCover()"
      >
        <v-list-item-title class="wrap">Set as Album Cover</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts" setup>
import { useCollectionStore } from '@/store/collectionStore'
import { useDataStore } from '@/store/dataStore'
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'

const collectionStore = useCollectionStore('mainId')
const dataStore = useDataStore('mainId')
const route = useRoute()

const albumId = computed(() => {
  const path = route.path
  if (path.startsWith('/album-')) {
    // Extract the album identifier (e.g., 'album-3jwdp89ndzovner66kqicnu2m37yuhjsqg2g6psro86izspduz3u4if02wughxm3')
    const segments = path.split('/')
    const albumSegment = segments.find((segment) => segment.startsWith('album-'))
    return albumSegment ? albumSegment.slice('album-'.length) : undefined
  } else {
    return undefined
  }
})

const setCover = async () => {
  try {
    // Ensure only one item is selected
    if (collectionStore.editModeCollection.size !== 1) {
      throw new Error('Exactly one item must be selected to set as cover.')
    }

    // Extract the only cover hash index from the collection
    const coverHashIndex = Array.from(collectionStore.editModeCollection)[0]

    // Retrieve the cover hash from the data store
    const dataEntry = dataStore.data.get(coverHashIndex)
    if (!dataEntry || !dataEntry.database || !dataEntry.database.hash) {
      throw new Error('Invalid cover hash data.')
    }
    const coverHash = dataEntry.database.hash

    // Retrieve the album ID
    const currentAlbumId = albumId.value
    if (!currentAlbumId) {
      throw new Error('Album ID is not available.')
    }

    // Prepare the payload
    const payload = {
      albumId: currentAlbumId,
      coverHash: coverHash
    }

    // Send POST request to the Rocket endpoint
    const response = await axios.post('/post/set_album_cover', payload, {
      headers: {
        'Content-Type': 'application/json'
      }
    })

    if (response.status === 200) {
      console.log('set cover successfully')
    } else {
      throw new Error(`Unexpected response status: ${response.status}`)
    }
  } catch (error: any) {
    console.error('Error setting album cover:', error)
    // Optionally, show an error message
  }
}
</script>

<style scoped>
/* Add any component-specific styles here */
</style>
