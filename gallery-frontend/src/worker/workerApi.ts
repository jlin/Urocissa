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
  timestampToken: string
  hashToken: string
}

export interface ProcessImagePayload {
  index: number
  hash: string
  devicePixelRatio: number
  albumId: null | string
  shareId: null | string
  timestampToken: string
  hashToken: string
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

// === Common Payloads ===
export interface NotificationPayload {
  text: string
  color: MessageColor
}

export interface RefreshTimestampTokenPayload {
  timestampToken: string
}

export interface RefreshHashTokenPayload {
  hash: string
  hashToken: string
}

// === To DataWorker Payloads ===
export interface FetchDataPayload {
  fetchMethod: FetchDataMethod
  batch: number
  timestamp: number
  timestampToken: string
}

export interface FetchRowPayload {
  index: number
  timestamp: number
  windowWidth: number
  isLastRow: boolean
  timestampToken: string
}

export interface EditTagsPayload {
  indexSet: Set<number>
  addTagsArray: string[]
  removeTagsArray: string[]
  timestamp: number
}

export interface EditAlbumsPayload {
  indexSet: Set<number>
  addAlbumsArray: string[]
  removeAlbumsArray: string[]
  timestamp: number
}

export interface DeleteDataPayload {
  indexArray: number[]
  timestamp: number
}

// === From DataWorker Payloads ===
export interface ReturnDataPayload {
  batch: number
  slicedDataArray: SlicedData[]
}

export interface FetchRowReturnPayload {
  rowWithOffset: RowWithOffset
  timestamp: number
}

export interface EditTagsReturnPayload {
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
  notification: (payload: NotificationPayload) => payload
})

export const toDataWorker = createActionCreators({
  fetchData: (payload: FetchDataPayload) => payload,
  fetchRow: (payload: FetchRowPayload) => payload,
  editTags: (payload: EditTagsPayload) => payload,
  editAlbums: (payload: EditAlbumsPayload) => payload,
  deleteData: (payload: DeleteDataPayload) => payload
})

export const fromDataWorker = createActionCreators({
  returnData: (payload: ReturnDataPayload) => payload,
  fetchRowReturn: (payload: FetchRowReturnPayload) => payload,
  editTagsReturn: (payload: EditTagsReturnPayload) => payload,
  notification: (payload: NotificationPayload) => payload,
  unauthorized: () => ({}),
  refreshTimestampToken: (payload: RefreshTimestampTokenPayload) => payload,
  refreshHashToken: (payload: RefreshHashTokenPayload) => payload
})

// ================== Main Thread -> Worker ==================

export interface PostToImgWorker {
  processSmallImage: (payload: ProcessSmallImagePayload) => void
  processImage: (payload: ProcessImagePayload) => void
  processAbort: (payload: ProcessAbortPayload) => void
}

export interface PostToDataWorker {
  fetchData: (payload: FetchDataPayload) => void
  fetchRow: (payload: FetchRowPayload) => void
  editTags: (payload: EditTagsPayload) => void
  editAlbums: (payload: EditAlbumsPayload) => void
  deleteData: (payload: DeleteDataPayload) => void
}

// ================== Worker -> Main Thread ==================

export interface PostFromImgWorker {
  smallImageProcessed: (payload: SmallImageProcessedPayload) => void
  imageProcessed: (payload: ImageProcessedPayload) => void
  unauthorized: () => void
  notification: (payload: NotificationPayload) => void
}

export interface PostFromDataWorker {
  returnData: (payload: ReturnDataPayload) => void
  fetchRowReturn: (payload: FetchRowReturnPayload) => void
  editTagsReturn: (payload: EditTagsReturnPayload) => void
  notification: (payload: NotificationPayload) => void
  unauthorized: () => void
  refreshTimestampToken: (payload: RefreshTimestampTokenPayload) => void
  refreshHashToken: (payload: RefreshHashTokenPayload) => void
}
