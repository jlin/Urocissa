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
      clearAll() {
        this.backupData.clear()
        this.queueOptimisticUpdate = []
      },
      optimisticUpdateTags(payload: EditTagsParams, pushIntoQueue: boolean) {
        const dataStore = useDataStore(isolationId)
        for (const index of dataStore.data.keys()) {
          if (payload.indexSet.has(index)) {
            const addTagsResult = dataStore.addTags(index, payload.addTagsArray)

            const removeTagsResult = dataStore.removeTags(index, payload.removeTagsArray)
            if (addTagsResult && removeTagsResult) {
              payload.indexSet.delete(index)
            }
          }
        }

        if (pushIntoQueue && payload.indexSet.size !== 0) {
          // some data has not been fetched yet
          this.queueOptimisticUpdate.push(payload)
        }
      },
      selfUpdate() {
        this.queueOptimisticUpdate.forEach((payload) => {
          this.optimisticUpdateTags(payload, false)
        })
      }
    }
  })()
