import { IsolationId } from '@type/types'
import { generateJsonString } from '@/script/lexer/generateJson'
import { defineStore } from 'pinia'
import { RouteLocationNormalizedLoaded } from 'vue-router'

export const useFilterStore = (isolationId: IsolationId) =>
  defineStore('filterStore' + isolationId, {
    state: (): {
      // Records the gallery search filter
      searchString: string | null
    } => ({
      searchString: null
    }),
    actions: {
      // Generates the filter JSON string using basicString and searchString
      // This JSON info is used to send to the backend
      generateFilterJsonString(basicString: string | null): string | null {
        try {
          if (basicString !== null && this.searchString === null) {
            return generateJsonString(basicString)
          } else if (basicString === null && this.searchString !== null) {
            try {
              return generateJsonString(this.searchString)
            } catch {
              return generateJsonString(`any: "${this.searchString}"`)
            }
          } else if (basicString !== null && this.searchString !== null) {
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
      },
      recordSearchString(route: RouteLocationNormalizedLoaded) {
        const searchString = route.query.search as string
        this.searchString = searchString ? searchString : null
      }
    }
  })()
