import { useDataLengthStore } from '@/store/dataLengthStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function fetchScrollbarInWorker() {
  const workerStore = useWorkerStore()
  const dataLengthStore = useDataLengthStore()

  if (workerStore.worker === null) {
    workerStore.initializeWorker()
  }
  const dataWorker = workerStore.worker!

  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))
  const timestamp = dataLengthStore.timestamp

  if (dataLengthStore.dataLength === 0) {
    return
  }

  if (timestamp !== null) {
    postToWorker.fetchScrollbar({
      timestamp: timestamp
    })
  }
}
