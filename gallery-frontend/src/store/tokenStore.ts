import { IsolationId } from '@type/types'
import { jwtDecode } from 'jwt-decode'
import { defineStore } from 'pinia'

interface JwtPayload {
  timestamp: number
  [key: string]: unknown
}

export const useTokenStore = (isolationId: IsolationId) =>
  defineStore('tokenStore' + isolationId, {
    state: (): {
      timestampToken: string | null
      hashTokenMap: Map<string, string>
    } => ({
      timestampToken: null,
      hashTokenMap: new Map<string, string>()
    }),
    actions: {
      decodeTimestamp(): number | null {
        if (this.timestampToken == null) return null
        try {
          const decoded = jwtDecode<JwtPayload>(this.timestampToken)
          return typeof decoded.timestamp === 'number' ? decoded.timestamp : null
        } catch (err) {
          console.warn('Invalid JWT:', err)
          return null
        }
      }
    }
  })()
