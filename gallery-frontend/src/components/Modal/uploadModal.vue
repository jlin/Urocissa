<template>
  <v-card
    class="mx-auto position-fixed"
    max-width="300"
    max-height="200"
    variant="elevated"
    id="upload-vcard"
    retain-focus
    :style="{
      bottom: '50px',
      left: '50px',
      zIndex: 5
    }"
  >
    <v-card-text class="text-medium-emphasis pa-6">
      <div class="text-h6 mb-6" v-if="uploadStore.total">Uploading</div>

      <div class="text-h4 font-weight-black mb-4">{{ uploadStore.percentComplete() }}%</div>

      <v-progress-linear
        bg-color="surface-variant"
        class="mb-6"
        color="primary"
        height="10"
        :model-value="uploadStore.percentComplete()"
        rounded="pill"
      ></v-progress-linear>

      <div v-if="uploadStore.loaded">
        {{ filesize(uploadStore.loaded) }} uploaded, remaining time:{{
          uploadStore.remainingTime()
        }}
        s
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
/**
 * This modal is used for displaying upload information.
 */
import { useUploadStore } from '@/store/uploadStore'
import { filesize } from 'filesize'
const uploadStore = useUploadStore()
</script>

<style scoped></style>
