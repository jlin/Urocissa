<template>
  <div
    class="buffer position-relative w-100 overflow-y-hidden"
    :style="{
      height: `${Math.max(bufferHeight, prefetchStore.totalHeight)}px`
    }"
  >
    <BufferPlaceholder
      id="placeholderTop"
      v-if="visibleRows.length > 0 && !(prefetchStore.totalHeight <= windowHeight)"
      :topPixel="visibleRows[0].topPixelAccumulated! -
        scrollTop +
        bufferHeight / 3 +
        visibleRows[0].offset"
      :modifyTopPixel="true"
    />
    <div
      v-for="row in visibleRows"
      :key="`${row.start}-${prefetchStore.timestamp}`"
      class="positioned-element position-absolute w-100 row-div"
      :style="{
        position: 'absolute',
        top: `${row.topPixelAccumulated! - scrollTop + bufferHeight / 3 + row.offset}px`,
        height: `${row.rowHeight}px`
      }"
      :start="`${row.start}`"
    >
      <RowBlock :row="row" />
    </div>
    <BufferPlaceholder
      id="placeholderBottom"
      v-if="visibleRows.length > 0 && !(prefetchStore.totalHeight <= windowHeight)"
      :topPixel="visibleRows[visibleRows.length - 1].topPixelAccumulated! -
        scrollTop +
        bufferHeight / 3 +
        visibleRows[visibleRows.length - 1].offset +
        visibleRows[visibleRows.length - 1].rowHeight"
      :modifyTopPixel="false"
    />
    <BufferPlaceholder
      id="placeholderNone"
      ref="placeholderNoneRef"
      v-if="visibleRows.length === 0 && windowWidth > 0"
      :topPixel="
        ((lastRowBottom - scrollTop + windowHeight) %
          (placeholderNoneRowRefHeight + 2 * paddingPixel)) +
        bufferHeight / 3 -
        windowHeight
      "
      :modifyTopPixel="false"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * Before understanding this component, one should first understand how its parent component (image-container) works.
 * Refer to the comments in Home.vue.
 *
 * Buffer has a large height to ensure that the parent Homepage can scroll without reaching the top or bottom prematurely.
 *
 * Buffer component contains a list of RowBlocks, with BufferPlaceholders at the top (placeholderTop) and bottom (placeholderBottom) of this list.
 * The BufferPlaceholder is crucial for improving the perceived load time and smoothness of scrolling.
 * If the list of RowBlocks is empty, BufferPlaceholder (placeholderNone) will be displayed instead.
 *
 * `topPixelAccumulated` represents the top pixel position of a RowBlock.
 * `scrollTop` is used to manage user scrolling because the scrollTop of the parent (image-container) is reset for every frame.
 * `bufferHeight / 3` is used to position the RowBlock at a sufficient distance from the top of the component so that the parent Homepage can scroll up without reaching the top prematurely.
 */
import { ComponentPublicInstance, Ref, computed, inject, ref, watch } from 'vue'
import { usePrefetchStore } from '@/store/prefetchStore'
import { useFetchImgs } from '../../hook/useFetchImgs'
import { useUpdateVisibleRows } from '../../hook/useUpdateVisibleRows'
import { useFetchRows } from '../../hook/useFetchRows'
import { batchNumber, paddingPixel } from '@/script/common/commonType'
import BufferPlaceholder from '@/components/Home/Buffer/BufferPlaceholder.vue'
import RowBlock from '@/components/Home/Buffer/BufferRowBlock.vue'

const prefetchStore = usePrefetchStore()

const windowWidth = inject<Ref<number>>('windowWidth')!
const windowHeight = inject<Ref<number>>('windowHeight')!
const imageContainerRef = inject<Ref<HTMLElement>>('imageContainerRef')!
const scrollTop = inject<Ref<number>>('scrollTop')!

type BufferPlaceholderInstance = ComponentPublicInstance<{
  placeholderRowRefHeight: number
}>
const placeholderNoneRef = ref<BufferPlaceholderInstance | null>(null)
const lastRowBottom = ref(0)

const placeholderNoneRowRefHeight = computed(() =>
  placeholderNoneRef.value ? placeholderNoneRef.value.placeholderRowRefHeight : 0
)
const visibleRowsLength = computed(() => visibleRows.value.length)
const startHeight = computed(() => scrollTop.value)
const endHeight = computed(() => scrollTop.value + windowHeight.value)

const { visibleRows } = useUpdateVisibleRows(
  imageContainerRef,
  scrollTop,
  startHeight,
  endHeight,
  lastRowBottom,
  windowHeight
)
useFetchImgs(visibleRows, visibleRowsLength, batchNumber)
useFetchRows(scrollTop, startHeight, endHeight)

watch(windowWidth, () => {
  visibleRows.value = []
})

defineProps<{
  bufferHeight: number
}>()
</script>
