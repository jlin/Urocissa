import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useScrollTopStore = (isolationId: IsolationId) =>
  defineStore('scrollTopStore' + isolationId, {
    state: (): {
      scrollTop: number
    } => ({
      scrollTop: 0
    }),
    actions: {}
  })()
