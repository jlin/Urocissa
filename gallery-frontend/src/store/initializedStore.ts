import { defineStore } from 'pinia'

export const useInitializedStore = defineStore({
  id: 'initializedStore',
  state: (): {
    login: boolean
    initialized: boolean
  } => ({
    login: false,
    initialized: false
  }),
  actions: {}
})
