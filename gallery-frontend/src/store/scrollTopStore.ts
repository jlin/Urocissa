import { defineStore } from 'pinia'

export const useScrollTopStore = defineStore('scrollTopStore', {
  state: (): {
    scrollTop: number
  } => ({
    scrollTop: 0
  }),
  actions: {}
})
