import { useCurrentPage } from '@/script/common/functions'
import { generateJsonString } from '@/script/lexer/generateJson'
import { defineStore } from 'pinia'
import { RouteLocationNormalizedLoaded } from 'vue-router'

export const useFilterStore = (isolationId: string) =>
  defineStore({
    id: 'filterStore' + isolationId,
    state: (): {
      // Records the type of gallery area: default, favorite, archived, etc.
      basicString: string | null
      // Records the gallery search filter
      filterString: string | null
      currentPage:
        | 'default'
        | 'all'
        | 'favorite'
        | 'archived'
        | 'trashed'
        | 'albums'
        | 'album'
        | 'view'
    } => ({
      basicString: null,
      filterString: null,
      currentPage: 'default'
    }),
    actions: {
      // Generates the filter JSON string using basicString and filterString
      // This JSON info is used to send to the backend
      generateFilterJsonString(): string | null {
        if (this.basicString !== null && this.filterString === null) {
          return generateJsonString(this.basicString)
        } else if (this.basicString === null && this.filterString !== null) {
          return generateJsonString(this.filterString)
        } else if (this.basicString !== null && this.filterString !== null) {
          return generateJsonString(`and(${this.basicString},${this.filterString})`)
        } else {
          return null
        }
      },
      handleFilterString(route: RouteLocationNormalizedLoaded) {
        const searchString = route.query.search as string
        this.filterString = searchString ? searchString : null
      },
      handleBasicString(route: RouteLocationNormalizedLoaded) {
        if (route.meta.isReadPage) {
          const album_id = route.params.hash
          this.basicString = `album:${album_id}`
        } else {
          this.basicString = route.meta.basicString
        }
      }
    }
  })()
