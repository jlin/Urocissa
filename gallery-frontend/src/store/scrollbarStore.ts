import { ScrollbarData } from '@/script/common/commonType'
import { defineStore } from 'pinia'
export const useScrollbarStore = defineStore({
  id: 'scrollbarStore',
  state: (): {
    scrollbarDataArray: ScrollbarData[]
    scrollbarDataArrayYear: ScrollbarData[]
    initialized: boolean
  } => ({
    scrollbarDataArray: [],
    scrollbarDataArrayYear: [],
    initialized: false
  }),
  actions: {
    initialize(scrollbarDataArray: ScrollbarData[]) {
      this.scrollbarDataArray = scrollbarDataArray
      let year: number | null = null
      this.scrollbarDataArray.forEach((scrollbarData) => {
        if (year !== scrollbarData.year) {
          year = scrollbarData.year
          this.scrollbarDataArrayYear.push(scrollbarData)
        }
      })

      this.initialized = true
    }
  }
})
