export const batchNumber = 100
export const scrollBarWidth = 50
export const paddingPixel = 4
export const fixedBigRowHeight = 2400
export const layoutBatchNumber = 20

export function getSrc(
  hash: string,
  original: boolean,
  ext: string,
  _password: string,
  _customParams: any
) {
  const compressedOrImported = original ? 'imported' : 'compressed'
  return `/object/${compressedOrImported}/${hash.slice(0, 2)}/${hash}.${ext}`
}
