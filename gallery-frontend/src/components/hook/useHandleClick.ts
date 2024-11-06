import { Router, RouteLocationNormalizedLoaded } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { useDataStore } from '@/store/dataStore'

export function useHandleClick(router: Router, route: RouteLocationNormalizedLoaded) {
  const handleClick = (event: MouseEvent, currentIndex: number) => {
    const collectionStore = useCollectionStore()
    if (collectionStore.editModeOn) {
      if (event.shiftKey && collectionStore.lastClick !== null) {
        const start = Math.min(collectionStore.lastClick, currentIndex)
        const end = Math.max(collectionStore.lastClick, currentIndex)
        let allInCollection = true

        for (let i = start; i <= end; i++) {
          if (!collectionStore.editModeCollection.has(i)) {
            allInCollection = false
            break
          }
        }
        if (allInCollection) {
          for (let i = start; i <= end; i++) {
            collectionStore.deleteApi(i)
          }
          collectionStore.lastClick = null
        } else {
          for (let i = start; i <= end; i++) {
            collectionStore.addApi(i)
          }
          collectionStore.lastClick = currentIndex
        }
      } else {
        if (collectionStore.editModeCollection.has(currentIndex)) {
          collectionStore.deleteApi(currentIndex)
          collectionStore.lastClick = null
        } else {
          collectionStore.addApi(currentIndex)
          collectionStore.lastClick = currentIndex
        }
      }
    } else {
      // collectionStore.editModeOn === false
      const dataStore = useDataStore()
      const hash = dataStore.data.get(currentIndex)!.database!.hash

      if (route.path.startsWith('/favorite')) {
        router.push({ path: '/favorite/view/' + hash, query: { ...route.query } })
      } else if (route.path.startsWith('/archived')) {
        router.push({ path: '/archived/view/' + hash, query: { ...route.query } })
      } else if (route.path.startsWith('/all')) {
        router.push({ path: '/all/view/' + hash, query: { ...route.query } })
      } else if (route.path.startsWith('/trashed')) {
        router.push({ path: '/trashed/view/' + hash, query: { ...route.query } })
      } else {
        router.push({ path: '/view/' + hash, query: { ...route.query } })
      }
    }
    if (collectionStore.editModeCollection.size === 0) {
      collectionStore.editModeOn = false
    }
  }

  return { handleClick }
}
