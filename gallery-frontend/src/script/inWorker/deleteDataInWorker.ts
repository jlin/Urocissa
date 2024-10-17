import { useDataLengthStore } from '@/store/dataLengthStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function deleteDataInWorker(indexArray: number[]) {
  const workerStore = useWorkerStore()
  const dataLengthStore = useDataLengthStore()

  if (workerStore.worker === null) {
    workerStore.initializeWorker()
  }
  const dataWorker = workerStore.worker!

  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))
  const timestamp = dataLengthStore.timestamp
  if (timestamp !== null) {
    postToWorker.deleteData({
      indexArray: indexArray,
      timestamp: timestamp
    })
  }
}
