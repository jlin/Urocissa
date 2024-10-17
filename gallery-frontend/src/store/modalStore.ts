import { defineStore } from 'pinia'

export const useModalStore = defineStore({
  id: 'modalStore',
  state: (): {
    showEditTagsModal: boolean
    showBatchEditTagsModal: boolean
    showEditAlbumsModal: boolean
    showCreateAlbumsModal: boolean
    showUploadModal: boolean
  } => ({
    showEditTagsModal: false,
    showBatchEditTagsModal: false,
    showEditAlbumsModal: false,
    showCreateAlbumsModal: false,
    showUploadModal: true
  }),
  actions: {}
})
