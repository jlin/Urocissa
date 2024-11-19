import { editTagsInWorker } from "../inWorker/editTagsInWorker"

export function quickAddTags(tag: string, index: number, isolationId: string) {
    const indexArray = [index]
    const addTagsArray: string[] = [tag]
    const removeTagsArray: string[] = []
    editTagsInWorker(indexArray, addTagsArray, removeTagsArray, isolationId)
  }
  
  export function quickRemoveTags(tag: string, index: number, isolationId: string) {
    const indexArray = [index]
    const addTagsArray: string[] = []
    const removeTagsArray: string[] = [tag]
    editTagsInWorker(indexArray, addTagsArray, removeTagsArray, isolationId)
  }
  