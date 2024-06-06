<script setup lang="ts">

import {computed, watch, ref, Ref} from "vue";
import { invoke } from "@tauri-apps/api";
import { marked } from "marked";
import CopybleText from "./CopybleText.vue";
import  NewsletterModal  from "./NewsletterModal.vue";
import { useMagicKeys } from '@vueuse/core';
const { current } = useMagicKeys()

const keys = computed(()=>Array.from(current));

watch(keys,(keysNow,keysthen)=>{
  console.log(window.getSelection()?.toString())
  window.getSelection()?.getRangeAt
  console.log(keysNow,keysthen)
  const sel = window.getSelection();
  if(sel){
    var range = sel.getRangeAt(0).cloneRange();
    let  string = range.toString();
    string = `**${string}**`;

    //sel.addRange(range);
        
  }
})
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const markDownText  = ref("");
const htmlMarkdown = computed(()=>{
  return marked(markDownText.value,{async:false}).toString()
  //return marked.parse(markDownText.value);
})

const englishMarkdown = ref("");
const englishHtml = computed(()=>{

  return marked.parse(englishMarkdown.value).toString();
});
const showCampaignModal = ref(false);
enum Langs{
  EN = "En",
  IT = "IT"
}
const campaingLang: Ref<null|Langs>= ref(null);

const translateNewsletter = async ()=>{

  invoke("translate",{text:markDownText.value}).then((r)=>{
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
      <div class="full">
        <h3>Italian Markdown</h3> 
        <textarea autofocus v-model="markDownText" ref="newsletterText" rows="10"></textarea>
      </div>
    </div>


    <div class="row">
      <CopybleText id="markdownPreview" title="Italian Html" :text="htmlMarkdown"></CopybleText>  
      <CopybleText id="translatedMarkdown" title="English Markdown" :text="englishMarkdown"></CopybleText>
      <CopybleText id="translatedHtml" title="English Html" :text="englishHtml"></CopybleText> 
      
    </div>
    <div class="row action">
      <button class="campaignButton" @click="showModal(Langs.IT)">Create Italian campaign</button>
      <button @click="translateNewsletter">Translate newsletter</button>
      <button @click="showModal(Langs.EN)">Create English campaign</button>
      <button @click="devtoDraft">Prepare Article</button>
    </div>
    <NewsletterModal :large="true" :lang="campaingLang" v-if="showCampaignModal" @close="showCampaignModal = false" :content="campaingLang == Langs.EN ? englishHtml : htmlMarkdown"></NewsletterModal>
</template>

<style scoped>
h1{
  margin: 20px auto;
}

#markdownPreview{
      border: 2px solid #1a6936;
}
    
textarea{
  width: 100%;
  padding: 0;
  resize: vertical;
}

</style>