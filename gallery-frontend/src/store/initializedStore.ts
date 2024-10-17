import { defineStore } from 'pinia'

export const useInitializedStore = defineStore({
  id: 'initializedStore',
  state: (): {
    initialized: boolean
  } => ({
    initialized: false
  }),
  actions: {}
})
