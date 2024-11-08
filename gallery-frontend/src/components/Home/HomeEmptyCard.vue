<template>
  <v-container class="d-flex align-center justify-center" fluid>
    <v-hover v-slot="{ isHovering, props }">
      <v-card
        class="pa-4 text-center mx-auto hover-cursor"
        :style="{
          border: isHovering ? '2px solid #BDBDBD' : '2px solid transparent'
        }"
        :elevation="isHovering ? 12 : 2"
        max-width="600"
        rounded="lg"
        width="100%"
        v-bind="props"
      >
        <v-icon class="mb-5" color="grey" size="100"> mdi-image-plus </v-icon>
        <v-card-item>
          <v-card-subtitle>
            Wow, so empty!<br />
            Try adding some photos here!
          </v-card-subtitle>
        </v-card-item>
      </v-card>
    </v-hover>
  </v-container>
</template>
<script setup lang="ts"></script>
<style scoped>
.hover-cursor {
  cursor: pointer !important;
}
</style>
