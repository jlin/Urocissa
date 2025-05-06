import { IsolationId, ResolvedShare } from '@type/types'
import { defineStore } from 'pinia'

export const useShareStore = (isolationId: IsolationId) =>
  defineStore('shareStore' + isolationId, {
    state: (): {
      albumId: null | string
      shareId: null | string
      resolvedShare: null | ResolvedShare
    } => ({
      albumId: null,
      shareId: null,
      resolvedShare: null
    }),
    actions: {}
  })()
