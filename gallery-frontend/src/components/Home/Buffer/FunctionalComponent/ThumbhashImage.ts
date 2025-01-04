import { FunctionalComponent, h } from 'vue'

interface ThumbhashImageProps {
  key: string | number
  src: string | undefined
}

const ThumbhashImage: FunctionalComponent<ThumbhashImageProps> = (props) => {
  return h('img', {
    id: 'thumbhash-image',
    draggable: false,
    key: props.key,
    style: {
      position: 'absolute',
      zIndex: 1
    },
    class: 'w-100 h-100 bg-grey-darken-2',
    src: props.src
  })
}

// Define props
ThumbhashImage.props = {
  key: {
    type: [String, Number],
    required: true
  },
  src: {
    type: String,
    required: false // `undefined` is valid
  }
}

export default ThumbhashImage
