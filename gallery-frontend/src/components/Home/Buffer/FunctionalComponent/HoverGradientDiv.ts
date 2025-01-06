import { FunctionalComponent, h } from 'vue'

interface HoverGradientDivProps {
  mobile: string | null
}

const HoverGradientDiv: FunctionalComponent<HoverGradientDivProps> = (props) => {
  if (props.mobile !== null) {
    return null
  }

  return h('div', {
    id: 'hover-gradient-div',
    class: 'position-absolute w-100 child',
    style: {
      zIndex: 3,
      height: '40px',
      background: 'linear-gradient(180deg, rgba(0,0,0,0.5) 0%, rgba(255,255,255,0) 100%)',
      pointerEvents: 'none'
    }
  })
}

HoverGradientDiv.props = {
  mobile: {
    type: [String, null],
    required: true
  }
}

export default HoverGradientDiv
