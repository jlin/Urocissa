<template>
  <v-dialog
    v-model="modalStore.showBatchEditTagsModal"
    variant="flat"
    persistent
    id="batch-edit-tag-overlay"
  >
    <v-card class="mx-auto w-100" max-width="400" variant="elevated">
      <v-card-title class="text-h5"> Edit Tags </v-card-title>
      <v-form v-model="formIsValid" @submit.prevent="submit" validate-on="input">
        <v-container>
          <v-combobox
            v-model="addTagsArray"
            chips
            multiple
            label="Add Tags"
            :rules="[addTagsRule]"
            :items="tagList.filter((tag) => !specialTag(tag))"
          ></v-combobox>
        </v-container>
        <v-container>
          <v-combobox
            v-model="removeTagsArray"
            chips
            multiple
            label="Remove Tags"
            :rules="[removeTagsRule]"
            :items="tagList.filter((tag) => !specialTag(tag))"
          ></v-combobox>
        </v-container>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="teal-accent-4"
            variant="outlined"
            class="ma-2 button button-submit"
            @click="modalStore.showBatchEditTagsModal = false"
          >
            Cancel
          </v-btn>
          <v-btn
            color="teal-accent-4"
            variant="outlined"
            class="ma-2 button button-submit"
            :disabled="!formIsValid"
            type="submit"
          >
            Submit
          </v-btn>
        </v-card-actions>
      </v-form>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
/**
 * This modal is used for editing the tags of multiple photos on the home page.
 */
import { useModalStore } from '@/store/modalStore'
import { computed, ref } from 'vue'
import { useCollectionStore } from '@/store/collectionStore'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { useTagStore } from '@/store/tagStore'
const formIsValid = ref(false)
const modalStore = useModalStore('')
const addTagsArray = ref<string[]>([])
const removeTagsArray = ref<string[]>([])
const collectionStore = useCollectionStore('')
const tagStore = useTagStore('')
const tagList = computed(() => {
  return tagStore.tags.map((tag) => tag.tag)
})

const specialTag = (tag: string): boolean => {
  return tag == '_archived' || tag == '_favorite'
}

// Rule for Add Tags to ensure no tag is added that's already in Remove Tags
const addTagsRule = (inputArray: string[]) =>
  inputArray.every((tag) => !removeTagsArray.value.includes(tag)) ||
  'Some tags are already selected in Remove Tags'

// Rule for Remove Tags to ensure no tag is added that's already in Add Tags
const removeTagsRule = (inputArray: string[]) =>
  inputArray.every((tag) => !addTagsArray.value.includes(tag)) ||
  'Some tags are already selected in Add Tags'

const submit = () => {
  const hashArray = Array.from(collectionStore.editModeCollection)
  editTagsInWorker(hashArray, addTagsArray.value, removeTagsArray.value)
  modalStore.showBatchEditTagsModal = false
}
</script>

<style scoped></style>
