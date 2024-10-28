import { useDataLengthStore } from '@/store/dataLengthStore'
import { useLocationStore } from '@/store/locationStore'
import { useQueueStore } from '@/store/queueStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

/**
 * Fetches a row of data using a web worker if it isn't already queued.
 *
 * @param {number} index - The index of the row to fetch.
 */
export function fetchRowInWorker(index: number) {
  const queueStore = useQueueStore()
  if (queueStore.row.has(index)) {
    return
  }
  // If a specific row is anchored, fetch only that row
  const locationStore = useLocationStore()
  if (locationStore.anchor !== null && locationStore.anchor !== index) {
    return
  }
  const dataLengthStore = useDataLengthStore()

  if (index > dataLengthStore.rowLength - 1 || index < 0) {
    return
  }
  const workerStore = useWorkerStore()

  if (workerStore.worker === null) {
    workerStore.initializeWorker()
  }
  const dataWorker = workerStore.worker!

  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))
  const timestamp = dataLengthStore.timestamp

  if (timestamp !== null) {
    queueStore.row.add(index)
    postToWorker.fetchRow({
      index: index,
      timestamp: timestamp,
      windowWidth: dataLengthStore.windowWidth,
      isLastRow: index === dataLengthStore.rowLength - 1
    })
  }
}
