<template>
  <Home
    v-if="basicString !== undefined"
    isolation-id="mainId"
    :basic-string="basicString"
    :search-string="searchString"
  >
    <template #reading-bar> <ShareBar /> </template
  ></Home>
</template>
<script setup lang="ts">
import { LocationQueryValue, useRoute } from 'vue-router'
import Home from './Home.vue'
import ShareBar from '@/components/NavBar/ShareBar.vue'
import { onBeforeMount, ref, Ref } from 'vue'
import Cookies from 'js-cookie'
const route = useRoute()
const albumId: Ref<string | undefined> = ref(undefined)
const shareId: Ref<string | undefined> = ref(undefined)
const basicString: Ref<string | undefined> = ref(undefined)
const searchString = ref<LocationQueryValue | LocationQueryValue[] | undefined>(null)
onBeforeMount(() => {
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
  searchString.value = route.query.search
})
</script>
