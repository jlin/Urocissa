import { IsolationId } from '@/script/common/types'
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
