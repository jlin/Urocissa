<template>
  <v-btn icon="mdi mdi-arrow-left" :to="leaveViewPage(route)"></v-btn>
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router'

import { leaveViewPage } from '@/script/navigator'

const route = useRoute()
</script>
