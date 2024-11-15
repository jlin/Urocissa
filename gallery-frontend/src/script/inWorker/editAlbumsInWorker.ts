import { usePrefetchStore } from '@/store/prefetchStore'
import { useAlbumStore } from '@/store/albumStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function editAlbumsInWorker(
  idArray: number[],
  addAlbumsArray: string[],
  removeAlbumsArray: string[]
) {
  const workerStore = useWorkerStore('mainId')
  const prefetchStore = usePrefetchStore('mainId')
  const albumStore = useAlbumStore('mainId')

  albumStore.fetched = false

  if (workerStore.worker === null) {
    workerStore.initializeWorker('mainId')
  }

  const dataWorker = workerStore.worker!
  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))
  const timestamp = prefetchStore.timestamp
  if (timestamp !== null) {
    const payload = {
      idArray: [...idArray],
      addAlbumsArray: [...addAlbumsArray],
      removeAlbumsArray: [...removeAlbumsArray],
      timestamp: timestamp
    }
    postToWorker.editAlbums(payload)
  }
}
