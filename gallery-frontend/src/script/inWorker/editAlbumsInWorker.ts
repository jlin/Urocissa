import { usePrefetchStore } from '@/store/prefetchStore'
import { useAlbumStore } from '@/store/albumStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'
import { useOptimisticStore } from '@/store/optimisticUpateStore'
import { IsolationId } from '../common/types'

export function editAlbumsInWorker(
  indexArray: number[],
  addAlbumsArray: string[],
  removeAlbumsArray: string[],
  isolationId: IsolationId
) {
  const prefetchStore = usePrefetchStore(isolationId)
  const workerStore = useWorkerStore('mainId')
  const albumStore = useAlbumStore('mainId')
  const optimisticUpateStore = useOptimisticStore(isolationId)

  albumStore.fetched = false

  if (workerStore.worker === null) {
    workerStore.initializeWorker('mainId')
  }

  const dataWorker = workerStore.worker

  const postToWorker = bindActionDispatch(toDataWorker, (action) => {
    if (dataWorker) {
      dataWorker.postMessage(action)
    }
  })
  const timestamp = prefetchStore.timestamp
  if (timestamp !== null) {
    const payload = {
      indexSet: new Set(indexArray),
      addAlbumsArray: [...addAlbumsArray],
      removeAlbumsArray: [...removeAlbumsArray],
      timestamp: timestamp
    }
    postToWorker.editAlbums(payload)
    optimisticUpateStore.optimisticUpdateAlbums(payload, false)
  }
}
