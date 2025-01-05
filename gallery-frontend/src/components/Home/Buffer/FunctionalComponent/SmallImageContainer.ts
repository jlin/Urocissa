import { Fragment, FunctionalComponent, h, PropType } from 'vue'
import DesktopSmallImage from './DesktopSmallImage'
import MobileSmallImage from './MobileSmallImage'
interface SmallImageContainerProps {
  mobile: string | null
  hasBorder: boolean
  src: string
  onPointerdown: (event: PointerEvent) => void
  onPointerup: (event: PointerEvent) => void
  onPointerleave: () => void
  onClick: (event: MouseEvent) => void
}

const SmallImageContainer: FunctionalComponent<SmallImageContainerProps> = (props) => {
  const chips = []
  if (props.mobile !== null) {
    chips.push(
      h(MobileSmallImage, {
        hasBorder: props.hasBorder,
        src: props.src,
        onPointerdown: props.onPointerdown,
        onPointerup: props.onPointerup,
        onPointerleave: props.onPointerleave
      })
    )
  } else {
    chips.push(
      h(DesktopSmallImage, {
        hasBorder: props.hasBorder,
        src: props.src,
        onClick: props.onClick
      })
    )
  }
  return h(Fragment, null, chips)
}

SmallImageContainer.props = {
  mobile: {
    type: [String, null] as PropType<string | null>,
    required: true
  },
  hasBorder: {
    type: Boolean,
    required: true
  },
  src: {
    type: String,
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

export default SmallImageContainer
