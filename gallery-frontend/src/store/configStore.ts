/**
 * These settings are used exclusively for personal debugging purposes.
 * They are not intended for end users or other developers.
 */

import { defineStore } from 'pinia'

export const useConfigStore = defineStore('configStore', {
  state: (): {
    enableS3: boolean
    disableImg: boolean
  } => ({
    enableS3: false,
    disableImg: false
  }),
  actions: {}
})
