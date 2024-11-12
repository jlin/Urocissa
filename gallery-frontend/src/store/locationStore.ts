import { defineStore } from 'pinia'

export const useLocationStore = (isolationId: string = '') =>
  defineStore('locateStore' + isolationId, {
    state: (): {
      /**
       * Index of the first photo that appears (partially) in the viewport
       */
      locationIndex: number | null
      anchor: number | null
    } => ({
      locationIndex: null,
      anchor: null
    }),
    actions: {
      clearAll() {
        this.locationIndex = null
        this.anchor = null
      },
      triggerForResize() {
        this.anchor = null
      }
    }
  })()
