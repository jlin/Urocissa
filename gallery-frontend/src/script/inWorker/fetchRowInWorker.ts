import { useDataLengthStore } from '@/store/dataLengthStore'
import { useLocationStore } from '@/store/locationStore'
import { useQueueStore } from '@/store/queueStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { clamp } from 'lodash'
import { bindActionDispatch } from 'typesafe-agent-events'

/**
 * Fetches a row of data using a web worker if it isn't already queued.
 *
 * @param {number} index - The index of the row to fetch.
 */
export function fetchRowInWorker(index: number) {
  const dataLengthStore = useDataLengthStore()
  const locationStore = useLocationStore()
  const queueStore = useQueueStore()

  if (dataLengthStore.rowLength === 0) {
    return // No data to fetch
  }

  index = clamp(index, 0, dataLengthStore.rowLength - 1)

  if (queueStore.row.has(index)) {
    return // Already fetched
  }

  if (locationStore.anchor !== null && locationStore.anchor !== index) {
    return // If a specific row is anchored, this make sure to fetch only that row
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
