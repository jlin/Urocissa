<template>
  <v-list-item
    prepend-icon="mdi-open-in-new"
    value="view-original-file"
    @click="handleClick"
    target="_blank"
  >
    <v-list-item-title class="wrap">{{ 'View Original File' }}</v-list-item-title>
  </v-list-item>
</template>
<script setup lang="ts">
import { storeHashToken } from '@/db/db'
import { useTokenStore } from '@/store/tokenStore'
import { tokenReturnSchema } from '@/type/schemas'
import { IsolationId } from '@/type/types'
import axios from 'axios'

const props = defineProps<{
  src: string
  hash: string
  isolationId: IsolationId
}>()
const tokenStore = useTokenStore(props.isolationId)

async function renewandStoreHashToken() {
  const maybeExpiredHashToken = tokenStore.hashTokenMap.get(props.hash)
  if (maybeExpiredHashToken !== undefined) {
    const timestampToken = tokenStore.timestampToken
    if (typeof timestampToken !== 'string') {
      throw new Error('No timestampToken found in request config')
    }

    const tokenResponse = await axios.post(
      `/post/renew-hash-token`,
      {
        expiredHashToken: maybeExpiredHashToken
      },
      {
        headers: {
          Authorization: `Bearer ${timestampToken}`
        }
      }
    )
    if (tokenResponse.status === 200) {
      const newToken = tokenReturnSchema.parse(tokenResponse.data)
      tokenStore.hashTokenMap.set(props.hash, newToken.token)
      await storeHashToken(props.hash, newToken.token)
    }
  }
}

async function handleClick() {
  try {
    await renewandStoreHashToken()
  } catch (err) {
    console.error('Token renewal failed:', err)
    return
  }
  window.open(props.src, '_blank')
}
</script>
