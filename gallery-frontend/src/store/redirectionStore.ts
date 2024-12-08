import { defineStore } from 'pinia'

export const useRedirectionStore = (isolationId: string) =>
  defineStore({
    id: 'redirectionStore' + isolationId,
    state: (): {
      redirection: null | string
    } => ({
      redirection: null
    }),
    actions: {}
  })()
