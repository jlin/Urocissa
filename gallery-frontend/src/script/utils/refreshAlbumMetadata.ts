import { fetchDataInWorker } from '@/api/fetchData'
import { useDataStore } from '@/store/dataStore'
import { useMessageStore } from '@/store/messageStore'
import { useWorkerStore } from '@/store/workerStore'
import { bindActionDispatch } from 'typesafe-agent-events'
import { toImgWorker } from '@/worker/workerApi'
import { watch } from 'vue'
import { useShareStore } from '@/store/shareStore'
import { useTokenStore } from '@/store/tokenStore'
export function refreshAlbumMetadata(albumId: string) {
  const dataStore = useDataStore('mainId')
  const workerStore = useWorkerStore('mainId')
  const messageStore = useMessageStore('mainId')
  const shareStore = useShareStore('mainId')
  const tokenStore = useTokenStore('mainId')

  const albumIndex = dataStore.hashMapData.get(albumId)
  if (albumIndex === undefined) {
    console.error(`cannot find albumIndex with albumId = ${albumId}`)
    return
  }

  // perform after fetchDataInWorker
  const stopWatch = watch(
    () => dataStore.data.get(albumIndex),
    () => {
      const postToWorker = bindActionDispatch(toImgWorker, (action) => {
        const worker = workerStore.imgWorker[0]
        if (worker) {
          worker.postMessage(action)
        } else {
          throw new Error(`Worker not found for index: 0`)
        }
      })

      const album = dataStore.data.get(albumIndex)?.album

      if (!album) {
        console.error(`cannot find album with albumIndex = ${albumIndex}`)
        return
      }

      const coverHash = album.cover

      if (coverHash === null) {
        return
      }

      const timestampToken = tokenStore.timestampToken
      if (timestampToken === null) {
        throw new Error('timestampToken is null')
      }

      const hashToken = tokenStore.hashTokenMap.get(coverHash)
      if (hashToken === undefined) {
        throw new Error('hashToken is undefined')
      }

      postToWorker.processImage({
        index: albumIndex,
        hash: coverHash,
        devicePixelRatio: window.devicePixelRatio,
        albumId: shareStore.albumId,
        shareId: shareStore.shareId,
        timestampToken,
        hashToken
      })

      postToWorker.processSmallImage({
        index: albumIndex,
        hash: coverHash,
        width: 300,
        height: 300,
        devicePixelRatio: window.devicePixelRatio,
        albumMode: true,
        albumId: shareStore.albumId,
        shareId: shareStore.shareId,
        timestampToken,
        hashToken
      })

      messageStore.success(`Album cover updated successfully`)
      stopWatch()
    }
  )

  fetchDataInWorker('single', albumIndex, 'mainId')
}
