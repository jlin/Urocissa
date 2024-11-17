import { prefetchInWorker } from '@/script/inWorker/prefetchInWorker'
import { RouteLocationNormalizedLoadedGeneric } from 'vue-router'
import { watchDebounced } from '@vueuse/core'
import { Ref } from 'vue'

export function prefetch(
  filterJsonString: string | null,
  windowWidth: Ref<number>,
  route: RouteLocationNormalizedLoadedGeneric,
  isolationId: string
) {
  const stopWatcher = watchDebounced(
    windowWidth,
    () => {
      if (windowWidth.value > 0) {
        const priorityId = route.query.priorityId as string
        const reverse = route.query.reverse as string
        let locate: string | null = null

        if (route.meta.isViewPage) {
          locate = route.params.hash as string
        } else {
          const queryLocate = route.query.locate
          if (typeof queryLocate === 'string') {
            locate = queryLocate
          }
        }

        prefetchInWorker(filterJsonString, priorityId, reverse, locate, isolationId)

        stopWatcher() // Stop the watcher after prefetching
      }
    },
    { immediate: true, debounce: 75, maxWait: 1000 }
  )
}
