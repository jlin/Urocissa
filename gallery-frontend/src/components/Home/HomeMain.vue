<template>
  <Home isolation-id="mainId" :basic-string="props.basicString" :search-string="searchString">
    <template #home-toolbar>
      <NavBar />
    </template>
  </Home>
</template>
<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import Home from './Home.vue'
import NavBar from '@/components/NavBar/NavBar.vue'
import { LocationQueryValue, useRoute } from 'vue-router'
const route = useRoute()
const searchString = ref<LocationQueryValue | LocationQueryValue[] | undefined>(null)
const props = defineProps<{
  basicString: string | null
}>()

onBeforeMount(() => {
  searchString.value = route.query.search
})
</script>
