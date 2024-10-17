import { defineStore } from 'pinia'

export const useLocationStore = defineStore('locateStore', {
  state: (): {
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
})
