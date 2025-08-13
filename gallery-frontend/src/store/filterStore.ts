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
        const hasBasicString = typeof basicString === 'string'
        const searchStringStr = typeof this.searchString === 'string' ? this.searchString : null
        const hasSearchString = searchStringStr !== null

        try {
          // No valid input
          if (!hasBasicString && !hasSearchString) {
            return null
          }

          // Only basicString
          if (hasBasicString && !hasSearchString) {
            return generateJsonString(basicString)
          }

          // Only searchString
          if (!hasBasicString && hasSearchString) {
            try {
              return generateJsonString(searchStringStr)
            } catch {
              return generateJsonString(`any: "${searchStringStr}"`)
            }
          }

          // Both strings
          try {
            return generateJsonString(`and(${basicString},${searchStringStr})`)
          } catch {
            return generateJsonString(`and(${basicString}, any: "${searchStringStr}")`)
          }
        } catch (err) {
          console.error(err)
          return null
        }
      }
    }
  })()
