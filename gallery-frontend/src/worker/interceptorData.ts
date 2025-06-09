import { IsolationId } from '@type/types'
import { jwtDecode } from 'jwt-decode'
import { defineStore } from 'pinia'
import axios from 'axios'

interface JwtPayload {
  timestamp: number
  exp?: number
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
      },
      async refreshTimestampTokenIfExpired(): Promise<void> {
        const token = this.timestampToken
        if (token == null) return

        let decoded: JwtPayload
        try {
          decoded = jwtDecode<JwtPayload>(token)
        } catch (err) {
          console.warn('Invalid JWT:', err)
          return
        }

        const nowInSec = Math.floor(Date.now() / 1000)
        if (typeof decoded.exp === 'number' && decoded.exp < nowInSec) {
          try {
            const response = await axios.post<{ token?: string }>('/post/renew-timestamp-token', {
              token
            })
            const newToken: unknown = response.data.token
            if (typeof newToken === 'string') {
              this.timestampToken = newToken
            } else {
              console.warn('No valid token returned in response')
            }
          } catch (err) {
            console.error('Failed to renew token:', err)
          }
        }
      },
      async refreshHashTokenIfExpired(hash: string): Promise<void> {
        const token = this.hashTokenMap.get(hash)
        if (token === undefined) {
          throw new Error(`No token found for hash: ${hash}`)
        }

        let decoded: JwtPayload
        try {
          decoded = jwtDecode<JwtPayload>(token)
        } catch (err) {
          console.warn(`Invalid JWT for hash: ${hash}`, err)
          return
        }

        const nowInSec = Math.floor(Date.now() / 1000)
        if (typeof decoded.exp === 'number' && decoded.exp < nowInSec) {
          try {
            const response = await axios.post<{ token?: string }>('/post/renew-hash-token', {
              token
            })
            const newToken: unknown = response.data.token
            if (typeof newToken === 'string') {
              this.hashTokenMap.set(hash, newToken)
            } else {
              console.warn(`No valid token returned for hash: ${hash}`)
            }
          } catch (err) {
            console.error(`Failed to renew token for hash: ${hash}`, err)
          }
        }
      }
    }
  })()
