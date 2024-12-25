import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useRerenderStore = (isolationId: IsolationId) =>
  defineStore('rerenderStore' + isolationId, {
    state: (): {
      homeIsolatedKey: boolean
    } => ({
      homeIsolatedKey: false
    }),
    actions: {
      rerenderHomeIsolated() {
        this.homeIsolatedKey = !this.homeIsolatedKey
      }
    }
  })()
