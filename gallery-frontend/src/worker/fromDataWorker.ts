import { useDataStore } from '@/store/dataStore'
import { Prefetch, SlicedDataItem } from '@/script/common/commonType'
import { usePrefetchStore } from '@/store/prefetchStore'
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
import router from '@/script/routes'
import axios from 'axios'
import { useConfigStore } from '@/store/configStore'
const workerHandlerMap = new Map<Worker, (e: MessageEvent) => void>()

export function handleDataWorkerReturn(dataWorker: Worker) {
  const dataStore = useDataStore()
  const messageStore = useMessageStore()
  const prefetchStore = usePrefetchStore()
  const tagStore = useTagStore()
  const initializedStore = useInitializedStore()
  const scrollbarStore = useScrollbarStore()
  const offsetStore = useOffsetStore()
  const rowStore = useRowStore()
  const locationStore = useLocationStore()
  const modalStore = useModalStore()
  const configStore = useConfigStore()
  const handler = createHandler<typeof fromDataWorker>({
    returnData: (payload) => {
      const slicedDataArray: SlicedDataItem[] = payload.slicedDataArray
      slicedDataArray.forEach(({ index, data }) => {
        dataStore.data.set(index, data)
        dataStore.hashMapData.set(data.hash(), index)
      })

      dataStore.batchFetched.set(payload.batch, true)
    },
    fetchRowReturn: (payload) => {
      const timestamp = payload.timestamp
      const rowWithOffset = payload.rowWithOffset
      const windowWidth = rowWithOffset.windowWidth
      if (windowWidth !== prefetchStore.windowWidth) {
        return
      }

      const offset = rowWithOffset.offset

      const row = rowWithOffset.row
      if (locationStore.anchor !== null && locationStore.anchor !== row.rowIndex) {
        return
      }
      const index = row.rowIndex
      if (timestamp === prefetchStore.timestamp && !offsetStore.offset.has(index)) {
        offsetStore.offset.set(index, offset)
        row.offset = offsetStore.accumulatedOffset(row.rowIndex)
        rowStore.rowData.forEach((row) => {
          if (row.rowIndex > index) {
            row.offset = row.offset + offset
          }
        })

        rowStore.rowData.set(row.rowIndex, row)

        prefetchStore.totalHeight = prefetchStore.totalHeight + offset
        offsetStore.accumulatedAll = offsetStore.accumulatedAll + offset
      }

      prefetchStore.updateFetchRowTrigger = !prefetchStore.updateFetchRowTrigger
      prefetchStore.updateVisibleRowTrigger = !prefetchStore.updateVisibleRowTrigger
    },
    prefetchReturn: async (payload) => {
      const result: Prefetch = payload.result
      if (result.dataLength === 0) {
        messageStore.message = 'Wow, so empty! Try adding some photos here!'
        messageStore.warn = false
        messageStore.showMessage = true
      }
      prefetchStore.timestamp = result.timestamp
      prefetchStore.updateVisibleRowTrigger = !prefetchStore.updateVisibleRowTrigger
      prefetchStore.calculateLength(result.dataLength)
      prefetchStore.locateTo = result.locateTo
      initializedStore.initialized = true

      // Perform initialization:
      if (!tagStore.fetched) {
        await tagStore.fetchTags()
      }
      fetchScrollbarInWorker()

      try {
        const response = await axios.get('/get/get-config.json')

        configStore.disableImg = response.data.disableImg
      } catch (error) {
        console.error('Error fetching config:', error)
        throw error
      }

      prefetchStore.updateFetchRowTrigger = !prefetchStore.updateFetchRowTrigger
    },
    editTagsReturn: (payload) => {
      if (payload.returnedTagsArray !== undefined) {
        tagStore.applyTags(payload.returnedTagsArray)
      } else {
        console.warn('editTags did not find tags')
      }
      modalStore.showEditTagsModal = false
      messageStore.showMessage = true
    },
    fetchScrollbarReturn: (payload) => {
      console.log('payload.scrollbarDataArray is ', payload.scrollbarDataArray)
      scrollbarStore.initialize(payload.scrollbarDataArray)
    },
    notification: function (payload: { message: string; messageType: 'info' | 'warn' }): void {
      messageStore.message = payload.message
      messageStore.warn = payload.messageType === 'warn'
      messageStore.showMessage = true
    },
    unauthorized: async () => {
      await router.push('/login')
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
