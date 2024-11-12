<template>
  <v-card
    class="mx-auto position-fixed"
    width="300"
    variant="elevated"
    id="upload-vcard"
    retain-focus
    :style="{
      bottom: '50px',
      left: '50px',
      zIndex: 5
    }"
  >
    <v-progress-linear
      bg-color="surface-variant"
      color="primary"
      height="10"
      :model-value="uploadStore.percentComplete()"
      rounded="pill"
    ></v-progress-linear>
    <v-card-item>
      <v-row no-gutters justify="space-between">
        <v-card-title class="font-weight-black">Uploading</v-card-title>
        <v-col cols="12">
          <v-card-title
            >Remaining time:
            {{
              humanizeDuration(uploadStore.remainingTime() * 1000, {
                units: ['h', 'm', 's'],
                largest: 1,
                round: true
              })
            }}</v-card-title
          >
        </v-col>
      </v-row>
    </v-card-item>
  </v-card>
</template>
<script setup lang="ts">
/**
 * This modal is used for displaying upload information.
 */
import { useUploadStore } from '@/store/uploadStore'
import humanizeDuration from 'humanize-duration'
const uploadStore = useUploadStore('')
</script>

<style scoped></style>
