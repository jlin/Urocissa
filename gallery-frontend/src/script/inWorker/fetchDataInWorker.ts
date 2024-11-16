import { useWorkerStore } from '@/store/workerStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import { bindActionDispatch } from 'typesafe-agent-events'
import { toDataWorker } from '@/worker/workerApi'

export function fetchDataInWorker(batch: number, isolationId: string) {
  const workerStore = useWorkerStore(isolationId)

  if (workerStore.worker === null) {
    workerStore.initializeWorker(isolationId)
  }

  const prefetchStore = usePrefetchStore(isolationId)
  const dataWorker = workerStore.worker

  const postToWorker = bindActionDispatch(toDataWorker, (action) => {
    if (dataWorker) {
      dataWorker.postMessage(action)
    }
  })
  const timestamp = prefetchStore.timestamp
  if (timestamp !== null) {
    // Photo data is fetched batch by batch
    postToWorker.fetchData({
      batch: batch,
      timestamp: timestamp
    })
  }
}
