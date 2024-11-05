// ===========================
// Imports
// ===========================
import { thumbHashToDataURL } from 'thumbhash'
import { z } from 'zod'

// ===========================
// Constants
// ===========================
export const batchNumber = 100
export const scrollBarWidth = 50
export const paddingPixel = 4
export const fixedBigRowHeight = 2400
export const layoutBatchNumber = 20

// ===========================
// Classes
// ===========================

/**
 * Represents a database entry with various attributes.
 */
export class DataBase {
  hash: string
  size: number
  width: number
  height: number
  timestamp: number
  thumbhashUrl: string
  phash: number[]
  hammingDistance: number
  randomIndex: number
  ext: string
  exif_vec: { [key: string]: string }
  tag: string[]
  album: string[]
  alias: Alias[]
  filename: string
  ext_type: string
  pending: boolean

  constructor(data: z.infer<typeof DataBaseParse>, timestamp: number) {
    this.hash = data.hash
    this.size = data.size
    this.width = data.width
    this.height = data.height
    this.timestamp = timestamp
    this.thumbhashUrl = thumbHashToDataURL(data.thumbhash)
    this.phash = data.phash
    this.hammingDistance = 0
    this.randomIndex = 0
    this.ext = data.ext
    this.exif_vec = data.exif_vec
    this.tag = data.tag
    this.album = data.album
    this.alias = data.alias
    this.filename = this.alias[0]?.file.split('/').pop() || ''
    this.ext_type = data.ext_type
    this.pending = data.pending
  }

  /**
   * Creates a default instance of DataBase.
   * @returns {DataBase | undefined} A new DataBase instance or undefined if parsing fails.
   */
  static createDefault(): DataBase | undefined {
    const defaultData = {
      DataBase: {
        album: [],
        alias: [{ file: '' }],
        exif_vec: {},
        ext: '',
        ext_type: '',
        hash: '',
        height: 200,
        pending: false,
        phash: [],
        size: 0,
        tag: [],
        thumbhash: [],
        width: 300
      }
    }

    try {
      return new DataBase(DataBaseParse.parse(defaultData), Date.now())
    } catch (error) {
      console.error('Failed to create default DataBase:', error)
      return undefined
    }
  }
}

/**
 * Represents a sub-row containing display elements.
 */
export class SubRow {
  displayElements: DisplayElement[]

  constructor(displayElements: DisplayElement[]) {
    this.displayElements = displayElements
  }
}

/**
 * Represents album data with relevant attributes.
 */
export class Album {
  album_id: string
  album_name: string
  active: boolean
  password: string

  constructor(album_id: string, album_name: string, active: boolean, password: string) {
    this.album_id = album_id
    this.album_name = album_name
    this.active = active
    this.password = password
  }
}

// ===========================
// Types and Interfaces
// ===========================

/**
 * Sorting options.
 */
export type Sorting = 'ascending' | 'descending' | 'random' | 'similar'

/**
 * Represents a sliced data item with an index and DataBase instance.
 */
export interface SlicedDataItem {
  index: number
  data: DataBase
}

/**
 * Represents sliced data with an index and DataBase instance.
 */
export interface SlicedData {
  index: number
  data: DataBase
}

/**
 * Represents prefetching sliced data.
 */
export interface PreFetchSliced {
  timestamp: string
  locateTo?: number
  dataLength: number
}

/**
 * Represents an alias with file information.
 */
export type Alias = z.infer<typeof AliasSchema>

/**
 * Represents tag information.
 */
export type TagInfo = z.infer<typeof tagInfoSchema>

/**
 * Represents scrollbar data.
 */
export type ScrollbarData = z.infer<typeof scrollbarDataSchema>

/**
 * Represents display elements within a row.
 */
export type DisplayElement = z.infer<typeof displayElementSchema>

/**
 * Represents a row within the layout.
 */
export type Row = z.infer<typeof rowSchema>

/**
 * Represents a row with offset information.
 */
export type RowWithOffset = z.infer<typeof rowWithOffsetSchema>

/**
 * Represents prefetch information.
 */
export type Prefetch = z.infer<typeof prefetchSchema>

// ===========================
// Zod Schemas
// ===========================

/**
 * Schema for alias objects.
 */
const AliasSchema = z.object({
  file: z.string(),
  modified: z.number(),
  scan_time: z.number()
})

/**
 * Schema for display elements.
 */
export const displayElementSchema = z.object({
  displayWidth: z.number(),
  displayHeight: z.number(),
  displayTopPixelAccumulated: z.number().optional()
})

/**
 * Schema for rows.
 */
export const rowSchema = z.object({
  start: z.number(),
  end: z.number(),
  rowHeight: z.number().optional().default(fixedBigRowHeight),
  displayElements: z.array(displayElementSchema),
  topPixelAccumulated: z.number().optional(),
  rowIndex: z.number(),
  offset: z.number().optional().default(0)
})

/**
 * Schema for rows with offset.
 */
export const rowWithOffsetSchema = z.object({
  row: rowSchema,
  offset: z.number(),
  windowWidth: z.number()
})

/**
 * Schema for prefetching data.
 */
export const prefetchSchema = z.object({
  timestamp: z.string(),
  dataLength: z.number(),
  locateTo: z.number().nullable()
})

export const DataBaseParse = z.object({
  album: z.array(z.string()),
  alias: z.array(AliasSchema),
  exif_vec: z.record(z.string(), z.string()),
  ext: z.string(),
  ext_type: z.string(),
  hash: z.string(),
  height: z.number(),
  pending: z.boolean(),
  phash: z.array(z.number()),
  size: z.number(),
  tag: z.array(z.string()),
  thumbhash: z.array(z.number()),
  width: z.number()
})
/**
 * Schema for DataBase constructor.
 */
export const DataBaseTimestamp = z.object({
  database: z.object({
    DataBase: z.object({
      album: z.array(z.string()),
      alias: z.array(AliasSchema),
      exif_vec: z.record(z.string(), z.string()),
      ext: z.string(),
      ext_type: z.string(),
      hash: z.string(),
      height: z.number(),
      pending: z.boolean(),
      phash: z.array(z.number()),
      size: z.number(),
      tag: z.array(z.string()),
      thumbhash: z.array(z.number()),
      width: z.number()
    })
  }),
  timestamp: z.number()
})

const ShareSchema = z.object({
  url: z.string().max(64),
  description: z.string(),
  password: z.string().optional(),
  show_metadata: z.boolean(),
  show_download: z.boolean(),
  show_upload: z.boolean(),
  exp: z.number().int().nonnegative()
})

const AlbumSchema = z.object({
  id: z.string().max(64),
  title: z.string().optional(),
  created_time: z.bigint(),
  cover: z.string().max(64).optional(),
  user_defined_metadata: z.record(z.array(z.string())),
  share_list: z.array(ShareSchema),
  tag: z.array(z.string()),
  width: z.number().int().nonnegative(),
  height: z.number().int().nonnegative()
})

const AbstractDataSchema = z.union([
  z.object({ DataBase: DataBaseParse }),
  z.object({ Album: AlbumSchema })
])

/**
 * Schema for database timestamp.
 */
export const databaseTimestampSchema = z.object({
  abstractData: AbstractDataSchema,
  timestamp: z.number()
})

/**
 * Schema for scrollbar data.
 */
export const scrollbarDataSchema = z.object({
  index: z.number(),
  year: z.number(),
  month: z.number()
})

/**
 * Schema for tag information.
 */
export const tagInfoSchema = z.object({
  tag: z.string(),
  number: z.number()
})
