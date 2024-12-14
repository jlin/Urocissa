import { defineStore } from 'pinia'
export const useSimilarStore = (isolationId: string) =>
  defineStore('similarStore' + isolationId, {
    state: (): {
      usingSimilarMode: boolean
    } => ({
      usingSimilarMode: false
    }),
    actions: {}
  })()
