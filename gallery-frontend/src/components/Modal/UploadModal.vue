<template>
  <!--   <v-card
    append-icon="mdi-open-in-new"
    class="mx-auto"
    href="https://github.com/vuetifyjs/vuetify/"
    max-width="344"
    prepend-icon="mdi-github"
    rel="noopener"
    subtitle="Check out the official repository"
    target="_blank"
    title="Vuetify on GitHub"
  ></v-card> -->
  <v-card
    class="mx-auto position-fixed text-white"
    append-icon=""
    :title="`${uploadStore.status}`"
    :subtitle="`${humanizeDuration(uploadStore.remainingTime() * 1000, {
      units: ['h', 'm', 's'],
      largest: 1,
      round: true
    })} remaining`"
    variant="elevated"
    id="upload-vcard"
    retain-focus
    :style="{
      bottom: '50px',
      left: '50px',
      zIndex: 5
    }"
  >
    <template #prepend>
      <v-progress-circular
        color="primary"
        :model-value="uploadStore.percentComplete()"
        class="ma-4"
      >
        <v-icon color="white" icon="mdi-cloud-upload" />
      </v-progress-circular>
    </template>
    <template #append>
      <v-btn
        variant="outlined"
        class="ma-4"
        @click="modalStore.showUploadModal = false"
        v-if="uploadStore.status === 'Completed'"
      >
        {{ 'Close' }}
      </v-btn>
      <v-btn
        v-else
        variant="outlined"
        class="ma-4"
        @click="uploadStore.cancelUpload()"
        :color="uploadStore.status === 'Canceled' ? 'red-lighten-4' : 'blue-lighten-4'"
      >
        {{ `${uploadStore.status === 'Canceled' ? 'Canceled' : 'Cancel'}` }}
      </v-btn>
    </template>
  </v-card>
</template>
<script setup lang="ts">
/**
 * This modal is used for displaying upload information.
 */
import { useModalStore } from '@/store/modalStore'
import { useUploadStore } from '@/store/uploadStore'
import humanizeDuration from 'humanize-duration'
const uploadStore = useUploadStore('mainId')
const modalStore = useModalStore('mainId')
</script>

<style scoped></style>
