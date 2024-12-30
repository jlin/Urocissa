<template>
  <v-list-item>
    <template #prepend>
      <v-avatar>
        <v-icon color="black">mdi-image</v-icon>
      </v-avatar>
    </template>
    <v-list-item-title class="text-wrap">{{ `${props.album.itemCount} items` }}</v-list-item-title>
    <v-list-item-subtitle class="text-wrap">
      {{ filesize(props.album.itemSize) }}
    </v-list-item-subtitle>
  </v-list-item>
</template>
<script setup lang="ts">
import { filesize } from 'filesize'
import { Album } from '@/script/common/types'

const props = defineProps<{
  album: Album
}>()
</script>
