<template>
  <v-dialog
    v-if="submit !== undefined"
    v-model="modalStore.showEditAlbumsModal"
    variant="flat"
    persistent
    id="edit-album-overlay"
  >
    <v-confirm-edit
      v-model="changedAlbums"
      :disabled="false"
      @save="submit"
      @cancel="modalStore.showEditAlbumsModal = false"
    >
      <template #default="{ model: proxyModel, actions }">
        <v-card class="mx-auto w-100" max-width="400" variant="elevated" retain-focus>
          <template #title>Edit&nbsp;Albums</template>

          <template #text>
            <v-form v-model="formIsValid" @submit.prevent="submit" validate-on="input">
              <v-select
                v-model="proxyModel.value"
                chips
                multiple
                :items="albumItems"
                item-title="displayName"
                item-value="albumId"
                label="Albums"
                variant="outlined"
                closable-chips
                return-object
              />
            </v-form>
          </template>

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
import { ref, computed, onMounted, toRaw } from 'vue'
import { useRoute } from 'vue-router'
import { useModalStore } from '@/store/modalStore'
import { useAlbumStore } from '@/store/albumStore'
import { editAlbumsInWorker } from '@/script/inWorker/editAlbumsInWorker'
import type { AlbumInfo } from '@type/types'
import { getHashIndexDataFromRoute, getIsolationIdByRoute } from '@utils/getter'

const formIsValid = ref(false)
const changedAlbums = ref<AlbumInfo[]>([])
const submit = ref<(() => void) | undefined>()

const route = useRoute()
const modalStore = useModalStore('mainId')
const albumStore = useAlbumStore('mainId')

const albumItems = computed<AlbumInfo[]>(() =>
  [...albumStore.albums.values()].map((a) => structuredClone(toRaw(a)))
)

onMounted(() => {
  const initSubmit = (): (() => void) | undefined => {
    const parsed = getHashIndexDataFromRoute(route)
    if (!parsed) {
      console.error('initSubmit: failed to parse route.')
      return
    }

    const { index, data } = parsed
    if (!data.database) {
      console.error("initSubmit: 'data.database' is undefined.")
      return
    }

    const defaultAlbumIds = [...data.database.album]

    const initialAlbums = defaultAlbumIds
      .map((id) => albumStore.albums.get(id))
      .filter((a): a is AlbumInfo => a !== undefined)
      .map((a) => structuredClone(toRaw(a)))
    // Temporary workaround: VConfirmEdit internally uses structuredClone,
    // which cannot clone Vue reactive proxies directly.
    // We use toRaw() + structuredClone() here to ensure plain objects.
    // This workaround is expected to be removed after Vuetify 3.9.0 fixes the issue.

    changedAlbums.value = initialAlbums

    const innerSubmit = () => {
      const selectedIds = changedAlbums.value.map((a) => a.albumId)

      const addAlbumIds = selectedIds.filter((id) => !defaultAlbumIds.includes(id))
      const removeAlbumIds = defaultAlbumIds.filter((id) => !selectedIds.includes(id))

      editAlbumsInWorker([index], addAlbumIds, removeAlbumIds, getIsolationIdByRoute(route))

      modalStore.showEditAlbumsModal = false
    }

    return innerSubmit
  }

  submit.value = initSubmit()
})
</script>
