import 'axios'

declare module 'axios' {
  export interface AxiosRequestConfig {
    timestampToken?: string
  }
}
