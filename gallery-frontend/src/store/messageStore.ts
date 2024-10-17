import { defineStore } from 'pinia'

export const useMessageStore = defineStore({
  id: 'messageStore',
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
})
