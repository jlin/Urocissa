import { IsolationId } from '@type/types'
import { defineStore } from 'pinia'

export const useRedirectionStore = (isolationId: IsolationId) =>
  defineStore('redirectionStore' + isolationId, {
    state: (): {
      redirection: null | string
    } => ({
      redirection: null
    }),
    actions: {}
  })()
