import { editTagsInWorker } from '../inWorker/editTagsInWorker'
import { IsolationId } from './types'

export function quickAddTags(tag: string, indexList: number[], isolationId: IsolationId) {
  const indexArray = indexList
  const addTagsArray: string[] = [tag]
  const removeTagsArray: string[] = []
  editTagsInWorker(indexArray, addTagsArray, removeTagsArray, isolationId)
}

export function quickRemoveTags(tag: string, indexList: number[], isolationId: IsolationId) {
  const indexArray = indexList
  const addTagsArray: string[] = []
  const removeTagsArray: string[] = [tag]
  editTagsInWorker(indexArray, addTagsArray, removeTagsArray, isolationId)
}

export function quickEditTags(
  tag: string,
  indexListAdd: number[],
  indexListRemove: number[],
  isolationId: IsolationId
) {
  quickAddTags(tag, indexListAdd, isolationId)
  quickRemoveTags(tag, indexListRemove, isolationId)
}
