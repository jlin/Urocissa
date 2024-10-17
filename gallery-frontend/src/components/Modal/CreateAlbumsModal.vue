<template>
  <v-dialog v-model="modalStore.showCreateAlbumsModal" variant='flat' persistent id="edit-tag-overlay">
    <v-card class="mx-auto pa-2" width="400" variant="elevated">
      <v-card-title class="text-h5 "> Create Albums </v-card-title>
      <v-container>
        <v-text-field v-model="albumName" :rules="[rules.required]" item-text="label" item-value="value"
          label="Album Name"></v-text-field>
        <v-text-field v-model="password" item-text="label" item-value="value"
          label="Password (Optional)"></v-text-field>
      </v-container>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="grey-lighten-2" variant="text" class="ma-2 button button-submit"
          @click="modalStore.showCreateAlbumsModal = false">
          Cancel
        </v-btn>
        <v-btn color="teal-accent-4" variant="outlined" class="ma-2 button button-submit" @click="createAlbum()">
          Submit
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { useModalStore } from '@/store/modalStore';
import { ref } from 'vue';
const albumName = ref<string>('')
const modalStore = useModalStore()
const password = ref<string>('')

const rules = {
  required: (value: string) => !!value || 'Album Name is required',
}

const createAlbum = async () => {
  let albumId = generateAlbumId(); // Generate the albumId on first run
  let attempt = 0; // Keep track of attempts in case we need to regenerate the albumId

  while (attempt <= 5) { // Limit the number of attempts to avoid infinite loops
    const response = await fetch('/create_album', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        albumId: albumId,
        albumName: albumName.value,
        password: password.value,
      })
    });

    if (response.status === 409) {
      console.warn(`album id ${albumId} already exists; regenerating id ...`);
      albumId = generateAlbumId(); // Regenerate the albumId
      attempt++; // Increment the attempt counter
      if (attempt < 5) {
        continue; // Skip the rest of the loop and try again
      } else {
        console.warn(`album id ${albumId} already exists; regenerating failed`);
        modalStore.showCreateAlbumsModal = false
        return;
      }
    }

    switch (response.status) {
      case 400: {
        console.warn("Failed to decode UTF8 string");
        modalStore.showCreateAlbumsModal = false
        return;
      }
      case 422: {
        console.warn("Failed to parse JSON");
        modalStore.showCreateAlbumsModal = false
        return;
      }
      case 201: {
        console.log("create album successfully");
        modalStore.showCreateAlbumsModal = false
        return;
      }
      default: {
        console.warn(`unknown error code: ${response.status}`);
        modalStore.showCreateAlbumsModal = false
        return;
      }
    }
  }
};

const generateAlbumId = () => {
  const characters = 'abcdefghijklmnopqrstuvwxyz0123456789';
  let result = '';
  const charactersLength = characters.length;
  for (let i = 0; i < 64; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
};
</script>

<style scoped></style>