import { defineStore } from 'pinia'

export const useScrollTopStore = (isolationId: string = '') =>
  defineStore('scrollTopStore' + isolationId, {
    state: (): {
      scrollTop: number
    } => ({
      scrollTop: 0
    }),
    actions: {}
  })()
