import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useMessageStore = (isolationId: IsolationId) =>
  defineStore('messageStore' + isolationId, {
    state: (): {
      message: string
      showMessage: boolean
      warn: boolean
    } => ({
      message: '',
      showMessage: false,
      warn: false
    }),
    actions: {}
  })()
