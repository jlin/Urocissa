<template>
  <div class="swiper-container h-100 w-100">
    <swiper
      :modules="modules"
      :slides-per-view="1"
      :space-between="10"
      :centered-slides="true"
      :initial-slide="currentSlideIndex"
      :resistance="true"
      :resistance-ratio="0.3"
      :allow-touch-move="canHandleNav()"
      @slide-change="onSlideChange"
      @swiper="onSwiper"
      class="h-100"
    >
      <swiper-slide v-if="previousHash !== undefined">
        <div class="slide-content">
          <ViewPageDisplayDatabase
            v-if="previousAbstractData && previousAbstractData.database && !configStore.disableImg"
            :index="index - 1"
            :hash="previousAbstractData.database.hash"
            :abstract-data="previousAbstractData"
            :col-width="colWidth ?? 0"
            :col-height="colHeight ?? 0"
            :isolation-id="isolationId"
          />
          <ViewPageDisplayAlbum
            v-if="previousAbstractData && previousAbstractData.album && !configStore.disableImg"
            :index="index - 1"
            :album="previousAbstractData.album"
            :col-width="colWidth ?? 0"
            :col-height="colHeight ?? 0"
          />
        </div>
      </swiper-slide>

      <swiper-slide>
        <div class="slide-content">
          <ViewPageDisplayDatabase
            v-if="abstractData && abstractData.database && !configStore.disableImg"
            :index="index"
            :hash="hash"
            :abstract-data="abstractData"
            :col-width="colWidth ?? 0"
            :col-height="colHeight ?? 0"
            :isolation-id="isolationId"
          />
          <ViewPageDisplayAlbum
            v-if="abstractData && abstractData.album && !configStore.disableImg"
            :index="index"
            :album="abstractData.album"
            :col-width="colWidth ?? 0"
            :col-height="colHeight ?? 0"
          />
        </div>
      </swiper-slide>

      <swiper-slide v-if="nextHash !== undefined">
        <div class="slide-content">
          <ViewPageDisplayDatabase
            v-if="nextAbstractData && nextAbstractData.database && !configStore.disableImg"
            :index="index + 1"
            :hash="nextAbstractData.database.hash"
            :abstract-data="nextAbstractData"
            :col-width="colWidth ?? 0"
            :col-height="colHeight ?? 0"
            :isolation-id="isolationId"
          />
          <ViewPageDisplayAlbum
            v-if="nextAbstractData && nextAbstractData.album && !configStore.disableImg"
            :index="index + 1"
            :album="nextAbstractData.album"
            :col-width="colWidth ?? 0"
            :col-height="colHeight ?? 0"
          />
        </div>
      </swiper-slide>
    </swiper>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDataStore } from '@/store/dataStore'
import ViewPageDisplayDatabase from './DisplayDatabase.vue'
import ViewPageDisplayAlbum from './DisplayAlbum.vue'
import { useConfigStore } from '@/store/configStore'
import { useModalStore } from '@/store/modalStore'
import { Manipulation } from 'swiper/modules'
import 'swiper/css'
import 'swiper/css/manipulation'
import type { Swiper as SwiperType } from 'swiper'
import { AbstractData, IsolationId } from '@type/types'
import { Swiper, SwiperSlide } from 'swiper/vue'
const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  abstractData: AbstractData | undefined
  colWidth: number | undefined
  colHeight: number | undefined
  previousHash: string | undefined
  nextHash: string | undefined
  previousPage: Record<string, unknown> | undefined
  nextPage: Record<string, unknown> | undefined
}>()

const configStore = useConfigStore(props.isolationId)
const dataStore = useDataStore(props.isolationId)
const route = useRoute()
const router = useRouter()

const modules = [Manipulation]
const swiperInstance = ref<SwiperType | null>(null)

const nextAbstractData = computed(() => dataStore.data.get(props.index + 1))
const previousAbstractData = computed(() => dataStore.data.get(props.index - 1))

const currentSlideIndex = computed(() => (props.previousHash !== undefined ? 1 : 0))

function canHandleNav(): boolean {
  const modalStore = useModalStore('mainId')
  return (
    configStore.isMobile &&
    ((route.meta.level === 2 && props.isolationId === 'mainId') ||
      (route.meta.level === 4 && props.isolationId === 'subId')) &&
    !modalStore.showEditTagsModal
  )
}

function onSwiper(swiper: SwiperType) {
  swiperInstance.value = swiper
}

function onSlideChange(swiper: SwiperType) {
  if (!canHandleNav()) return

  const currentIndex = swiper.activeIndex
  const hasPrevious = props.previousHash !== undefined
  const hasNext = props.nextHash !== undefined

  if (hasPrevious) {
    if (currentIndex === 0 && props.previousPage) {
      router.replace(props.previousPage).catch((error: unknown) => {
        console.error('Navigation Error:', error)
      })
    } else if (currentIndex === 2 && hasNext && props.nextPage) {
      router.replace(props.nextPage).catch((error: unknown) => {
        console.error('Navigation Error:', error)
      })
    }
  } else {
    if (currentIndex === 1 && hasNext && props.nextPage) {
      router.replace(props.nextPage).catch((error: unknown) => {
        console.error('Navigation Error:', error)
      })
    }
  }
}

// next/previous page are provided via props from the parent

watch(
  () => props.index,
  () => {
    if (swiperInstance.value && configStore.isMobile) {
      swiperInstance.value.slideTo(currentSlideIndex.value, 0)
    }
  }
)
</script>

<style scoped>
.swiper-container {
  width: 100%;
  overflow: hidden;
}

.slide-content {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

:deep(.swiper) {
  width: 100%;
  height: 100%;
}

:deep(.swiper-slide) {
  text-align: center;
  font-size: 18px;
  background: transparent;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
