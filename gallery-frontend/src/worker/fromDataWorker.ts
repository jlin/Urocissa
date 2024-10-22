import { useDataStore } from '@/store/dataStore'
import { Prefetch, SlicedDataItem } from '@/script/common/commonType'
import { useDataLengthStore } from '@/store/dataLengthStore'
import { useMessageStore } from '@/store/messageStore'
import { useInitializedStore } from '@/store/initializedStore'
import { useTagStore } from '@/store/tagStore'
import { createHandler } from 'typesafe-agent-events'
import { fromDataWorker } from '@/worker/workerApi'
import { useScrollbarStore } from '@/store/scrollbarStore'
import { useOffsetStore } from '@/store/offsetStore'
import { useRowStore } from '@/store/rowStore'
import { useLocationStore } from '@/store/locationStore'
import { fetchScrollbarInWorker } from '@/script/inWorker/fetchScrollbarInWorker'
import { useModalStore } from '@/store/modalStore'

const workerHandlerMap = new Map<Worker, (e: MessageEvent) => void>()

export function handleDataWorkerReturn(dataWorker: Worker) {
  const dataStore = useDataStore()
  const messageStore = useMessageStore()
  const dataLengthStore = useDataLengthStore()
  const tagStore = useTagStore()
  const initializedStore = useInitializedStore()
  const scrollbarStore = useScrollbarStore()
  const offsetStore = useOffsetStore()
  const rowStore = useRowStore()
  const locationStore = useLocationStore()
  const modalStore = useModalStore()
  const handler = createHandler<typeof fromDataWorker>({
    returnData: (payload) => {
      const slicedDataArray: SlicedDataItem[] = payload.slicedDataArray
      slicedDataArray.forEach(({ index, data }) => {
        dataStore.data.set(index, data)
        dataStore.hashMapData.set(data.hash, index)
      })

      dataStore.batchFetched.set(payload.batch, true)
    },
    fetchRowReturn: (payload) => {
      const timestamp = payload.timestamp
      const rowWithOffset = payload.rowWithOffset
      const windowWidth = rowWithOffset.windowWidth
      if (windowWidth !== dataLengthStore.windowWidth) {
        return
      }

      const offset = rowWithOffset.offset

      const row = rowWithOffset.row
      if (locationStore.anchor !== null && locationStore.anchor !== row.rowIndex) {
        return
      }
      const index = row.rowIndex
      if (timestamp === dataLengthStore.timestamp && !offsetStore.offset.has(index)) {
        offsetStore.offset.set(index, offset)
        row.offset = offsetStore.accumulatedOffset(row.rowIndex)
        rowStore.rowData.forEach((row) => {
          if (row.rowIndex > index) {
            row.offset = row.offset + offset
          }
        })

        rowStore.rowData.set(row.rowIndex, row)

        dataLengthStore.totalHeight = dataLengthStore.totalHeight + offset
        offsetStore.accumulatedAll = offsetStore.accumulatedAll + offset
      }

      dataLengthStore.updateFetchRowTrigger = !dataLengthStore.updateFetchRowTrigger
      dataLengthStore.updateVisibleRowTrigger = !dataLengthStore.updateVisibleRowTrigger
    },
    prefetchReturn: async (payload) => {
      const result: Prefetch = payload.result
      dataLengthStore.timestamp = result.timestamp
      dataLengthStore.updateVisibleRowTrigger = !dataLengthStore.updateVisibleRowTrigger
      dataLengthStore.calculateLength(result.dataLength)
      dataLengthStore.locateTo = result.locateTo
      initializedStore.initialized = true

      if (!tagStore.fetched) {
        await tagStore.fetchTags()
      }
      fetchScrollbarInWorker()

      dataLengthStore.updateFetchRowTrigger = !dataLengthStore.updateFetchRowTrigger
    },
    editTagsReturn: (payload) => {
      if (payload.returnedTagsArray !== undefined) {
        tagStore.applyTags(payload.returnedTagsArray)
      } else {
        console.warn('editTags did not find tags')
      }
      modalStore.showEditTagsModal = false
      messageStore.message = payload.result
      messageStore.warn = payload.warn
      messageStore.showMessage = true
    },
    deleteDataReturn: (payload) => {
      messageStore.message = payload.result
      messageStore.warn = payload.warn
      messageStore.showMessage = true
    },
    fetchScrollbarReturn: (payload) => {
      console.log('payload.scrollbarDataArray is ', payload.scrollbarDataArray)
      scrollbarStore.initialize(payload.scrollbarDataArray)
    }
  })

  const messageHandler = (e: MessageEvent) => {
    handler(e.data)
  }

  dataWorker.addEventListener('message', messageHandler)
  workerHandlerMap.set(dataWorker, messageHandler)
}

export function removeHandleDataWorkerReturn(dataWorker: Worker) {
  const messageHandler = workerHandlerMap.get(dataWorker)
  if (messageHandler) {
    dataWorker.removeEventListener('message', messageHandler)
    workerHandlerMap.delete(dataWorker)
  }
}
