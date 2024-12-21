import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useModalStore = (isolationId: IsolationId) =>
  defineStore('modalStore' + isolationId, {
    state: (): {
      showEditTagsModal: boolean
      showBatchEditTagsModal: boolean
      showEditAlbumsModal: boolean
      showBatchEditAlbumsModal: boolean
      showCreateAlbumsModal: boolean
      showUploadModal: boolean
      showIsolatedHomeModal: boolean
    } => ({
      showEditTagsModal: false,
      showBatchEditTagsModal: false,
      showEditAlbumsModal: false,
      showBatchEditAlbumsModal: false,
      showCreateAlbumsModal: false,
      showUploadModal: false,
      showIsolatedHomeModal: false
    }),
    actions: {}
  })()
