import { defineStore } from 'pinia'
export const useInfoStore = (isolationId: string) =>
  defineStore('infoStore' + isolationId, {
    state: (): {
      showInfo: boolean
    } => ({
      showInfo: false
    }),
    actions: {}
  })()
