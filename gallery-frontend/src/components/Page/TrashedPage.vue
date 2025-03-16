<template>
  <HomeMain :basic-string="basicString" />
</template>

<script setup lang="ts">
import HomeMain from '@/components/Home/Page/HomeMain.vue'
const basicString = 'and(tag:"_trashed")'
</script>
