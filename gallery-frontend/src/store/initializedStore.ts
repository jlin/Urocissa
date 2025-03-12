import { IsolationId } from '@type/types'
import { defineStore } from 'pinia'

export const useInitializedStore = (isolationId: IsolationId) =>
  defineStore('initializedStore' + isolationId, {
    state: (): {
      login: boolean
      initialized: boolean
    } => ({
      login: false,
      initialized: false
    }),
    actions: {}
  })()
