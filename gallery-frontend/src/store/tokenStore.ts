import { IsolationId, TokenResponse } from '@type/types'
import { jwtDecode } from 'jwt-decode'
import { defineStore } from 'pinia'
import axios from 'axios'
import { TokenResponseSchema } from '@/type/schemas'
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
            const response = await axios.post('/post/renew-timestamp-token', { token: token })
            const parsed: TokenResponse = TokenResponseSchema.parse(response.data)
            this.timestampToken = parsed.token
          } catch (err) {
            console.error('Failed to renew timestamp token:', err)
          }
        }
      },

      async refreshHashTokenIfExpired(hash: string): Promise<void> {
        const expiredToken = this.hashTokenMap.get(hash)
        if (expiredToken === undefined) {
          throw new Error(`No token found for hash: ${hash}`)
        }

        let decoded: JwtPayload
        try {
          decoded = jwtDecode<JwtPayload>(expiredToken)
        } catch (err) {
          console.warn(`Invalid JWT for hash: ${hash}`, err)
          return
        }

        const nowInSec = Math.floor(Date.now() / 1000)
        if (typeof decoded.exp === 'number' && decoded.exp < nowInSec) {
          try {
            const timestampToken = this.timestampToken
            if (timestampToken == null) {
              throw new Error('Missing timestampToken for authorization')
            }

            const response = await axios.post(
              '/post/renew-hash-token',
              {
                expiredHashToken: expiredToken
              },
              {
                headers: {
                  Authorization: `Bearer ${timestampToken}`
                }
              }
            )

            const parsed: TokenResponse = TokenResponseSchema.parse(response.data)
            this.hashTokenMap.set(hash, parsed.token)
          } catch (err) {
            console.error(`Failed to renew token for hash: ${hash}`, err)
          }
        }
      }
    }
  })()
