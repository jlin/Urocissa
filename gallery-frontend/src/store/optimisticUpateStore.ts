import { AbstractData } from '@/script/common/types'
import { EditTagsParams } from '@/worker/workerApi'
import { defineStore } from 'pinia'
import { useDataStore } from './dataStore'

export const useOptimisticStore = (isolationId: string) =>
  defineStore({
    id: 'optimisticUpdateStore' + isolationId,
    state: (): {
      backupData: Map<number, AbstractData> // dataIndex -> data
      queueOptimisticUpdate: EditTagsParams[]
    } => ({
      backupData: new Map(),
      queueOptimisticUpdate: []
    }),
    actions: {
      optimisticUpdateTags(payload: EditTagsParams) {
        const dataStore = useDataStore(isolationId)
        payload.indexArray.forEach((index) => {
          const addTagsResult = dataStore.addTags(index, payload.addTagsArray)

          const removeTagsResult = dataStore.removeTags(index, payload.removeTagsArray)
          if (addTagsResult && removeTagsResult) {
            payload.indexArray = payload.indexArray.filter((i) => i !== index)
          }
        })

        if (payload.indexArray.length !== 0) {
          // some data has not been fetched yet
          this.queueOptimisticUpdate.push(payload)
        }
      },
      selfUpdate() {
        console.log('perform selfupdate')

        this.queueOptimisticUpdate.forEach((payload) => {
          this.optimisticUpdateTags(payload)
        })
      }
    }
  })()
