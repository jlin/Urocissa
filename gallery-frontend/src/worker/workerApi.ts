// Import necessary types from the commonType module
import { Prefetch, RowWithOffset, ScrollbarData, SlicedData, TagInfo } from '@/script/common/types'

// Import createActionCreators from typesafe-agent-events for defining strongly-typed actions
import { createActionCreators } from 'typesafe-agent-events'

/**
 * Typesafe-agent-events is a library that helps create strongly-typed event-driven architectures.
 * It provides a utility called `createActionCreators` to define actions with type-safe payloads.
 * The resulting action creators can be used to send or receive messages between threads or workers.
 */

/* ================================================================================
 * toImgWorker
 * Actions to be sent to the image processing worker.
 * These actions are used to request image processing tasks.
 * ================================================================================ */

// Define payload types for image processing actions
export type processSmallImagePayload = {
  index: number
  hash: string
  width: number
  height: number
  devicePixelRatio: number
  jwt: string
  albumMode?: boolean
}

export type processImagePayload = {
  index: number
  hash: string
  devicePixelRatio: number
  jwt: string
}

export type processAbortPayload = {
  index: number
}

// Define actions to be sent to the image processing worker
export const toImgWorker = createActionCreators({
  processSmallImage: (payload: processSmallImagePayload) => payload,
  processImage: (payload: processImagePayload) => payload,
  processAbort: (payload: processAbortPayload) => payload
})

/* ================================================================================
 * fromImgWorker
 * Actions to be received from the image processing worker.
 * These actions are used to handle responses from the worker after processing images.
 * ================================================================================ */

// Define actions to be received from the image processing worker
export const fromImgWorker = createActionCreators({
  smallImageProcessed: (payload: { index: number; url: string }) => payload,
  imageProcessed: (payload: { index: number; url: string }) => payload,
  unauthorized: (payload: {}) => payload
})

/* ================================================================================
 * toDataWorker
 * Actions to be sent to the worker for various data and UI management tasks.
 * These actions request tasks like data fetching, resizing, and editing.
 * ================================================================================ */

// Define the parameter types for different actions to manage data and UI state
type FetchDataParams = { batch: number; timestamp: string }
type FetchRowParams = { index: number; timestamp: string; windowWidth: number; isLastRow: boolean }
type PrefetchParams = {
  filterJsonString: string | null
  priorityId: string
  reverse: string | undefined
  locate: string | null
}
type EditTagsParams = {
  indexArray: number[]
  addTagsArray: string[]
  removeTagsArray: string[]
  timestamp: string
}
type EditAlbumsParams = {
  idArray: number[]
  addAlbumsArray: string[]
  removeAlbumsArray: string[]
  timestamp: string
}
type DeleteDataParams = { indexArray: number[]; timestamp: string }
type FetchScrollBarParams = { timestamp: string }

// Define actions for the worker to receive and execute tasks
export const toDataWorker = createActionCreators({
  fetchData: (payload: FetchDataParams) => payload,
  fetchRow: (payload: FetchRowParams) => payload,
  prefetch: (payload: PrefetchParams) => payload,
  editTags: (payload: EditTagsParams) => payload,
  editAlbums: (payload: EditAlbumsParams) => payload,
  deleteData: (payload: DeleteDataParams) => payload,
  fetchScrollbar: (payload: FetchScrollBarParams) => payload
})

/* ================================================================================
 * fromDataWorker
 * Actions to be received from the worker after completing tasks.
 * These actions handle responses, data updates, or state changes.
 * ================================================================================ */

// Define the parameter types for responses from the worker back to the main thread
type ReturnDataParams = { batch: number; slicedDataArray: SlicedData[] }
type FetchRowReturnParams = { rowWithOffset: RowWithOffset; timestamp: string }
type PrefetchReturnParams = { result: Prefetch }
type EditTagsReturnParams = {
  returnedTagsArray: TagInfo[] | undefined
}
type FetchScrollBarReturnParams = { scrollbarDataArray: ScrollbarData[] }
type NotificationReturnParams = { message: string; messageType: 'info' | 'warn' }

// Define actions for the worker to send responses back to the main thread
export const fromDataWorker = createActionCreators({
  returnData: (payload: ReturnDataParams) => payload,
  fetchRowReturn: (payload: FetchRowReturnParams) => payload,
  prefetchReturn: (payload: PrefetchReturnParams) => payload,
  editTagsReturn: (payload: EditTagsReturnParams) => payload,
  fetchScrollbarReturn: (payload: FetchScrollBarReturnParams) => payload,
  notification: (payload: NotificationReturnParams) => payload,
  unauthorized: () => ({})
})
