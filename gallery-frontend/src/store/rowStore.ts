import type { IsolationId, Row } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useRowStore = (isolationId: IsolationId) =>
  defineStore('rowStore' + isolationId, {
    state: (): {
      rowData: Map<number, Row> //  Map<rowIndex, Row>
      lastVisibleRow: Map<number, Row>
    } => ({
      rowData: new Map(),
      lastVisibleRow: new Map()
    }),
    actions: {
      clearAll() {
        this.rowData.clear()
        this.lastVisibleRow.clear()
      },
      clearForResize() {
        this.rowData.clear()
        this.lastVisibleRow.clear()
      }
    }
  })()
