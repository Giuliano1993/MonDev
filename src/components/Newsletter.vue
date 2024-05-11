<script setup lang="ts">

import {computed, onMounted, ref, Ref} from "vue";
import { invoke } from "@tauri-apps/api";
import { marked } from "marked";
import CopybleText from "./CopybleText.vue";
import  NewsletterModal  from "./NewsletterModal.vue";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const markDownText  = ref("");
const htmlMarkdown = computed(()=>{
  return marked.parse(markDownText.value);
})

const newsletterText = ref(null);
onMounted(()=>{
  if(newsletterText.value){
    newsletterText.value.focus();
  }

})
const englishMarkdown = ref("");
const englishHtml = computed(()=>{

  return marked.parse(englishMarkdown.value);
});
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
const devtoDraft = async ()=>{

  invoke("create_article", {content: englishMarkdown.value}).then((r)=>{
    console.log(r)
  }) 
}
 const showModal = (lang: Langs) =>{
  showCampaignModal.value = true;
  campaingLang.value = lang
 }

</script>

<template>
  <h1>MonDev station</h1>
    <div class="row">
      <div>
        <h3>Italian Markdown</h3> 
        <textarea v-model="markDownText" ref="newsletterText"></textarea>
      </div>
      <CopybleText id="markdownPreview" title="Italian Html" :text="htmlMarkdown"></CopybleText>
      
    <button class="campaignButton" @click="showModal(Langs.IT)">Create campaign</button>
    </div>
    <button @click="translateNewsletter">Translate newsletter</button>
    <div class="row">
      <CopybleText id="translatedMarkdown" title="English Markdown" :text="englishMarkdown"></CopybleText>
      <CopybleText id="translatedHtml" title="English Html" :text="englishHtml"></CopybleText> 
      
      <button @click="showModal(Langs.EN)">Create  campaign</button>
      <button @click="devtoDraft">Prepare Article</button>
    </div>
    <NewsletterModal :lang="campaingLang" v-if="showCampaignModal" @close="showCampaignModal = false" :content="campaingLang == Langs.EN ? englishHtml : htmlMarkdown"></NewsletterModal>
</template>

<style scoped>
#markdownPreview{
      border: 2px solid #1a6936;
    }
</style>