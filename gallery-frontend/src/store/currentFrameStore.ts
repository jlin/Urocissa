import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useCurrentFrameStore = (isolationId: IsolationId) =>
  defineStore('currentFrameStore' + isolationId, {
    state: (): {
      currentFrame: number | undefined // unit: second
    } => ({
      currentFrame: undefined
    }),
    actions: {}
  })()
