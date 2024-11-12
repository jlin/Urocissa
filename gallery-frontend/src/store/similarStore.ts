import { defineStore } from 'pinia'
export const useSimilarStore = (isolationId: string = '') =>
  defineStore({
    id: 'similarStore' + isolationId,
    state: (): {
      usingSimilarMode: boolean
    } => ({
      usingSimilarMode: false
    }),
    actions: {}
  })()
