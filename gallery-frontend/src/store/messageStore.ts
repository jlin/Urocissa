import { IsolationId } from '@type/types'
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
    actions: {
      showInfo(message: string) {
        this.message = message
        this.warn = false
        this.showMessage = true
      },
      showWarn(message: string) {
        this.message = message
        this.warn = true
        this.showMessage = true
      }
    }
  })()
