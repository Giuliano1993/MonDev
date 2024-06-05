<script setup lang="ts">
import Modal from './Modal.vue'
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api";
enum Langs{
  EN = "En",
  IT = "IT"
}
const props = defineProps<{
  content: string|Promise<string>,
  lang: Langs|null
}>()

defineEmits([
  'close'
])

const preSubjText = props.lang == "En" ? "MonDEV - The Developer's Monday" : "MonDEV - Il lunedì dello sviluppatore";
const preRecipientGroup = props.lang == "En" ? 4 : 3;

const campaingnName = ref(props.lang == "En" ? " - EN" : " - IT");
const subject = ref(preSubjText);
const previewText = ref("");
const recipients: Ref<null|Number> = ref(preRecipientGroup);
  const content = ref<string>(typeof props.content == 'string' ? props.content : '')



const createCampagin = ()=>{
  const pars = {
    name: campaingnName.value,
    subject: subject.value,
    previewText: previewText.value,
    list: Number(recipients.value),
    content: content.value
  }
  console.log(pars);
  invoke("create_campaing",pars).then((r)=>{
    console.log(r)
  })
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