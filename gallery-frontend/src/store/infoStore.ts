import { defineStore } from 'pinia'
export const useInfoStore = defineStore({
  id: 'infoStore',
  state: (): {
    showInfo: boolean
  } => ({
    showInfo: false
  }),
  actions: {}
})
