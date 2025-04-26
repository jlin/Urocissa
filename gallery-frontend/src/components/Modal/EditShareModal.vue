<template>
  <v-dialog
    v-if="submit !== undefined"
    v-model="modalStore.showEditShareModal"
    id="share-modal"
    variant="flat"
    persistent
    rounded
  >
    <v-confirm-edit
      v-model="shareModel"
      :disabled="false"
      @save="submit"
      @cancel="modalStore.showEditShareModal = false"
    >
      <template #default="{ model: proxyModel, actions }">
        <v-card
          class="h-100 mx-auto w-100"
          max-width="400"
          variant="elevated"
          retain-focus
          rounded="xl"
        >
          <v-toolbar color="transparent">
            <v-toolbar-title class="text-h5">Share</v-toolbar-title>
            <template #append>
              <v-btn icon="mdi-close" @click="modalStore.showEditShareModal = false" />
            </template>
          </v-toolbar>

          <v-divider />

          <v-list class="px-6" density="compact">
            <v-list-item>
              <v-textarea
                v-model="proxyModel.value.description"
                label="Description of this link"
                hide-details="auto"
              />
            </v-list-item>

            <v-list-item density="compact">
              <template #prepend>
                <v-list-item-action start>
                  <v-switch
                    v-model="proxyModel.value.showDownload"
                    color="primary"
                    :label="`Allow public user to download`"
                    hide-details
                  />
                </v-list-item-action>
              </template>
            </v-list-item>

            <v-list-item v-if="false" density="compact">
              <template #prepend>
                <v-list-item-action start>
                  <v-switch
                    v-model="proxyModel.value.showMetadata"
                    color="primary"
                    :label="`Show metadata`"
                    hide-details
                  />
                </v-list-item-action>
              </template>
            </v-list-item>

            <v-list-item v-if="false" density="compact">
              <template #prepend>
                <v-list-item-action start>
                  <v-switch
                    v-model="proxyModel.value.showUpload"
                    color="primary"
                    :label="`Allow public user to upload`"
                    hide-details
                  />
                </v-list-item-action>
              </template>
            </v-list-item>
          </v-list>

          <v-divider />

          <template #actions>
            <v-spacer />
            <component :is="actions" />
          </template>
        </v-card>
      </template>
    </v-confirm-edit>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'
import { useModalStore } from '@/store/modalStore'
import type { EditShareData } from '@/type/types'

const props = defineProps<{ editShareData: EditShareData }>()

interface ShareModel {
  url: string
  description: string
  showDownload: boolean
  showUpload: boolean
  showMetadata: boolean
  exp: number | null
}

const modalStore = useModalStore('mainId')

const shareModel = ref<ShareModel>({
  url: props.editShareData.share.url,
  description: props.editShareData.share.description,
  showDownload: props.editShareData.share.showDownload,
  showUpload: props.editShareData.share.showUpload,
  showMetadata: props.editShareData.share.showMetadata,
  exp: props.editShareData.share.exp
})

const submit = ref<(() => void) | undefined>()

onMounted(() => {
  /* 以 void 包裝 Promise → 消除 @typescript-eslint/no-misused-promises */
  submit.value = () => {
    void axios
      .put('/put/edit_share', {
        albumId: props.editShareData.albumId,
        share: shareModel.value
      })
      .then(() => {
        modalStore.showEditShareModal = false
      })
      .catch((e: unknown) => {
        console.error('Failed to update share', e)
      })
  }
})
</script>
