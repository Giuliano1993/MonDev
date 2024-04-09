<script setup lang="ts">
import Modal from './Modal.vue'
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api";
const props = defineProps({
  content: String
})
defineEmits([
  'close'
])
const campaingnName = ref("");
const subject = ref("");
const previewText = ref("");
const recipients: Ref<null|Number> = ref(null);
const content = ref(props.content || "");



const createCampagin = ()=>{

}
</script>

<template>
 <Modal @close="$emit('close')">
    <div>
      <label>Nome Campagna</label>
      <input type="text" v-model="campaingnName">
    </div>
    
    <div>
      <label>Subject</label>
      <input type="text" v-model="subject">
    </div>
    <div>
      <label>Preview Text</label>
      <input type="text" v-model="previewText">
    </div>
    <div>
      <label for="">recipients</label>
      <select v-model="recipients">
        <option disabled>Select An option</option>
        <option value="8">Mondev Test</option>
        <option value="3">Mondev It</option>
        <option value="4">Mondev En</option>
      </select>
    </div>
    <div>
      <label>Content</label>
      <textarea v-model="content"></textarea>
    </div>
    <button @click="createCampagin">Create Campaign</button>
 </Modal>
</template>

<style scoped>
div{
  display: flex;
  flex-direction: column;
  margin-top: 15px;
  margin-bottom: 15px;
}
</style>