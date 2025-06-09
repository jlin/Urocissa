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
      _getTimestampFromToken(): number | null {
        if (this.timestampToken == null) return null
        try {
          const decoded = jwtDecode<JwtPayload>(this.timestampToken)
          return typeof decoded.timestamp === 'number' ? decoded.timestamp : null
        } catch (err) {
          console.warn('Invalid JWT:', err)
          return null
        }
      },
      _getTimestampFromHashToken(hash: string): number | undefined {
        const token = this.hashTokenMap.get(hash)
        if (token === undefined) return undefined
        try {
          const decoded = jwtDecode<JwtPayload>(token)
          return typeof decoded.timestamp === 'number' ? decoded.timestamp : undefined
        } catch (err) {
          console.warn(`Invalid JWT for hash: ${hash}`, err)
          return undefined
        }
      }
    }
  })()
