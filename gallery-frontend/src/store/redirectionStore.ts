import router from '@/script/routes'
import { IsolationId } from '@type/types'
import { defineStore } from 'pinia'

export const useRedirectionStore = (isolationId: IsolationId) =>
  defineStore('redirectionStore' + isolationId, {
    state: (): {
      redirection: null | string
    } => ({
      redirection: null
    }),
    actions: {
      async redirectionToLogin() {
        console.warn('401 Unauthorized detected, redirecting to /login')
        this.redirection = router.currentRoute.value.fullPath
        await router.push('/login')
      }
    }
  })()
