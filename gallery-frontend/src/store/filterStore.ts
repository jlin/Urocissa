import { IsolationId } from '@type/types'
import { generateJsonString } from '@/script/lexer/generateJson'
import { defineStore } from 'pinia'
import { LocationQueryValue } from 'vue-router'

export const useFilterStore = (isolationId: IsolationId) =>
  defineStore('filterStore' + isolationId, {
    state: (): {
      // Records the gallery search filter
      searchString: LocationQueryValue | LocationQueryValue[] | undefined
    } => ({
      searchString: null
    }),
    actions: {
      // Generates the filter JSON string using basicString and searchString
      // This JSON info is used to send to the backend
      generateFilterJsonString(basicString: string | null): string | null {
        try {
          if (typeof basicString === 'string' && typeof this.searchString !== 'string') {
            return generateJsonString(basicString)
          } else if (typeof basicString !== 'string' && typeof this.searchString === 'string') {
            try {
              return generateJsonString(this.searchString)
            } catch {
              return generateJsonString(`any: "${this.searchString}"`)
            }
          } else if (typeof basicString === 'string' && typeof this.searchString === 'string') {
            try {
              return generateJsonString(`and(${basicString},${this.searchString})`)
            } catch {
              return generateJsonString(`and(${basicString}, any: "${this.searchString}")`)
            }
          } else {
            return null
          }
        } catch (err) {
          console.error(err)
          return null
        }
      }
    }
  })()
