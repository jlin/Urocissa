<template>
  <v-overlay
    :model-value="true"
    :height="'100%'"
    :width="'100%'"
    class="d-flex"
    id="view-page"
    transition="false"
    :close-on-back="false"
  >
    <Home isolation-id="tempId" :temp-mode="generateJsonString(basicString)" :title="null" />
  </v-overlay>
</template>
<script setup lang="ts">
import { Album } from '@/script/common/types'
import Home from './Home.vue'
import { generateJsonString } from '@/script/lexer/generateJson'

const props = defineProps<{
  album: Album
}>()

const basicString = `and(not(type:"album"), not(tag:"_trashed"), not(album:"${props.album.id}"))`
</script>
