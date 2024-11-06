// schemas.ts

import { z } from 'zod';
import { fixedBigRowHeight } from './constants';

/**
 * Schema for alias objects.
 */
export const AliasSchema = z.object({
  file: z.string(),
  modified: z.number(),
  scan_time: z.number(),
});

/**
 * Schema for display elements.
 */
export const displayElementSchema = z.object({
  displayWidth: z.number(),
  displayHeight: z.number(),
  displayTopPixelAccumulated: z.number().optional(),
});

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
  offset: z.number().optional().default(0),
});

/**
 * Schema for rows with offset.
 */
export const rowWithOffsetSchema = z.object({
  row: rowSchema,
  offset: z.number(),
  windowWidth: z.number(),
});

/**
 * Schema for prefetching data.
 */
export const prefetchSchema = z.object({
  timestamp: z.string(),
  dataLength: z.number(),
  locateTo: z.number().nullable(),
});

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
  width: z.number(),
});

/**
 * Schema for DataBase with additional fields.
 */
export const DataBaseSchema = DataBaseParse.extend({
  timestamp: z.number(),
  thumbhashUrl: z.string(), // need initialize
  filename: z.string(),     // need initialize
});

/**
 * Schema for database timestamp.
 */
export const DataBaseTimestamp = z.object({
  database: z.object({
    DataBase: DataBaseParse,
  }),
  timestamp: z.number(),
});

/**
 * Schema for share information.
 */
export const ShareSchema = z.object({
  url: z.string().max(64),
  description: z.string(),
  password: z.string().optional(),
  show_metadata: z.boolean(),
  show_download: z.boolean(),
  show_upload: z.boolean(),
  exp: z.number().int().nonnegative(),
});

/**
 * Schema for album.
 */
export const AlbumSchema = z.object({
  id: z.string().max(64),
  title: z.string().optional(),
  created_time: z.bigint(),
  cover: z.string().max(64).optional(),
  user_defined_metadata: z.record(z.array(z.string())),
  share_list: z.array(ShareSchema),
  tag: z.array(z.string()),
  width: z.number().int().nonnegative(),
  height: z.number().int().nonnegative(),
});

export const AbstractDataParseSchema = z.union([
  z.object({ DataBase: DataBaseParse }),
  z.object({ Album: AlbumSchema }),
]);

export const AbstractDataSchema = z.object({
  database: DataBaseSchema.optional(),
  album: AlbumSchema.optional(),
});

/**
 * Schema for scrollbar data.
 */
export const scrollbarDataSchema = z.object({
  index: z.number(),
  year: z.number(),
  month: z.number(),
});

/**
 * Schema for tag information.
 */
export const tagInfoSchema = z.object({
  tag: z.string(),
  number: z.number(),
});

/**
 * Schema for database timestamp.
 */
export const databaseTimestampSchema = z.object({
  abstractData: AbstractDataParseSchema,
  timestamp: z.number(),
});
