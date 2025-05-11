<template>
  <HomeMain :basic-string="basicString" />
</template>

<script setup lang="ts">
import HomeMain from '@/components/Home/HomeMain.vue'
const basicString = 'not(tag:"_trashed")'
</script>
