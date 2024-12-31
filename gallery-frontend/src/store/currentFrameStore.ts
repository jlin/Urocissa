import { IsolationId } from '@/script/common/types'
import { defineStore } from 'pinia'

export const useCurrentFrameStore = (isolationId: IsolationId) =>
  defineStore('currentFrameStore' + isolationId, {
    state: (): {
      video: HTMLVideoElement | null // unit: second
    } => ({
      video: null
    }),
    actions: {
      getCapture() {
        if (this.video) {
          const canvas = document.createElement('canvas')
          canvas.width = this.video.videoWidth
          canvas.height = this.video.videoHeight
          const context = canvas.getContext('2d')
          if (context) {
            // Draw the current video frame onto the canvas
            context.drawImage(this.video, 0, 0, canvas.width, canvas.height)

            // Get the image data as a data URL
            const capturedImage = canvas.toDataURL('image/png')

            // Create a download link
            const link = document.createElement('a')
            link.href = capturedImage
            link.download = 'captured-frame.png' // Set the desired file name

            // Simulate a click to trigger the download
            document.body.appendChild(link)
            link.click()
            document.body.removeChild(link)
          }
        }
      }
    }
  })()
