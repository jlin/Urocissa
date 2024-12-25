import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useRerenderStore = (isolationId: IsolationId) =>
  defineStore('rerenderStore' + isolationId, {
    state: (): {
      homeKey: boolean
      homeIsolatedKey: boolean
    } => ({
      homeKey: false,
      homeIsolatedKey: false
    }),
    actions: {
      rerenderHome() {
        this.homeKey = !this.homeKey
      },
      rerenderHomeIsolated() {
        this.homeIsolatedKey = !this.homeIsolatedKey
      }
    }
  })()
