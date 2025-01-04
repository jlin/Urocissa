<template>
  <div class="w-100 d-flex flex-wrap row-flex-container no-select">
    <div
      v-for="(data, subIndex) in row.displayElements"
      :key="`${row.start}-${subIndex}-${prefetchStore.timestamp}`"
      :style="{
        width: `${data.displayWidth}px`,
        height: `${data.displayHeight}px`
      }"
      class="ma-1"
    >
      <v-hover>
        <template #default="{ isHovering: imgIsHovering, props: hoverProps }">
          <div
            class="position-relative w-100 h-100"
            :style="{
              border:
                collectionStore.editModeOn &&
                collectionStore.editModeCollection.has(row.start + subIndex)
                  ? '4px solid #81D4FA'
                  : ''
            }"
            v-bind="hoverProps"
          >
            <div v-if="subIndex < timeInterval" class="delay-show w-100 h-100 position-absolute">
              <DesktopHoverIcon
                class="icon-hover"
                v-if="!mobile"
                v-show="imgIsHovering"
                :on-click="(event) => handleClickIcon(event, row.start + subIndex)"
              />
              <v-chip
                id="processing-chip"
                prepend-icon="mdi-alert-circle-outline"
                density="comfortable"
                size="small"
                v-if="dataStore.data.get(row.start + subIndex)?.database?.pending"
                color="grey"
                variant="flat"
                class="position-absolute ma-2"
                :style="{
                  top: '0px',
                  right: '0px',
                  zIndex: 4
                }"
              >
                {{ 'Processing' }}
              </v-chip>
              <v-chip
                id="duration-chip"
                density="comfortable"
                size="small"
                v-if="
                  dataStore.data.has(row.start + subIndex) &&
                  dataStore.data.get(row.start + subIndex)?.database?.ext_type === 'video'
                "
                color="grey"
                variant="flat"
                class="position-absolute ma-2"
                :style="{
                  bottom: '0px',
                  right: '0px',
                  zIndex: 4
                }"
              >
                {{
                  formatDuration(
                    dataStore.data.get(row.start + subIndex)?.database?.exif_vec.duration!
                  )
                }}
              </v-chip>
              <v-chip
                id="album-chip"
                density="comfortable"
                size="small"
                v-if="
                  dataStore.data.has(row.start + subIndex) &&
                  dataStore.data.get(row.start + subIndex)?.album
                "
                color="black"
                variant="flat"
                class="position-absolute ma-2"
                :style="{
                  bottom: '0px',
                  right: '0px',
                  zIndex: 4
                }"
              >
                <span
                  class="text-truncate"
                  :style="{
                    maxWidth: `${(data.displayWidth - 8) * 0.75}px`
                  }"
                >
                  {{ dataStore.data.get(row.start + subIndex)?.album?.title ?? 'Untitled' }}
                </span>
              </v-chip>
              <div
                id="hover-gradient-div"
                v-if="!mobile"
                v-show="imgIsHovering"
                class="position-absolute w-100"
                :style="{
                  zIndex: 3,
                  height: `40px`,
                  background: `linear-gradient(180deg, rgba(0,0,0,0.5) 0%, rgba(255,255,255,0) 100%)`,
                  pointerEvents: 'none'
                }"
              ></div>
              <MobileSmallImage
                v-if="
                  mobile &&
                  !configStore.disableImg &&
                  dataStore.data.has(row.start + subIndex) &&
                  checkAndFetch(row.start + subIndex, data.displayWidth, data.displayHeight) &&
                  imgStore.imgUrl.has(row.start + subIndex)
                "
                :has-border="dataStore.data.get(row.start + subIndex)?.album !== undefined"
                :src="imgStore.imgUrl.get(row.start + subIndex)!"
                :on-pointerdown="(event: PointerEvent) => handlePointerdown(event, row.start + subIndex)"
                :on-pointerup="(event: PointerEvent) => handlePointerUp(event, row.start + subIndex)"
                :on-pointerleave="handlePointerLeave"
              />
              <DesktopSmallImage
                v-if="
                  !mobile &&
                  !configStore.disableImg &&
                  dataStore.data.has(row.start + subIndex) &&
                  checkAndFetch(row.start + subIndex, data.displayWidth, data.displayHeight) &&
                  imgStore.imgUrl.has(row.start + subIndex)
                "
                :has-border="dataStore.data.get(row.start + subIndex)?.album !== undefined"
                :src="imgStore.imgUrl.get(row.start + subIndex)!"
                :on-click="(event: MouseEvent) => handleClick(event, row.start + subIndex)"
              />

              <transition name="slide-fade" appear>
                <img
                  id="thumbhash-image"
                  draggable="false"
                  v-if="dataStore.data.has(row.start + subIndex) && !configStore.disableImg && dataStore.data.get(row.start + subIndex)!.database"
                  :key="row.start + subIndex"
                  :style="{
                    position: 'absolute',
                    zIndex: 1
                  }"
                  class="w-100 h-100 bg-grey-darken-2"
                  :src="dataStore.data.get(row.start + subIndex)!.database?.thumbhashUrl"
                />
              </transition>
            </div>
            <div
              id="grey-background-placeholder"
              :style="{
                position: 'absolute',
                zIndex: 0
              }"
              @click="($event) => handleClick($event, row.start + subIndex)"
              class="w-100 h-100 bg-grey-darken-2"
            ></div>
          </div>
        </template>
      </v-hover>
    </div>
  </div>
