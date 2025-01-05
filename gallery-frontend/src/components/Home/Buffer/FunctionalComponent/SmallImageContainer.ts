import { Fragment, FunctionalComponent, h, PropType } from 'vue'
import DesktopSmallImage from './DesktopSmallImage'
import MobileSmallImage from './MobileSmallImage'
import { AbstractData, DisplayElement, IsolationId } from '@/script/common/types'
import { useImgStore } from '@/store/imgStore'
import { useQueueStore } from '@/store/queueStore'
import { useWorkerStore } from '@/store/workerStore'
import { getArrayValue, getCookiesJwt } from '@/script/common/functions'
import { useConfigStore } from '@/store/configStore'

interface SmallImageContainerProps {
  abstractData?: AbstractData
  index: number
  displayElement: DisplayElement
  isolationId: IsolationId
  mobile: string | null
  hasBorder: boolean
  onPointerdown: (event: PointerEvent) => void
  onPointerup: (event: PointerEvent) => void
  onPointerleave: () => void
  onClick: (event: MouseEvent) => void
}

const SmallImageContainer: FunctionalComponent<SmallImageContainerProps> = (props) => {
  const imgStore = useImgStore(props.isolationId)
  if (!props.abstractData) {
    return null
  }
  if (!showSmallImage(props.abstractData, props.index, props.displayElement, props.isolationId)) {
    return null
  }
  const src = imgStore.imgUrl.get(props.index)

  if (src !== undefined) {
    const chips = []
    if (props.mobile !== null) {
      chips.push(
        h(MobileSmallImage, {
          hasBorder: props.hasBorder,
          src: src,
          onPointerdown: props.onPointerdown,
          onPointerup: props.onPointerup,
          onPointerleave: props.onPointerleave
        })
      )
    } else {
      chips.push(
        h(DesktopSmallImage, {
          hasBorder: props.hasBorder,
          src: src,
          onClick: props.onClick
        })
      )
    }
    return h(Fragment, null, chips)
  } else {
    return null
  }
}

SmallImageContainer.props = {
  abstractData: {
    type: Object as PropType<AbstractData>,
    required: false
  },
  displayElement: {
    type: Object as PropType<DisplayElement>,
    required: true
  },
  isolationId: {
    type: String as PropType<IsolationId>,
    required: true
  },
  index: {
    type: Number,
    required: true
  },
  mobile: {
    type: [String, null] as PropType<string | null>,
    required: true
  },
  hasBorder: {
    type: Boolean,
    required: true
  },
  onPointerdown: {
    type: Function as PropType<(event: PointerEvent) => void>,
    required: true
  },
  onPointerup: {
    type: Function as PropType<(event: PointerEvent) => void>,
    required: true
  },
  onPointerleave: {
    type: Function as PropType<() => void>,
    required: true
  },
  onClick: {
    type: Function as PropType<(event: MouseEvent) => void>,
    required: true
  }
}

function showSmallImage(
  abstractData: AbstractData,
  index: number,
  displayElement: DisplayElement,
  isolationId: IsolationId
): boolean {
  const configStore = useConfigStore(isolationId)
  const imgStore = useImgStore(isolationId)
  return (
    !configStore.disableImg &&
    checkAndFetch(
      abstractData,
      index,
      displayElement.displayWidth,
      displayElement.displayHeight,
      isolationId
    ) &&
    imgStore.imgUrl.has(index)
  )
}

function checkAndFetch(
  abstractData: AbstractData,
  index: number,
  displayWidth: number,
  displayHeight: number,
  isolationId: IsolationId
): boolean {
  const imgStore = useImgStore(isolationId)
  const queueStore = useQueueStore(isolationId)
  const workerStore = useWorkerStore(isolationId)

  if (imgStore.imgUrl.has(index)) {
    return true
  } else if (!queueStore.img.has(index)) {
    queueStore.img.add(index)
    const workerIndex = index % workerStore.concurrencyNumber

    if (workerStore.postToWorkerList !== undefined) {
      if (abstractData.database) {
        getArrayValue(workerStore.postToWorkerList, workerIndex).processSmallImage({
          index: index,
          hash: abstractData.database.hash,
          width: displayWidth,
          height: displayHeight,
          devicePixelRatio: window.devicePixelRatio,
          jwt: getCookiesJwt()
        })
      } else if (abstractData.album?.cover !== null && abstractData.album?.cover !== undefined) {
        getArrayValue(workerStore.postToWorkerList, workerIndex).processSmallImage({
          index: index,
          hash: abstractData.album.cover,
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

export default SmallImageContainer
