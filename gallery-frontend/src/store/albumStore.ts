import type { AlbumData } from '@/script/common/commonType'
import { defineStore } from 'pinia'

// This store is not used yet.
export const useAlbumStore = defineStore({
  id: 'albumStore',
  state: (): {
    albumData: AlbumData[]
  } => ({
    albumData: []
  }),
  actions: {}
})
