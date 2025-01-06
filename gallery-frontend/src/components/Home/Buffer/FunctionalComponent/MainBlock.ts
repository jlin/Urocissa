import { DisplayElement, IsolationId } from '@/script/common/types'
import { useConfigStore } from '@/store/configStore'
import { useDataStore } from '@/store/dataStore'
import { useImgStore } from '@/store/imgStore'
import { useQueueStore } from '@/store/queueStore'
import { FunctionalComponent, h, PropType, Transition } from 'vue'
import ChipsContainer from './ChipsContainer'
import SmallImageContainer from './SmallImageContainer'
import HoverGradientDiv from './HoverGradientDiv'

interface MainBlockProps {
  index: number
  displayElement: DisplayElement
  isolationId: IsolationId
  mobile: string | null
  onPointerdown: (event: PointerEvent) => void
  onPointerup: (event: PointerEvent) => void
  onPointerleave: () => void
  onClick: (event: MouseEvent) => void
}

const MainBlock: FunctionalComponent<MainBlockProps> = (props) => {
  const configStore = useConfigStore(props.isolationId)
  const imgStore = useImgStore(props.isolationId)
  const queueStore = useQueueStore(props.isolationId)
  const dataStore = useDataStore(props.isolationId)

  const abstractData = dataStore.data.get(props.index)

  if (!abstractData) {
    return null
  }

  const chips = []
  chips.push(
    h(ChipsContainer, {
      abstractData: abstractData,
      displayElement: props.displayElement
    })
  )
  chips.push(
    h(HoverGradientDiv, {
      mobile: props.mobile
    })
  )
  chips.push(
    h(SmallImageContainer, {
      index: props.index,
      displayElement: props.displayElement,
      isolationId: props.isolationId,
      mobile: props.mobile,
      onPointerdown: props.onPointerdown,
      onPointerup: props.onPointerup,
      onPointerleave: props.onPointerleave,
      onClick: props.onClick
    })
  )
  return h(
    'div',
    {
      class: 'w-100 h-100 position-absolute'
    },
    chips
  )
}

MainBlock.props = {
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

export default MainBlock
