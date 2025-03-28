<template>
  <v-toolbar
    flat
    height="2"
    class="no-select"
    :style="{
      backgroundColor: '#212121'
    }"
  >
  </v-toolbar>
  <v-toolbar
    class="position-relative"
    :style="{
      backgroundColor: '#212121'
    }"
  >
    <v-card elevation="0" class="w-100">
      <v-card-title> test </v-card-title>
    </v-card>
    <v-spacer></v-spacer>
  </v-toolbar>
  <ProgessBar isolation-id="mainId" />
</template>
<script setup lang="ts">
import ProgessBar from '@/components/NavBar/ProgessBar.vue'
</script>
