import { defineStore } from 'pinia'

export const useCollectionStore = (isolationId: string = '') =>
  defineStore({
    id: 'collectionStore' + isolationId,
    state: (): {
      editModeOn: boolean
      editModeCollection: Set<number>
      lastClick: null | number
    } => ({
      editModeOn: false,
      editModeCollection: new Set(),
      lastClick: null
    }),
    actions: {
      addApi(index: number) {
        this.editModeCollection.add(index)
        if (this.editModeCollection.size === 0) {
          this.editModeOn = false
        }
      },
      deleteApi(index: number) {
        this.editModeCollection.delete(index)
        if (this.editModeCollection.size === 0) {
          this.editModeOn = false
        }
      }
    }
  })()
