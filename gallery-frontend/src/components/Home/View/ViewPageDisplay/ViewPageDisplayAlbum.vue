<template>
  <router-view></router-view>
  <v-col v-if="metadata && metadata.album" class="h-100 d-flex align-center justify-center">
    <v-row>
      <v-col
        :class="[
          'd-flex',
          'align-center',
          'justify-center',
          colWidth < colHeight ? 'flex-column' : 'flex-row'
        ]"
      >
        <img
          v-if="imgStore.imgOriginal.get(index)"
          id="album-img"
          :key="index"
          rounded="xl"
          aspect-ratio="1"
          cover
          :src="imgStore.imgOriginal.get(index)"
          :style="{
            width: `${Math.round(
              Math.max(Math.min(colHeight, colWidth / 2), Math.min(colWidth, colHeight / 2))
            )}px`,
            height: `${Math.round(
              Math.max(Math.min(colHeight, colWidth / 2), Math.min(colWidth, colHeight / 2))
            )}px`,
            maxWidth: '500px',
            maxHeight: '500px',
            objectFit: 'cover',
            border: '8px solid white'
          }"
        />
        <v-card
          v-if="metadata && metadata.album"
          :style="{
            width: `${Math.round(
              Math.max(Math.min(colHeight, colWidth / 2), Math.min(colWidth, colHeight / 2))
            )}px`,
            height: `${Math.round(
              Math.max(Math.min(colHeight, colWidth / 2), Math.min(colWidth, colHeight / 2))
            )}px`,
            maxWidth: '500px',
            maxHeight: '500px'
          }"
          outlined
          style="padding: 16px"
          class="d-flex flex-column"
        >
          <v-card-item>
            <v-card-title class="text-h4">
              {{ metadata.album.title }}
            </v-card-title>
          </v-card-item>
          <v-divider></v-divider>
          <v-list>
            <v-list-item>
              <v-list-item-title v-if="metadata.album.startTime">
                {{ `${dater(metadata.album.startTime)} ~ ${dater(metadata.album.endTime!)}` }}
              </v-list-item-title>
              <v-list-item-subtitle>
                {{ `${metadata.album.itemCount} item${metadata.album.itemCount === 1 ? '' : 's'}` }}
                •
                {{ filesize(metadata.album.itemSize) }}
              </v-list-item-subtitle>
            </v-list-item>
          </v-list>

          <!-- Use this div to take up remaining space -->
          <div class="flex-grow-1"></div>

          <v-card-actions class="justify-end">
            <v-btn
              color="teal-accent-4"
              variant="flat"
              class="ma-2 button button-submit"
              :to="`${route.fullPath}/isolated`"
            >
              test enter
            </v-btn>
            <v-btn
              color="teal-accent-4"
              variant="flat"
              class="ma-2 button button-submit"
              :to="`/album-${metadata.album!.id}`"
              @click="
                () => {
                  albumStore.leaveAlbumPath = route.fullPath
                }
              "
            >
              Enter Album
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
    <v-card
      v-if="metadata?.database?.pending"
      class="d-flex align-center justify-start"
      outlined
      style="padding: 16px"
    >
      <v-row align="center" no-gutters>
        <v-col cols="auto" class="d-flex align-center">
          <v-icon size="48" color="warning">mdi-alert-circle-outline</v-icon>
        </v-col>
        <v-col class="text-left pl-4">
          <div>This video is currently being processed.</div>
          <div>Please check back later.</div>
        </v-col>
      </v-row>
    </v-card>
  </v-col>
</template>

<script setup lang="ts">
import { VCol } from 'vuetify/components'
import { useImgStore } from '@/store/imgStore'

import { AbstractData } from '@/script/common/types'
import { filesize } from 'filesize'
import { useAlbumStore } from '@/store/albumStore'
import { useRoute } from 'vue-router'
const route = useRoute()
const albumStore = useAlbumStore()

function dater(timestamp: number): string {
  const locale = navigator.language
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  }).format(timestamp)
}

defineProps<{
  index: number
  metadata: AbstractData
  colWidth: number
  colHeight: number
}>()

const imgStore = useImgStore()
</script>
