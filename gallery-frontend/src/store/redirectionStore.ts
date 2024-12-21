import { IsolationId } from '@/script/common/types'
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
