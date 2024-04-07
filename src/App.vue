<script setup lang="ts">
import {computed, ref} from "vue";
import { invoke } from "@tauri-apps/api";
import { marked } from "marked";
import CopybleText from "./components/CopybleText.vue";
import SecretSave from "./components/SecretSave.vue";

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
const translateNewsletter = async ()=>{

  invoke("translate",{text:markDownText.value}).then((r)=>{
    console.log(r)
    console.log('tutto fatto');
    englishMarkdown.value = String(r);
  })
 
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
      
    </div>
    <button @click="translateNewsletter">Translate newsletter</button>
    <div>
      <CopybleText id="translatedMarkdown" title="English Markdown" :text="englishMarkdown"></CopybleText>
      <CopybleText id="translatedHtml" title="English Html" :text="englishHtml"></CopybleText> 
      <Suspense>

        <SecretSave label="openAi" name="openAi"></SecretSave>
      <template #fallback>
          carico
      </template>
      </Suspense>
    </div>
  </div>

  

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

