import { useCurrentPage } from '@/script/common/functions'
import { generateJsonString } from '@/script/lexer/generateJson'
import { defineStore } from 'pinia'
import { RouteLocationNormalizedLoaded } from 'vue-router'

export const useFilterStore = (isolationId: string = '') =>
  defineStore({
    id: 'filterStore' + isolationId,
    state: (): {
      // Records the type of gallery area: default, favorite, archived, etc.
      basicString: string | null
      // Records the gallery search filter
      filterString: string | null
      currentPage: 'default' | 'all' | 'favorite' | 'archived' | 'trashed' | 'albums' | 'album'
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
        const currentPage = useCurrentPage(route).value
        switch (currentPage) {
          case 'default': {
            this.basicString = 'and(not(tag: _archived), not(tag:_trashed))'
            break
          }
          case 'favorite': {
            this.basicString = 'and(tag:_favorite, not(tag:_trashed))'
            break
          }
          case 'archived': {
            this.basicString = 'and(tag:_archived, not(tag:_trashed))'
            break
          }
          case 'albums': {
            this.basicString = 'type:album'
            break
          }
          case 'album': {
            // Access the 'id' parameter from the route
            const id = route.params.id
            this.basicString = `album:${id}`
            break
          }
          case 'trashed': {
            this.basicString = 'and(tag:_trashed)'
            break
          }
          case 'all': {
            this.basicString = null
            break
          }
          default: {
            this.basicString = null
            break
          }
        }
      }
    }
  })()
