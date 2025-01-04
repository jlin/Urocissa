import { FunctionalComponent, h } from 'vue'

interface ThumbhashImageProps {
  src: string | undefined
}

const ThumbhashImage: FunctionalComponent<ThumbhashImageProps> = (props) => {
  return h('img', {
    draggable: false,
    style: {
      position: 'absolute',
      zIndex: 1
    },
    class: 'thumbhash-image w-100 h-100 bg-grey-darken-2',
    src: props.src
  })
}

ThumbhashImage.props = {
  src: {
    type: String,
    required: false // `undefined` is valid
  }
}

export default ThumbhashImage
