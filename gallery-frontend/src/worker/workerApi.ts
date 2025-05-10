// workerApi.ts

// ================== Imports ==================
import { FetchDataMethod, MessageColor, RowWithOffset, SlicedData, TagInfo } from '@type/types'
import { createActionCreators } from 'typesafe-agent-events'

// ================== Payload Types ==================

// === To ImgWorker Payloads ===
export interface ProcessSmallImagePayload {
  index: number
  hash: string
  width: number
  height: number
  devicePixelRatio: number
  albumMode?: boolean
  albumId: null | string
  shareId: null | string
}

export interface ProcessImagePayload {
  index: number
  hash: string
  devicePixelRatio: number
  albumId: null | string
  shareId: null | string
}

export interface ProcessAbortPayload {
  index: number
}

// === From ImgWorker Payloads ===
export interface SmallImageProcessedPayload {
  index: number
  url: string
}

export interface ImageProcessedPayload {
  index: number
  url: string
}

export interface NotificationReturnParams {
  text: string
  color: MessageColor
}

// === To DataWorker Payloads ===
export interface FetchDataParams {
  fetchMethod: FetchDataMethod
  batch: number
  timestamp: number
  timestampToken: string
}

export interface FetchRowParams {
  index: number
  timestamp: number
  windowWidth: number
  isLastRow: boolean
  timestampToken: string
}

export interface EditTagsParams {
  indexSet: Set<number>
  addTagsArray: string[]
  removeTagsArray: string[]
  timestamp: number
}

export interface EditAlbumsParams {
  indexSet: Set<number>
  addAlbumsArray: string[]
  removeAlbumsArray: string[]
  timestamp: number
}

export interface DeleteDataParams {
  indexArray: number[]
  timestamp: number
}

// === From DataWorker Payloads ===
export interface ReturnDataParams {
  batch: number
  slicedDataArray: SlicedData[]
}

export interface FetchRowReturnParams {
  rowWithOffset: RowWithOffset
  timestamp: number
}

export interface EditTagsReturnParams {
  returnedTagsArray: TagInfo[] | undefined
}

// ================== Worker Action Creators ==================

export const toImgWorker = createActionCreators({
  processSmallImage: (payload: ProcessSmallImagePayload) => payload,
  processImage: (payload: ProcessImagePayload) => payload,
  processAbort: (payload: ProcessAbortPayload) => payload
})

export const fromImgWorker = createActionCreators({
  smallImageProcessed: (payload: SmallImageProcessedPayload) => payload,
  imageProcessed: (payload: ImageProcessedPayload) => payload,
  unauthorized: () => ({}),
  notification: (payload: NotificationReturnParams) => payload
})

export const toDataWorker = createActionCreators({
  fetchData: (payload: FetchDataParams) => payload,
  fetchRow: (payload: FetchRowParams) => payload,
  editTags: (payload: EditTagsParams) => payload,
  editAlbums: (payload: EditAlbumsParams) => payload,
  deleteData: (payload: DeleteDataParams) => payload
})

export const fromDataWorker = createActionCreators({
  returnData: (payload: ReturnDataParams) => payload,
  fetchRowReturn: (payload: FetchRowReturnParams) => payload,
  editTagsReturn: (payload: EditTagsReturnParams) => payload,
  notification: (payload: NotificationReturnParams) => payload,
  unauthorized: () => ({})
})

// ================== Main Thread -> Worker ==================

export interface PostToDataWorker {
  fetchData: (payload: FetchDataParams) => void
  fetchRow: (payload: FetchRowParams) => void
  editTags: (payload: EditTagsParams) => void
  editAlbums: (payload: EditAlbumsParams) => void
  deleteData: (payload: DeleteDataParams) => void
}

export interface PostToImgWorker {
  processSmallImage: (payload: ProcessSmallImagePayload) => void
  processImage: (payload: ProcessImagePayload) => void
  processAbort: (payload: ProcessAbortPayload) => void
}

// ================== Worker -> Main Thread ==================

export interface PostFromDataWorker {
  returnData: (payload: ReturnDataParams) => void
  fetchRowReturn: (payload: FetchRowReturnParams) => void
  editTagsReturn: (payload: EditTagsReturnParams) => void
  notification: (payload: NotificationReturnParams) => void
  unauthorized: () => void
}

export interface PostFromImgWorker {
  smallImageProcessed: (payload: SmallImageProcessedPayload) => void
  imageProcessed: (payload: ImageProcessedPayload) => void
  unauthorized: () => void
  notification: (payload: NotificationReturnParams) => void
}
