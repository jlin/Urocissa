/* eslint-disable @typescript-eslint/return-await */
import axios, { AxiosInstance, AxiosResponse } from 'axios'
import { tokenReturnSchema } from '@/script/common/schemas'
import { postToMain } from './toDataWorker'
import { getToken } from '@/indexedDb/token'
import { interceptorData } from './interceptorData'

const subAxios = axios.create()
interceptorData(subAxios)

export function interceptorImg(axiosInstance: AxiosInstance): void {
  axiosInstance.interceptors.response.use(
    (response: AxiosResponse) => response,
    async (error) => {
      if (!axios.isAxiosError(error)) {
        console.error('Unexpected error:', error)
        postToMain.notification({ message: 'An unexpected error occurred', messageType: 'warn' })
        console.error(error)
        return Promise.reject(error instanceof Error ? error : new Error(String(error)))
      }

      const { config, response } = error

      if (response?.status === 401) {
        const requestUrl = config?.url

        // Check if the request URL matches any of the specified endpoints
        if (requestUrl === undefined) {
          postToMain.unauthorized()
          return
        }

        if (requestUrl.startsWith('/object')) {
          try {
            const expiredToken = new URLSearchParams(requestUrl.split('?')[1]).get('token')

            if (expiredToken == null) {
              throw new Error('No hash token found in query parameters')
            }

            const timestampToken = await getToken()
            if (timestampToken === null) {
              throw new Error('No timestampToken found in query parameters')
            }

            const tokenResponse = await subAxios.post(
              `/post/renew-hash-token`,
              {
                expiredHashToken: expiredToken
              },
              {
                headers: {
                  Authorization: `Bearer ${timestampToken}`
                }
              }
            )

            if (tokenResponse.status === 200) {
              const newToken = tokenReturnSchema.parse(tokenResponse.data)
              const newUrl = requestUrl.replace(`token=${expiredToken}`, `token=${newToken.token}`)
              if (config) {
                config.url = newUrl
                return axiosInstance.request(config)
              }
            }
          } catch (err) {
            if (!(err instanceof Error && err.name === 'CanceledError')) {
              console.error('Token renewal failed:', err)
            }
          }
        } else {
          postToMain.unauthorized()
        }
        postToMain.notification({ message: 'Unauthorized. Please log in.', messageType: 'warn' })
      } else if (response) {
        postToMain.notification({ message: 'An error occurred', messageType: 'warn' })
      } else {
        postToMain.notification({ message: 'No response from server', messageType: 'warn' })
      }

      if (!(error instanceof Error && error.name === 'CanceledError')) {
        console.error('Token renewal failed:', error)
      }
      return Promise.reject(error)
    }
  )
}
