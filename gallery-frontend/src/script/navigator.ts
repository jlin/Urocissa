import { RouteLocationNormalizedLoadedGeneric } from 'vue-router'

export function leaveRead(route: RouteLocationNormalizedLoadedGeneric) {
  return {
    name: `${route.meta.baseName}ViewPage`,
    params: {
      hash: route.params.hash
    },
    query: route.query
  }
}

export function intoViewPage(route: RouteLocationNormalizedLoadedGeneric, hashOrSubhash: string) {
  if (!route.meta.isReadPage) {
    return {
      name: `${route.meta.baseName}ViewPage`,
      params: { hash: hashOrSubhash },
      query: route.query
    }
  } else {
    return {
      name: `${route.meta.baseName}ReadViewPage`,
      params: { hash: route.meta.hash as string, subhash: hashOrSubhash },
      query: route.query
    }
  }
}

export function leavePage(route: RouteLocationNormalizedLoadedGeneric) {
  console.log('trigger')

  if (!route.meta.isViewPage && route.meta.isReadPage) {
    return {
      name: `${route.meta.baseName}ViewPage`,
      params: {
        hash: route.params.hash
      },
      query: route.query
    }
  }
  if (route.meta.isViewPage && !route.meta.isReadPage) {
    return { name: route.meta.baseName, query: route.query }
  } else if (route.meta.isViewPage && route.meta.isReadPage) {
    return {
      name: `${route.meta.baseName}ReadPage`,
      params: {
        hash: route.params.hash
      },
      query: route.query
    }
  }
}
