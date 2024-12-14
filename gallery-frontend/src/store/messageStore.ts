import { defineStore } from 'pinia'

export const useMessageStore = (isolationId: string) =>
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
