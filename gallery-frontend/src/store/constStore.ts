import { IsolationId } from '@type/types'
import { defineStore } from 'pinia'
import {
  storeSubRowHeightScale,
  getSubRowHeightScale,
  storeShowInfo,
  getShowInfo
} from '@/db/settingsDb'

export const useConstStore = (isolationId: IsolationId) =>
  defineStore('constStore' + isolationId, {
    state: (): {
      disableImg: boolean
      subRowHeightScale: number
      showInfo: boolean
    } => ({
      disableImg: false,
      subRowHeightScale: 250,
      showInfo: false
    }),
    actions: {
      async updateSubRowHeightScale(value: number): Promise<void> {
        this.subRowHeightScale = value
        await storeSubRowHeightScale(value)
      },

      async loadSubRowHeightScale(): Promise<void> {
        const storedValue = await getSubRowHeightScale()
        if (storedValue !== null) {
          this.subRowHeightScale = storedValue
        }
      },

      async updateShowInfo(value: boolean): Promise<void> {
        this.showInfo = value
        await storeShowInfo(value)
      },

      async loadShowInfo(): Promise<void> {
        const stored = await getShowInfo()
        if (stored !== null) {
          this.showInfo = stored
        }
      }
    }
  })()
