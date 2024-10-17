import { TagInfo, tagInfoSchema } from '@/script/common/commonType'
import axios from 'axios'
import { defineStore } from 'pinia'
import { z, ZodError } from 'zod'

export const useTagStore = defineStore({
  id: 'tagStore',
  state: (): {
    tags: TagInfo[]
    fetched: boolean
  } => ({
    tags: [],
    fetched: false
  }),
  actions: {
    async fetchTags() {
      try {
        const response = await axios.get('/get/get-tags')

        if (response.status !== 200) {
          throw new Error('Network response was not ok')
        }

        const tagsArraySchema = z.array(tagInfoSchema)
        const tags = tagsArraySchema.parse(response.data)

        this.tags = tags
        this.tags.sort((a, b) => a.tag.localeCompare(b.tag))
        this.fetched = true
      } catch (error) {
        if (error instanceof ZodError) {
          console.error('Validation errors:', error.errors)
        } else {
          console.error('Failed to fetch tags:', error)
        }
      }
    },
    applyTags(tagsJson: { tag: string; number: number }[]) {
      this.tags = tagsJson
      this.tags.sort((a, b) => a.tag.localeCompare(b.tag))
      this.fetched = true
    }
  }
})
