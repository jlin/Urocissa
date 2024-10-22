<template>
  <v-sheet class="position-fixed w-100 h-100" :style="{ backgroundColor: `#424242` }">
    <v-sheet
      v-if="imageContainerRef"
      ref="scrollBarRef"
      class="position-fixed"
      id="scroll-bar"
      :style="{
        width: `${scrollBarWidth}px`,
        height: `calc(100% - 100px)`,
        right: `${0}`,
        zIndex: `3`,
        cursor: `vertical-text`,
        backgroundColor: `#424242`,
        marginTop: '8px'
      }"
      @click="handleClickScroll"
      @mousedown="handleMouseDown"
      @mouseup="handleMouseUp"
      @mousemove="handleMouseMove"
      @mouseleave="handleMouseLeave"
      @touchstart="handleTouchStart"
      @touchend="handleTouchEnd"
      @touchmove="handleTouchMove"
    >
      <v-sheet class="position-relative w-100 h-100">
        <!-- Sheet to used as a visual indicator (a dash line) representing the timestamp of the currentscroll position within the view. -->
        <v-sheet
          v-if="scrollbarStore.initialized"
          id="current-date-chip"
          class="w-100 position-absolute"
          :style="{
            height: `calc(${(currentDateChipIndex / rowLength) * 100}% + 1px)`,
            borderBottom: '1px solid deepskyblue'
          }"
        ></v-sheet>
        <!-- Chips to show the all year labels. -->
        <v-chip
          v-for="scrollbarData in displayScrollbarDataArrayYear"
          :key="scrollbarData.index"
          size="x-small"
          variant="text"
          class="w-100 position-absolute text-grey-lighten-2 pa-0 ma-0 d-flex align-center justify-center"
          :style="{
            top: `${(Math.floor(scrollbarData.index / layoutBatchNumber) / rowLength) * 100}%`,
            userSelect: 'none'
          }"
        >
          {{ scrollbarData.year }}
        </v-chip>
        <!-- This sheet's height is adjusted to visually indicate the size of the current row block. -->
        <v-sheet
          v-if="scrollBarRef"
          id="block-chip"
          class="w-100 position-absolute bg-grey-darken-2"
          :style="{
            height: `${scrollbarHeight / rowLength}px`,
            top: `${(hoverLabelRowIndex / rowLength) * 100}%`,
            pointerEvents: 'none'
          }"
        >
          <!-- Chip to show the current view year and month label. -->
          <v-sheet
            id="day-chip"
            class="position-absolte w-100 d-flex align-center justify-center text-caption"
            :style="{
              backgroundColor: '#3a3a3a',
              borderTop: '1px solid deepskyblue',
              height: `25px`,
              zIndex: 2,
              userSelect: 'none'
            }"
          >
            {{ hoverLabelDate }}</v-sheet
          >
        </v-sheet>
      </v-sheet>
    </v-sheet>
  </v-sheet>
</template>

<script setup lang="ts">
import { useDataLengthStore } from '@/store/dataLengthStore'
import { ref, inject, Ref, computed, watch, watchEffect } from 'vue'
import { fixedBigRowHeight, layoutBatchNumber, ScrollbarData } from '@/script/common/commonType'
import { useScrollbarStore } from '@/store/scrollbarStore'
import { fetchRowInWorker } from '@/script/inWorker/fetchRowInWorker'
import { useRowStore } from '@/store/rowStore'
import { useOffsetStore } from '@/store/offsetStore'
import { useQueueStore } from '@/store/queueStore'
import { useLocationStore } from '@/store/locationStore'
import { debounce } from 'lodash'
import { scrollBarWidth } from '@/script/common/commonType'
import { useElementSize } from '@vueuse/core'

const isDragging = ref(false)
const locationStore = useLocationStore()
const dataLengthStore = useDataLengthStore()
const imageContainerRef = inject<Ref<HTMLElement | null>>('imageContainerRef')
const scrollTop = inject<Ref<number>>('scrollTop')
const scrollBarRef = ref<HTMLElement | null>(null)
const scrollbarStore = useScrollbarStore()
const rowStore = useRowStore()
const offsetStore = useOffsetStore()
const queueStore = useQueueStore()
const hoverLabelRowIndex = ref(0)
const rowLength = computed(() => dataLengthStore.rowLength)
const isScrolling = ref(false)
const currentDateChipIndex = ref(0)
const chipSize = 25
const singleRowChipHeight = computed(() => {
  return scrollbarHeight.value / rowLength.value
})

const rowIndexDifferenceLowerBound = computed(() => {
  return Math.ceil(chipSize / singleRowChipHeight.value)
})

const { height: scrollbarHeight } = useElementSize(imageContainerRef)

const currentBatchIndex = computed(() => {
  return Math.floor(locationStore.locationIndex! / layoutBatchNumber)
})