</template>

<script setup lang="ts">
import { layoutBatchNumber } from '@/script/common/constants'
import { IsolationId, Row } from '@/script/common/types'
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import { useDataStore } from '@/store/dataStore'
import { useImgStore } from '@/store/imgStore'
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useHandleClick } from '@/components/hook/useHandleClick'
import { useRouter, useRoute } from 'vue-router'
import { useConfigStore } from '@/store/configStore'
import { useQueueStore } from '@/store/queueStore'
import { useWorkerStore } from '@/store/workerStore'
import DesktopSmallImage from './FunctionalComponent/DesktopSmallImage'
import DesktopHoverIcon from './FunctionalComponent/DesktopHoverIcon'
import {
  getArrayValue,
  getCookiesJwt,
  getInjectValue,
  getMapValue
} from '@/script/common/functions'
import { useScrollTopStore } from '@/store/scrollTopStore'

const props = defineProps<{
  row: Row
  isolationId: IsolationId
}>()

const router = useRouter()
const route = useRoute()
const mobile = getInjectValue('mobile')
const prefetchStore = usePrefetchStore(props.isolationId)
const collectionStore = useCollectionStore(props.isolationId)
const dataStore = useDataStore(props.isolationId)
const imgStore = useImgStore(props.isolationId)
const configStore = useConfigStore(props.isolationId)
const queueStore = useQueueStore(props.isolationId)
const workerStore = useWorkerStore(props.isolationId)
const scorllTopStore = useScrollTopStore(props.isolationId)
const timeInterval = ref(0)
const isLongPress = ref(false)
const pressTimer = ref<number | null>(null) // 定時器 ID
const scrollingTimer = ref<number | null>(null)

const { handleClick } = useHandleClick(router, route, props.isolationId)

const handleClickIcon = (event: MouseEvent, currentIndex: number) => {
  if (!collectionStore.editModeOn) {
    collectionStore.editModeOn = true
    collectionStore.addApi(currentIndex)
    collectionStore.lastClick = currentIndex
  } else {
    handleClick(event, currentIndex)
  }
}

const handlePointerdown = (event: MouseEvent, currentIndex: number) => {
  if (isScrolling.value) {
    return
  }
  isLongPress.value = false // 初始為非長按
  pressTimer.value = window.setTimeout(() => {
    isLongPress.value = true // 設置為長按
    handleLongPressClick(event, currentIndex) // 觸發長按事件
  }, 600) // 長按持續時間 (例如 800 毫秒)
}

const handlePointerUp = (event: MouseEvent, currentIndex: number) => {
  if (isScrolling.value) {
    return
  }
  if (pressTimer.value !== null) {
    clearTimeout(pressTimer.value) // 清除定時器
    pressTimer.value = null
  }
  if (!isLongPress.value) {
    handleClick(event, currentIndex) // 若非長按則觸發點擊事件
  }
}

