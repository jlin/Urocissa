import { defineStore } from 'pinia'

export const useInitializedStore = (isolationId: string = '') =>
  defineStore({
    id: 'initializedStore' + isolationId,
    state: (): {
      login: boolean
      initialized: boolean
    } => ({
      login: false,
      initialized: false
    }),
    actions: {}
  })()
