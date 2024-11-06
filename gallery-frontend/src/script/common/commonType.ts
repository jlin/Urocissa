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
 * Schema for alias objects.
 */
const AliasSchema = z.object({
  file: z.string(),
  modified: z.number(),
  scan_time: z.number()
})

export const DataBaseSchema = z.object({
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
  width: z.number(),
  timestamp: z.number(),
  thumbhashUrl: z.string(), // need initialize
  filename: z.string() // need initialize
})

export type DataBase = z.infer<typeof DataBaseSchema>

export function createDataBase(
  databaseParse: z.infer<typeof DataBaseParse>,
  timestamp: number
): DataBase {
  const database: DataBase = {
    album: databaseParse.album,
    alias: databaseParse.alias,
    exif_vec: databaseParse.exif_vec,
    ext: databaseParse.ext,
    ext_type: databaseParse.ext_type,
    hash: databaseParse.hash,
    height: databaseParse.height,
    pending: databaseParse.pending,
    phash: databaseParse.phash,
    size: databaseParse.size,
    tag: databaseParse.tag,
    thumbhash: databaseParse.thumbhash,
    width: databaseParse.width,
    timestamp: timestamp,
    thumbhashUrl: thumbHashToDataURL(databaseParse.thumbhash),
    filename: databaseParse.alias[0]?.file.split('/').pop() || ''
  }
  return database
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
  data: AbstractData
}

/**
 * Represents sliced data with an index and DataBase instance.
 */
export interface SlicedData {
  index: number
  data: AbstractData
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

/**
 * Represents Album.
 */
export type Album = z.infer<typeof AlbumSchema>

// ===========================
// Zod Schemas
// ===========================

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

const AbstractDataParseSchema = z.union([
  z.object({ DataBase: DataBaseParse }),
  z.object({ Album: AlbumSchema })
])

const AbstractDataSchema = z.object({
  database: DataBaseSchema.optional(),
  album: AlbumSchema.optional()
})

export type AbstractData = z.infer<typeof AbstractDataSchema>

export function createAbstractData(data: DataBase | Album): AbstractData {
  if ('hash' in data) {
    const abstractData: AbstractData = {
      database: data
    }
    return abstractData
  } else {
    const abstractData: AbstractData = {
      album: data
    }
    return abstractData
  }
}

/**
 * Schema for database timestamp.
 */
export const databaseTimestampSchema = z.object({
  abstractData: AbstractDataParseSchema,
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
