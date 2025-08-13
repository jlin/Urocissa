import { IsolationId } from '@type/types'
import { defineStore } from 'pinia'
import {
  storeSubRowHeightScale,
  getSubRowHeightScale,
  storeShowInfo,
  getShowInfo,
  // 新增：
  storeConcurrencyNumber,
  getConcurrencyNumber
} from '@/db/settingsDb'

export const useConstStore = (isolationId: IsolationId) =>
  defineStore('constStore' + isolationId, {
    state: (): {
      subRowHeightScale: number
      showInfo: boolean
      concurrencyNumber: number
    } => ({
      subRowHeightScale: 250,
      showInfo: false,
      concurrencyNumber: Math.max(Math.floor(navigator.hardwareConcurrency / 2), 1)
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
      },

      // ===== 新增：concurrencyNumber =====
      async updateConcurrencyNumber(value: number): Promise<void> {
        // 保障為 >= 1 的整數
        const v = Math.max(1, Math.floor(value))
        this.concurrencyNumber = v
        await storeConcurrencyNumber(v)
      },

      async loadConcurrencyNumber(): Promise<void> {
        const stored = await getConcurrencyNumber()
        if (typeof stored === 'number' && Number.isFinite(stored) && stored > 0) {
          this.concurrencyNumber = Math.floor(stored)
        }
      }
    }
  })()
