import { defineStore } from 'pinia'
export const useInfoStore = (isolationId: string = '') =>
  defineStore({
    id: 'infoStore' + isolationId,
    state: (): {
      showInfo: boolean
    } => ({
      showInfo: false
    }),
    actions: {}
  })()
