import { fixedBigRowHeight, layoutBatchNumber } from '@/script/common/commonType'
import { defineStore } from 'pinia'

export const useDataLengthStore = defineStore({
  id: 'dataLengthStore',
  state: (): {
    windowWidth: number
    timestamp: string | null
    totalHeight: number
    totalHeightOriginal: number
    dataLength: number
    rowLength: number
    locateTo: number | null
    updateVisibleRowTrigger: boolean
    updateFetchRowTrigger: boolean
  } => ({
    windowWidth: 0,
    timestamp: null,
    totalHeight: 0,
    totalHeightOriginal: 0,
    dataLength: 0,
    rowLength: 0,
    locateTo: null,
    updateVisibleRowTrigger: false,
    updateFetchRowTrigger: false
  }),
  actions: {
    clearAll() {
      this.timestamp = null
      this.totalHeight = 0
      this.totalHeightOriginal = 0
      this.dataLength = 0
      this.locateTo = null
      this.updateVisibleRowTrigger = !this.updateVisibleRowTrigger
    },
    clearForResize() {
      this.totalHeight = Math.ceil(this.dataLength / layoutBatchNumber) * fixedBigRowHeight
      this.totalHeightOriginal = this.totalHeight
      this.updateVisibleRowTrigger = !this.updateVisibleRowTrigger
    }
  }
})
