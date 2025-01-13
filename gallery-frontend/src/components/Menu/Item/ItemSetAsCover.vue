<template>
  <v-list-item prepend-icon="mdi-archive-arrow-down" @click="setAsCover()">
    <v-list-item-title class="wrap">Set as Cover</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { useDataStore } from '@/store/dataStore'
import { useMessageStore } from '@/store/messageStore'
import { getCookiesJwt, getIsolationIdByRoute } from '@/script/common/functions'

import axios from 'axios'

import { bindActionDispatch } from 'typesafe-agent-events'
import { toImgWorker } from '@/worker/workerApi'
import { useWorkerStore } from '@/store/workerStore'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const collectionStore = useCollectionStore(isolationId)
const dataStore = useDataStore(isolationId)
const messageStore = useMessageStore('mainId')
const workerStore = useWorkerStore('mainId')

const setAsCover = async () => {
  if (collectionStore.editModeCollection.size !== 1) {
    console.warn('editModeCollection must contain exactly one item to set as cover.')
    return
  }

  const coverIndex = Array.from(collectionStore.editModeCollection)[0]
  if (coverIndex === undefined) {
    return
  }

  const coverHash = dataStore.data.get(coverIndex)?.database?.hash
  if (coverHash === undefined) {
    return
  }

  const albumId = route.params.hash

  if (typeof albumId !== 'string') {
    return
  }

  await axios.post(
    '/post/set_album_cover',
    {
      albumId: albumId,
      coverHash: coverHash
    },
    {
      headers: {
        'Content-Type': 'application/json'
      }
    }
  )

  const postToWorker = bindActionDispatch(toImgWorker, (action) => {
    const worker = workerStore.imgWorker[0]
    if (worker) {
      worker.postMessage(action)
    } else {
      throw new Error(`Worker not found for index: 0`)
    }
  })

  const mainDataStore = useDataStore('mainId')
  const albumIndex = mainDataStore.hashMapData.get(albumId)
  if (albumIndex !== undefined) {
    postToWorker.processImage({
      index: albumIndex,
      hash: coverHash,
      devicePixelRatio: window.devicePixelRatio,
      jwt: getCookiesJwt()
    })
    const abstractData = mainDataStore.data.get(albumIndex)
    const album = abstractData?.album
    if (album) {
      album.cover = coverHash

      mainDataStore.data.set(albumIndex, abstractData)
      postToWorker.processSmallImage({
        index: albumIndex,
        hash: coverHash,
        width: 300,
        height: 300,
        devicePixelRatio: window.devicePixelRatio,
        jwt: getCookiesJwt(),
        albumMode: true
      })

      messageStore.message = 'Successfully set as cover.'
      messageStore.warn = false
      messageStore.showMessage = true

      collectionStore.editModeOn = false
    }
  }
}
</script>
