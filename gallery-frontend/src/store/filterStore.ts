import { generateJsonString } from '@/script/lexer/generateJson'
import { defineStore } from 'pinia'
import { computed } from 'vue'
import { RouteLocationNormalizedLoaded } from 'vue-router'

export const useFilterStore = defineStore({
  id: 'filterStore',
  state: (): {
    // Records the type of gallery area: default, favorite, archived, etc.
    basicString: string | null
    // Records the gallery search filter
    filterString: string | null
  } => ({
    basicString: null,
    filterString: null
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
      const currentPage = computed(() => {
        if (route.path.startsWith('/favorite')) {
          return 'favorite'
        } else if (route.path.startsWith('/archived')) {
          return 'archived'
        } else if (route.path.startsWith('/trashed')) {
          return 'trashed'
        } else if (route.path.startsWith('/all')) {
          return 'all'
        } else {
          return 'default'
        }
      })

      switch (currentPage.value) {
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
        case 'trashed': {
          this.basicString = 'tag:_trashed'
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
})
