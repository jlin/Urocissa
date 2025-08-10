import { useRerenderStore } from '@/store/rerenderStore'
import { Router } from 'vue-router'

export async function navigateToAlbum(albumId: string, router: Router) {
  const albumPath = `/albums/view/${albumId}/read` // Adjust the path as necessary
  if (router.currentRoute.value.fullPath.startsWith('/albums')) {
    const rerenderStore = useRerenderStore('mainId')
    rerenderStore.rerenderHome()
  }
  await router.push({ path: albumPath })
}
