// useFetchRows.ts
import { Ref, watch } from 'vue'
import { useInitializedStore } from '@/store/initializedStore'
import { fetchRowInWorker } from '@/script/inWorker/fetchRowInWorker'
import debounce from 'lodash/debounce'
import { usePrefetchStore } from '@/store/prefetchStore'
import { useRowStore } from '@/store/rowStore'
import { useOffsetStore } from '@/store/offsetStore'
import { useScrollTopStore } from '@/store/scrollTopStore'

/**
 * Computes the sum of offsets for rows above the given scroll position.
 *
 * @param scrollTop - The given scroll position in pixels.
 * @returns The sum of offsets for all rows above the given scroll position.
 */
function computeOffSetSumOfAboveRowsIndex(scrollTop: number) {
  const aboveRowsIndex: number[] = []
  const rowStore = useRowStore()

  for (const row of rowStore.rowData.values()) {
    if (row.topPixelAccumulated! + row.offset < scrollTop) {
      aboveRowsIndex.push(row.rowIndex)
    }
  }

  const offsetStore = useOffsetStore()
  let offsetSum = 0

  aboveRowsIndex.forEach((rowIndex) => {
    offsetSum += offsetStore.offset.get(rowIndex)!
  })

  return offsetSum
}

/**
 * Custom hook to fetch rows of data in a virtual scrolling environment based on the current scroll position.
 *
 * @param scrollTop - Reference to the current scroll position.
 * @param startHeight - Reference to the start height of the viewport.
 * @param endHeight - Reference to the end height of the viewport.
 * @param debounceTime - Time in milliseconds to debounce fetch requests (default: 50ms).
 * @param maxWait - Maximum wait time in milliseconds for debounced requests (default: 100ms).
 */
export function useFetchRows(
  startHeight: Ref<number>,
  endHeight: Ref<number>,
  debounceTime = 50,
  maxWait = 100
) {
  const initializedStore = useInitializedStore()
  const prefetchStore = usePrefetchStore()
  const scrollTopStore = useScrollTopStore()

  const debouncedFetch = debounce(
    () => {
      if (initializedStore.initialized) {
        const offSetSumOfAboveRowsIndex = computeOffSetSumOfAboveRowsIndex(scrollTopStore.scrollTop)
        const fixedHeight = 2400
        const startHeightOffseted = startHeight.value - offSetSumOfAboveRowsIndex - fixedHeight
        const endHeightOffseted = endHeight.value - offSetSumOfAboveRowsIndex + fixedHeight
        const startIndex = Math.floor(startHeightOffseted / fixedHeight)
        const endIndex = Math.ceil(endHeightOffseted / fixedHeight)

        for (let i = startIndex; i < endIndex; i++) {
          fetchRowInWorker(i)
        }

        const prependBatch = Math.floor(startHeightOffseted / fixedHeight) - 1

        fetchRowInWorker(prependBatch)

        const appendBatch = Math.ceil(endHeightOffseted / fixedHeight) + 1

        fetchRowInWorker(appendBatch)
      }
    },
    debounceTime,
    { maxWait }
  )

  watch(
    [
      () => initializedStore.initialized,
      () => scrollTopStore.scrollTop,
      () => prefetchStore.updateFetchRowTrigger
    ],
    debouncedFetch,
    { immediate: true }
  )
}
