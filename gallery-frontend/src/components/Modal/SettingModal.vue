<template>
  <v-dialog v-model="modalStore.showSettingModal" id="setting-modal" variant="flat" rounded>
    <v-card class="mx-auto w-100" max-width="400" variant="elevated" retain-focus>
      <v-card-title>Settings</v-card-title>
      <v-card-text>
        <v-row align="center" no-gutters>
          <v-col cols="auto">
            <v-chip variant="text"> Thumbnail size </v-chip>
          </v-col>
          <v-col>
            <v-slider
              show-ticks="always"
              v-model="subRowHeightScaleValue"
              :min="250"
              :max="450"
              :step="10"
              :thumb-label="true"
              :disabled="!initializedStore.initialized"
              hide-details
              thumb-size="16"
              prepend-icon="mdi-minus"
              append-icon="mdi-plus"
              @click:prepend="onSubRowHeightScaleUpdate(-10)"
              @click:append="onSubRowHeightScaleUpdate(10)"
            ></v-slider>
          </v-col>
        </v-row>
        <v-row align="center" no-gutters class="mt-4" v-if="false">
          <v-col cols="auto">
            <v-chip variant="text"> Limit Ratio </v-chip>
          </v-col>
          <v-col>
            <v-switch
              :model-value="limitRatioValue"
              @update:model-value="onLimitRatioUpdate"
              :disabled="!initializedStore.initialized"
              hide-details
            ></v-switch>
          </v-col>
        </v-row>
        <v-row align="center" no-gutters class="mt-4" >
          <v-col cols="auto">
            <v-chip variant="text"> Theme </v-chip>
          </v-col>
          <v-col>
            <v-switch
              :model-value="themeIsLight"
              @update:model-value="onThemeUpdate"
              :disabled="!initializedStore.initialized"
              hide-details
            ></v-switch>
          </v-col>
        </v-row>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn @click="modalStore.showSettingModal = false">Close</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useModalStore } from '@/store/modalStore'
import { useInitializedStore } from '@/store/initializedStore'
import { useConstStore } from '@/store/constStore'
import { useTheme } from 'vuetify'

const modalStore = useModalStore('mainId')
const initializedStore = useInitializedStore('mainId')
const constStore = useConstStore('mainId')
const vuetifyTheme = useTheme()

// Read/write computed for subRowHeightScale (source of truth is constStore)
const subRowHeightScaleValue = computed<number>({
  get: () => constStore.subRowHeightScale,
  set: (newVal: number | null) => {
    const value = newVal ?? constStore.subRowHeightScale
    const clamped = Math.max(250, Math.min(450, Number(value)))
    constStore.updateSubRowHeightScale(clamped).catch((error: unknown) => {
      console.error('Failed to update subRowHeightScale (via setter):', error)
    })
  }
})

// Read/write computed for limitRatio (source of truth is constStore)
const limitRatioValue = computed<boolean>({
  get: () => constStore.limitRatio,
  set: (newVal: boolean | null) => {
    const value = !!newVal
    constStore.updateLimitRation(value).catch((error: unknown) => {
      console.error('Failed to update limitRatio (via setter):', error)
    })
  }
})

// computed boolean for light theme switch (read/write)
const themeIsLight = computed<boolean>({
  get: () => constStore.theme === 'light',
  set: async (newVal: boolean | null) => {
    const wantLight = !!newVal
    const newTheme = wantLight ? 'light' : 'dark'
    try {
      await constStore.updateTheme(newTheme)

      if (vuetifyTheme && typeof vuetifyTheme.change === 'function') {
        vuetifyTheme.change(newTheme)
      }
    } catch (err) {
      console.error('Failed to update theme (via setter):', err)
    }
  }
})

// Handler invoked when the slider updates its model value
const onSubRowHeightScaleUpdate = (newValue: number | null) => {
  const value = newValue ?? constStore.subRowHeightScale
  const clamped = Math.max(250, Math.min(450, value))
  constStore.updateSubRowHeightScale(clamped).catch((error: unknown) => {
    console.error('Failed to update subRowHeightScale:', error)
  })
}

const onLimitRatioUpdate = (newValue: boolean | null) => {
  const value = !!newValue
  constStore.updateLimitRation(value).catch((error: unknown) => {
    console.error('Failed to update limitRatio:', error)
  })
}

const onThemeUpdate = async (newValue: boolean | null) => {
  const wantLight = !!newValue
  const newTheme = wantLight ? 'light' : 'dark'
  try {
    await constStore.updateTheme(newTheme)

    if (vuetifyTheme && typeof vuetifyTheme.change === 'function') {
      vuetifyTheme.change(newTheme)
    }
  } catch (err) {
    console.error('Failed to update theme:', err)
  }
}
</script>
