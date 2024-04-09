<script setup lang="ts">
import {computed, ref, Ref} from "vue";
import { invoke } from "@tauri-apps/api";
import { marked } from "marked";
import CopybleText from "./components/CopybleText.vue";
import SecretSave from "./components/SecretSave.vue";
import  NewsletterModal  from "./components/NewsletterModal.vue";
import ConfigModal from "./components/ConfigModal.vue";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const markDownText  = ref("");
const htmlMarkdown = computed(()=>{
  return marked.parse(markDownText.value);
})


const englishMarkdown = ref("");
const englishHtml = computed(()=>{

  return marked.parse(englishMarkdown.value);
});
const showConfigModal = ref(false);
const showCampaignModal = ref(false);
enum Langs{
  EN = "En",
  IT = "IT"
}
const campaingLang: Ref<null|Langs>= ref(null);

const translateNewsletter = async ()=>{

  invoke("translate",{text:markDownText.value}).then((r)=>{
    console.log(r)
    console.log('tutto fatto');
    englishMarkdown.value = String(r);
  })
 
}

 const showModal = (lang: Langs) =>{
  showCampaignModal.value = true;
  campaingLang.value = lang
 }
</script>


<template>
  <div class="container">
    <h1>MonDev station</h1>
    <div class="row">
      <div>
        <h3>Italian Markdown</h3> 
        <textarea v-model="markDownText"></textarea>
      </div>
      <CopybleText id="markdownPreview" title="Italian Html" :text="htmlMarkdown"></CopybleText>
      
    <button class="campaignButton" @click="showModal(Langs.IT)">Create campaign</button>
    </div>
    <button @click="translateNewsletter">Translate newsletter</button>
    <div class="row">
      <CopybleText id="translatedMarkdown" title="English Markdown" :text="englishMarkdown"></CopybleText>
      <CopybleText id="translatedHtml" title="English Html" :text="englishHtml"></CopybleText> 
      
      <button @click="showModal(Langs.EN)">Create  campaign</button>
    </div>
    <div>
      <button @click="showConfigModal = true">Show Configs</button>
    </div>
  </div>

  <NewsletterModal v-if="showCampaignModal" @close="showCampaignModal = false" :content="campaingLang == Langs.EN ? englishHtml : htmlMarkdown"></NewsletterModal>
  <ConfigModal v-if="showConfigModal" @close="showConfigModal = false"></ConfigModal>  

</template>

<style>
.container{
  h1{
    color: #1a6936;
  }
  & > div{
    display: flex;
    gap: 10px;
    margin-top: 10px;
    margin-bottom: 10px;
    flex-wrap: wrap;
    & > * {
      flex-basis: calc(50% - 10px);
    }
    & > button{
      flex-basis: 100%;
    }
    & > div{

      border: 2px solid #1a6936;
      border-radius: 5px;
    }

  }
}

    #markdownPreview{
      border: 2px solid #1a6936;
    }
</style>

