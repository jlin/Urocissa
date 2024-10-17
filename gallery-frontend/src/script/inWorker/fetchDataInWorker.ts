import { useWorkerStore } from '@/store/workerStore'
import { useDataLengthStore } from '@/store/dataLengthStore'
import { bindActionDispatch } from 'typesafe-agent-events'
import { toDataWorker } from '@/worker/workerApi'

export function fetchDataInWorker(batch: number) {
  const workerStore = useWorkerStore()

  if (workerStore.worker === null) {
    workerStore.initializeWorker()
  }

  const dataLengthStore = useDataLengthStore()
  const dataWorker = workerStore.worker!

  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))
  const timestamp = dataLengthStore.timestamp
  if (timestamp !== null) {
    // Photo data is fetched batch by batch
    postToWorker.fetchData({
      batch: batch,
      timestamp: timestamp
    })
  }
}
