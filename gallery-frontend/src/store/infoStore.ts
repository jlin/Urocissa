import { IsolationId } from '@type/types'
import { defineStore } from 'pinia'
export const useInfoStore = (isolationId: IsolationId) =>
  defineStore('infoStore' + isolationId, {
    state: (): {
      showInfo: boolean
    } => ({
      showInfo: false
    }),
    actions: {}
  })()
