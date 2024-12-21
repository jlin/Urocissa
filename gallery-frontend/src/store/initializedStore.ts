import { IsolationId } from '@/script/common/types'
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
