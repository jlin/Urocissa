import { defineStore } from 'pinia'
export const useSimilarStore = defineStore({
  id: 'similarStore',
  state: (): {
    usingSimilarMode: boolean
  } => ({
    usingSimilarMode: false
  }),
  actions: {}
})
