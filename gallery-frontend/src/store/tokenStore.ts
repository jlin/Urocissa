import { IsolationId } from '@type/types'
import { defineStore } from 'pinia'

export const useTokenStore = (isolationId: IsolationId) =>
  defineStore('tokenStore' + isolationId, {
    state: (): {
      timestampToken: string | null
    } => ({
      timestampToken: null
    }),
    actions: {}
  })()
