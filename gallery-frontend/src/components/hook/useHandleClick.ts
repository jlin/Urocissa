import { Router, RouteLocationNormalizedLoaded } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { useDataStore } from '@/store/dataStore'

export function useHandleClick(router: Router, route: RouteLocationNormalizedLoaded) {
  const handleClick = async (event: MouseEvent, currentIndex: number) => {
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
      const abstractData = dataStore.data.get(currentIndex)!

      const hashOrId = abstractData.database ? abstractData.database.hash : abstractData.album!.id

      if (route.path.startsWith('/favorite')) {
        router.push({ path: `/favorite/view/${hashOrId}`, query: { ...route.query } })
      } else if (route.path.startsWith('/archived')) {
        router.push({ path: `/archived/view/${hashOrId}`, query: { ...route.query } })
      } else if (route.path.startsWith('/all')) {
        router.push({ path: `/all/view/${hashOrId}`, query: { ...route.query } })
      } else if (route.path.startsWith('/trashed')) {
        router.push({ path: `/trashed/view/${hashOrId}`, query: { ...route.query } })
      } else if (route.path.startsWith('/album-')) {
        // Extract the album identifier from the path
        const segments = route.path.split('/')
        const albumIdentifier = segments.find((segment) => segment.startsWith('album-'))
        if (albumIdentifier) {
          router.push({ path: `/${albumIdentifier}/view/${hashOrId}`, query: { ...route.query } })
        } else {
          // Fallback if album identifier is not found
          router.push({ path: `/view/${hashOrId}`, query: { ...route.query } })
        }
      } else {
        router.push({ path: `/view/${hashOrId}`, query: { ...route.query } })
      }
    }
    if (collectionStore.editModeCollection.size === 0) {
      collectionStore.editModeOn = false
    }
  }

  return { handleClick }
}
