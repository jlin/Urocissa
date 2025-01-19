<template>
  <div class="text-center ma-2">
    <v-snackbar
      v-model="messageStore.showMessage"
      :color="messageStore.warn ? 'red-darken-1' : 'teal-darken-1'"
      timer="2000"
      :style="{
        zIndex: 3000
      }"
    >
      {{ messageStore.message }}
      <template #actions> </template>
    </v-snackbar>
  </div>
</template>

<script setup lang="ts">
import { useMessageStore } from '@/store/messageStore'
const messageStore = useMessageStore('mainId')
</script>
