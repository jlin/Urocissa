<template>
  <Home v-if="basicString !== undefined" isolation-id="mainId" :basic-string="basicString">
    <template #reading-bar> <InfoBar /> </template
  ></Home>
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router'
import Home from './Home.vue'
import InfoBar from '@/components/NavBar/InfoBar.vue'
import { onMounted, ref, Ref } from 'vue'
import Cookies from 'js-cookie'
const route = useRoute()
const albumId: Ref<string | undefined> = ref(undefined)
const shareId: Ref<string | undefined> = ref(undefined)
const basicString: Ref<string | undefined> = ref(undefined)

onMounted(() => {
  const albumIdOpt = route.params.albumId
  const shareIdOpt = route.params.shareId
  if (typeof albumIdOpt === 'string' && typeof shareIdOpt === 'string') {
    albumId.value = albumIdOpt
    shareId.value = shareIdOpt
    Cookies.set('albumId', albumIdOpt)
    Cookies.set('shareId', shareIdOpt)
    basicString.value = `and(not(tag:"_trashed"), album:"${albumIdOpt}")`
  } else {
    console.error(`(albumId, shareId) is (${albumId.value}, ${shareId.value})`)
  }
})
</script>
