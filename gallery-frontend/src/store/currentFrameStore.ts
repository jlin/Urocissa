import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useCurrentFrameStore = (isolationId: IsolationId) =>
  defineStore('currentFrameStore' + isolationId, {
    state: (): {
      currentFrame: HTMLVideoElement | null // unit: second
    } => ({
      currentFrame: null
    }),
    actions: {}
  })()
