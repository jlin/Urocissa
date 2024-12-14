import { AbstractData } from '@/script/common/types'
import { EditAlbumsParams, EditTagsParams } from '@/worker/workerApi'
import { defineStore } from 'pinia'
import { useDataStore } from './dataStore'

export const useOptimisticStore = (isolationId: string) =>
  defineStore('optimisticUpdateStore' + isolationId, {
    state: (): {
      backupData: Map<number, AbstractData> // dataIndex -> data
      queueTagsUpdate: EditTagsParams[]
      queueAlbumsUpdate: EditAlbumsParams[]
    } => ({
      backupData: new Map(),
      queueTagsUpdate: [],
      queueAlbumsUpdate: []
    }),
    actions: {
      clearAll() {
        this.backupData.clear()
        this.queueTagsUpdate = []
        this.queueAlbumsUpdate = []
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

        if (
          pushIntoQueue && // only the new task should be pushed
          payload.indexSet.size !== 0
        ) {
          // some data has not been fetched yet
          this.queueTagsUpdate.push(payload)
        }
      },
      optimisticUpdateAlbums(payload: EditAlbumsParams, pushIntoQueue: boolean) {
        const dataStore = useDataStore(isolationId)
        for (const index of dataStore.data.keys()) {
          if (payload.indexSet.has(index)) {
            const addTagsResult = dataStore.addAlbums(index, payload.addAlbumsArray)

            const removeTagsResult = dataStore.removeAlbums(index, payload.removeAlbumsArray)
            if (addTagsResult && removeTagsResult) {
              payload.indexSet.delete(index)
            }
          }
        }

        if (
          pushIntoQueue && // only the new task should be pushed
          payload.indexSet.size !== 0
        ) {
          // some data has not been fetched yet
          this.queueAlbumsUpdate.push(payload)
        }
      },
      selfUpdate() {
        this.queueTagsUpdate.forEach((payload) => {
          this.optimisticUpdateTags(payload, false)
        })
        this.queueAlbumsUpdate.forEach((payload) => {
          this.optimisticUpdateAlbums(payload, false)
        })
      }
    }
  })()
