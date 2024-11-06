<template>
    <v-dialog v-model="modalStore.showEditTagsModal" variant='flat' persistent id="edit-tag-overlay">
        <v-card class="mx-auto w-100" max-width="400"  variant="elevated" retain-focus>
            <v-card-title> Edit Tags </v-card-title>
            <v-container>
                <v-combobox v-model="changedTagsArray" chips multiple item-title="label" item-value="value"
                    :items="tagList.filter(tag => !specialTag(tag))" label="Tags" closable-chips></v-combobox>
            </v-container>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="teal-accent-4" variant="outlined" class="ma-2 button button-submit"
                    @click="modalStore.showEditTagsModal = false">
                    Cancel
                </v-btn>
                <v-btn color="teal-accent-4" variant="outlined" class="ma-2 button button-submit" @click="change()"
                    :loading="!tagStore.fetched">
                    Submit
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
/**
 * This modal is used for editing the tag of a single photo on the single photo view page.
 */
import { useModalStore } from '@/store/modalStore';
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useDataStore } from '@/store/dataStore';
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker';
import { useTagStore } from '@/store/tagStore';
const modalStore = useModalStore()
const storeData = useDataStore()
const route = useRoute();
const changedTagsArray = ref<string[]>([]);
const tagStore = useTagStore()
const tagList = computed(() => {
    return tagStore.tags.map(tag => tag.tag);
})

const specialTag = (tag: string): boolean => {
    return tag == '_archived' || tag == '_favorite'
}

onMounted(() => {
    changedTagsArray.value = storeData.data.get(storeData.hashMapData.get(route.params.hash as string)!)!.get_tag().filter(tag => !specialTag(tag))
})

const defaultTags = computed(() => {
    return storeData.data.get(storeData.hashMapData.get(route.params.hash as string)!)!.get_tag()
})

const change = () => {
    const hashArray: number[] = [storeData.hashMapData.get(route.params.hash as string)!];
    const addTagsArrayComputed = changedTagsArray.value.filter(tag => !specialTag(tag) && !defaultTags.value.includes(tag));
    const removeTagsArrayComputed = defaultTags.value.filter(tag => !specialTag(tag) && !changedTagsArray.value.includes(tag));
    editTagsInWorker(hashArray, addTagsArrayComputed, removeTagsArrayComputed)
};
</script>

<style scoped></style>