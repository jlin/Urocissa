import { defineStore } from 'pinia'
import { useMessageStore } from './messageStore'
import axios from 'axios'

export const useUploadStore = (isolationId: string) =>
  defineStore('uploadStore' + isolationId, {
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
        if (this.total !== undefined && this.loaded !== undefined) {
          return Math.floor((this.loaded / this.total) * 100)
        }
        return 0
      },
      elapsedTime() {
        if (this.startTime !== undefined) {
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
      },
      triggerFileInput(): void {
        if (this.uploadButton) {
          this.uploadButton.click()
        }
      },
      async handleFileUpload(event: Event): Promise<void> {
        const messageStore = useMessageStore('mainId')
        const target = event.target as HTMLInputElement
        const files = target.files
        if (!files || files.length === 0) return

        const formData = new FormData()
        let totalSize = 0

        Array.from(files).forEach((file, i) => {
          formData.append(`lastModified${i}`, `${file.lastModified}`)
          formData.append(`file${i}`, file)
          totalSize += file.size
        })

        console.log(`Total upload size: ${totalSize} bytes`)

        try {
          const startTime = Date.now()

          this.total = 0
          this.loaded = 0
          this.startTime = startTime
          this.uploading = true

          await axios.post('/upload', formData, {
            headers: {
              'Content-Type': 'multipart/form-data'
            },
            onUploadProgress: (progressEvent) => {
              if (progressEvent.total !== undefined) {
                this.total = progressEvent.total
                this.loaded = progressEvent.loaded
                this.startTime = startTime

                console.log(`Upload is ${this.percentComplete()}% complete`)
                console.log(`Remaining time: ${this.remainingTime()} seconds`)
              }
            }
          })

          messageStore.message = 'Files uploaded successfully!'
          messageStore.warn = false
          messageStore.showMessage = true
        } catch (error) {
          console.error('There was an error uploading the files: ', error)

          if (error instanceof Error) {
            messageStore.message = `There was an error uploading the files: ${error.message}`
          } else {
            messageStore.message = `There was an error uploading the files: ${String(error)}`
          }

          messageStore.warn = true
          messageStore.showMessage = true
        }
      }
    }
  })()
