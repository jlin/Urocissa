import { usePrefetchStore } from '@/store/prefetchStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'
import { IsolationId } from '@type/types'

export function deleteDataInWorker(indexArray: number[], isolationId: IsolationId) {
  const workerStore = useWorkerStore(isolationId)
  const prefetchStore = usePrefetchStore(isolationId)

  if (workerStore.worker === null) {
    workerStore.initializeWorker(isolationId)
  }

  const dataWorker = workerStore.worker

  const postToWorker = bindActionDispatch(toDataWorker, (action) => {
    if (dataWorker) {
      dataWorker.postMessage(action)
    }
  })

  const timestamp = prefetchStore.timestamp
  if (timestamp !== null) {
    postToWorker.deleteData({
      indexArray: indexArray,
      timestamp: timestamp
    })
  }
}