const hoverLabelDate = computed(() => {
  let returnedString = ''
  for (let scrollbarData of scrollbarStore.scrollbarDataArray) {
    const scrollbarDataRowIndex = Math.floor(scrollbarData.index / layoutBatchNumber)
    if (hoverLabelRowIndex.value >= scrollbarDataRowIndex) {
      returnedString = `${scrollbarData.year}.${scrollbarData.month}`
    } else {
      break
    }
  }
  return returnedString
})

const debouncedFetchRow = debounce((index: number) => {
  fetchRowInWorker(index)
}, 100)

const handleClickScroll = (event: MouseEvent | TouchEvent) => {
  const scrollbarElement = event.currentTarget
  const clientY = 'touches' in event ? event.touches[0].clientY : event.clientY

  if (scrollbarElement instanceof HTMLElement && scrollTop !== undefined) {
    const scrollbar = scrollbarElement.getBoundingClientRect()
    const clickPositionRelative = clientY - scrollbar.top // relative to the top of the scroll bar

    const targetRowIndex = clamp(
      Math.floor((rowLength.value * clickPositionRelative) / scrollbar.height),
      0,
      rowLength.value - 1
    )

    currentDateChipIndex.value = targetRowIndex
    locationStore.anchor = targetRowIndex
    offsetStore.clearAll()
    queueStore.clearAll()
    dataLengthStore.clearForResize()
    rowStore.clearForResize()
    scrollTop.value = targetRowIndex * fixedBigRowHeight
    debouncedFetchRow(targetRowIndex)
  }
}

const handleMouseMove = (event: MouseEvent) => {
  const scrollbarElement = event.currentTarget
  if (scrollbarElement instanceof HTMLElement && scrollTop !== undefined) {
    const scrollbar = scrollbarElement.getBoundingClientRect()
    const hoverPositionRelative = event.clientY - scrollbar.top // relative to the top of the scroll bar
    const targetRowIndex = clamp(
      Math.floor((rowLength.value * hoverPositionRelative) / scrollbar.height),
      0,
      rowLength.value - 1
    )

    if (targetRowIndex >= 0 && targetRowIndex <= rowLength.value - 1) {
      if (isDragging.value) {
        handleClickScroll(event)
      }
      hoverLabelRowIndex.value = targetRowIndex
    }
  }
}

const handleMouseDown = () => {
  isScrolling.value = true
  isDragging.value = true
}

const displayScrollbarDataArrayYear: Ref<ScrollbarData[]> = ref([])
watchEffect(() => {
  let array: ScrollbarData[] = []
  let lastIndex: number | null = null
  scrollbarStore.scrollbarDataArrayYear.forEach((scrollbarData) => {
    if (
      lastIndex === null ||
      (Math.floor(scrollbarData.index / layoutBatchNumber) - lastIndex >=
        rowIndexDifferenceLowerBound.value &&
        Math.floor(scrollbarData.index / layoutBatchNumber) <
          rowLength.value - rowIndexDifferenceLowerBound.value)
    ) {
      lastIndex = Math.floor(scrollbarData.index / layoutBatchNumber)
      array.push(scrollbarData)
    }
  })
  displayScrollbarDataArrayYear.value = array
})

function clamp(givenNumber: number, min: number, max: number): number {
  return Math.min(Math.max(givenNumber, min), max)
}

const handleMouseLeave = () => {
  hoverLabelRowIndex.value = currentBatchIndex.value
  isDragging.value = false
}

const handleMouseUp = () => {
  isDragging.value = false
}

const handleTouchStart = (event: TouchEvent) => {
  isScrolling.value = true
  isDragging.value = true
  handleClickScroll(event)
}

const handleTouchMove = (event: TouchEvent) => {
  const scrollbarElement = event.currentTarget
  if (scrollbarElement instanceof HTMLElement && scrollTop !== undefined) {
    const scrollbar = scrollbarElement.getBoundingClientRect()
    const hoverPositionRelative = event.touches[0].clientY - scrollbar.top // relative to the top of the scroll bar
    const targetRowIndex = clamp(
      Math.floor((rowLength.value * hoverPositionRelative) / scrollbar.height),
      0,
      rowLength.value - 1
    )
    if (targetRowIndex >= 0 && targetRowIndex <= rowLength.value) {
      if (isDragging.value) {
        handleClickScroll(event)
      }
      hoverLabelRowIndex.value = targetRowIndex
    }
  }
}

const handleTouchEnd = () => {
  isDragging.value = false
}

watch(
  () => locationStore.locationIndex,
  () => {
    isScrolling.value = true
    hoverLabelRowIndex.value = currentBatchIndex.value
    currentDateChipIndex.value = currentBatchIndex.value
  }
)
</script>
