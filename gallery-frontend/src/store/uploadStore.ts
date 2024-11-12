import { defineStore } from 'pinia'

export const useUploadStore = (isolationId: string = '') =>
  defineStore({
    id: 'uploadStore' + isolationId,
    state: (): {
      uploading: boolean
      total: number | undefined
      loaded: number | undefined
      startTime: number | undefined
      uploadButton: HTMLInputElement | null
    } => ({
      uploading: false,
      total: undefined,
      loaded: undefined,
      startTime: undefined,
      uploadButton: null
    }),
    actions: {
      percentComplete() {
        if (this.total && this.loaded !== undefined) {
          return Math.floor((this.loaded / this.total) * 100)
        }
        return 0
      },
      elapsedTime() {
        if (this.startTime) {
          return (Date.now() - this.startTime) / 1000 // time in seconds
        }
        return 0
      },
      uploadSpeed() {
        const elapsed = this.elapsedTime()
        if (elapsed > 0 && this.loaded !== undefined) {
          return this.loaded / elapsed // speed in MB/s
        }
        return 0
      },
      remainingTime() {
        const speed = this.uploadSpeed()
        if (speed > 0 && this.total !== undefined && this.loaded !== undefined) {
          return (this.total - this.loaded) / speed // time in seconds
        }
        return 0
      }
    }
  })()