const handlePointerLeave = () => {
  if (pressTimer.value !== null) {
    clearTimeout(pressTimer.value) // 取消長按事件
    pressTimer.value = null
  }
}

const handleLongPressClick = (event: MouseEvent, currentIndex: number) => {
  if (!collectionStore.editModeOn) {
    collectionStore.editModeOn = true
    collectionStore.addApi(currentIndex)
    collectionStore.lastClick = currentIndex
  } else {
    handleClick(event, currentIndex)
  }
}

const checkAndFetch = (index: number, displayWidth: number, displayHeight: number): boolean => {
  if (imgStore.imgUrl.has(index)) {
    return true
  } else if (!queueStore.img.has(index)) {
    queueStore.img.add(index)
    const workerIndex = index % workerStore.concurrencyNumber

    if (workerStore.postToWorkerList !== undefined) {
      const data = getMapValue(dataStore.data, index) // always succeed by v-if
      if (data.database) {
        getArrayValue(workerStore.postToWorkerList, workerIndex).processSmallImage({
          index: index,
          hash: data.database.hash,
          width: displayWidth,
          height: displayHeight,
          devicePixelRatio: window.devicePixelRatio,
          jwt: getCookiesJwt()
        })
      } else if (data.album?.cover !== null && data.album?.cover !== undefined) {
        getArrayValue(workerStore.postToWorkerList, workerIndex).processSmallImage({
          index: index,
          hash: data.album.cover,
          width: displayWidth,
          height: displayHeight,
          devicePixelRatio: window.devicePixelRatio,
          jwt: getCookiesJwt(),
          albumMode: true
        })
      }
    } else {
      console.error('workerStore.postToWorkerList is undefined')
    }
    return false
  } else {
    return false
  }
}

function formatDuration(durationString: string) {
  // Convert the duration string to a number and truncate to the integer part
  const durationInSeconds = Math.floor(parseFloat(durationString))

  // Calculate hours, minutes, and seconds
  const hours = Math.floor(durationInSeconds / 3600)
  const minutes = Math.floor((durationInSeconds % 3600) / 60)
  const seconds = durationInSeconds % 60

  // Determine the formatted duration based on the presence of hours, minutes, and seconds
  let formattedDuration = ''
  if (hours > 0) {
    formattedDuration = `${hours}:${minutes.toString().padStart(2, '0')}:${seconds
      .toString()
      .padStart(2, '0')}`
  } else {
    formattedDuration = `${minutes.toString().padStart(2, '0')}:${seconds
      .toString()
      .padStart(2, '0')}`
  }

  return formattedDuration
}

onMounted(() => {
  const intervalId = setInterval(() => {
    // this part is crutial: if we do not delay the show of img, the scrub will lag if the img already loading
    if (timeInterval.value < layoutBatchNumber) {
      timeInterval.value += layoutBatchNumber
    } else {
      clearInterval(intervalId)
    }
  }, 0)
})

onBeforeUnmount(() => {
  for (let abortIndex = props.row.start; abortIndex <= props.row.end; abortIndex++) {
    const workerIndex = abortIndex % workerStore.concurrencyNumber
    if (workerStore.postToWorkerList !== undefined) {
      getArrayValue(workerStore.postToWorkerList, workerIndex).processAbort({
        index: abortIndex
      })
    } else {
      console.error('workerStore.postToWorkerList is undefined')
    }
    queueStore.img.delete(abortIndex)
  }
})

const isScrolling = ref(false)

// Prevent accidental touches while scrolling
watch(
  () => scorllTopStore.scrollTop,
  () => {
    isScrolling.value = true

    if (scrollingTimer.value !== null) {
      clearTimeout(scrollingTimer.value)
    }

    scrollingTimer.value = window.setTimeout(() => {
      isScrolling.value = false

      scrollingTimer.value = null
    }, 100)
  }
)
</script>
<style scoped>
.no-select {
  user-select: none;
}
.no-select * {
  user-select: none;
}
.icon-hover {
  color: var(--v-grey-lighten-5); /* Default color */
  transition: color 0.3s;
  cursor: pointer;
}

.icon-hover:hover {
  color: var(--v-white); /* Hover color */
}
</style>
