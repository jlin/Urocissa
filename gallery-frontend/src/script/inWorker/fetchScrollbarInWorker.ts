import { usePrefetchStore } from '@/store/prefetchStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function fetchScrollbarInWorker() {
  const workerStore = useWorkerStore()
  const prefetchStore = usePrefetchStore()

  if (workerStore.worker === null) {
    workerStore.initializeWorker()
  }
  const dataWorker = workerStore.worker!

  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))
  const timestamp = prefetchStore.timestamp

  if (prefetchStore.dataLength === 0) {
    return
  }

  if (timestamp !== null) {
    postToWorker.fetchScrollbar({
      timestamp: timestamp
    })
  }
}
