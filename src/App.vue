<script setup lang="ts">
import {computed, ref, Ref} from "vue";
import { invoke } from "@tauri-apps/api";
import { marked } from "marked";
import CopybleText from "./components/CopybleText.vue";
import SecretSave from "./components/SecretSave.vue";
import  NewsletterModal  from "./components/NewsletterModal.vue";
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
const testCampagin = async ()=>{
  invoke("create_brevo_campaign").then((r)=>{
    console.log(r);
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
    <div>
      <div>
        <h3>Italian Markdown</h3> 
        <textarea v-model="markDownText"></textarea>
      </div>
      <CopybleText id="markdownPreview" title="Italian Html" :text="htmlMarkdown"></CopybleText>
      
    <button @click="showModal(Langs.IT)">Create test campaign</button>
    </div>
    <button @click="translateNewsletter">Translate newsletter</button>
    <div>
      <CopybleText id="translatedMarkdown" title="English Markdown" :text="englishMarkdown"></CopybleText>
      <CopybleText id="translatedHtml" title="English Html" :text="englishHtml"></CopybleText> 
      
      <button @click="showModal(Langs.EN)">Create test campaign</button>
    </div>
    <div>
      <Suspense>
        <SecretSave name="openAiSecret" label="openAi"></SecretSave>
        <template #fallback>
            carico
        </template>
      </Suspense>
      <Suspense>
        <SecretSave name="brevoApi" label="brevo"></SecretSave>
        <template #fallback>
            carico
        </template>
      </Suspense>
    </div>
  </div>

  <NewsletterModal v-if="showCampaignModal" @close="showCampaignModal = false" :content="campaingLang == Langs.EN ? englishHtml : htmlMarkdown"></NewsletterModal>
  

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
    & > * {
      flex-basis: 50%;
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

