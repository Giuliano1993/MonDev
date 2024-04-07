<script setup>

import { writeText } from "@tauri-apps/api/clipboard";
import { ref } from "vue";
import { open } from "@tauri-apps/api/dialog";
const props = defineProps({

  text: String,
  id: String,
  title: String
})
defineEmits([])
const justCopied = ref(false);
const copyText = ()=>{
  justCopied.value = true;
  console.log('ciao')

  console.log(props.text)
  writeText(props.text)
  setTimeout(()=>{justCopied.value = false},2000)
}

//const openFile = async ()=>{
//  const openedFile = await open({
//    directory:false,
//    multiple: false,
//  })
//  console.log(openedFile);
//}

</script>

<template>
  <div id="{{ id }}">
    <h3>{{title}}</h3> 
    <div>{{ text }}</div>
    <button @click="copyText">{{ !justCopied ? "Copy" : "Copied"}}</button> 
    <!-- <button @click="openFile">Get from file</button> -->
  </div>
</template>

<style scoped>
</style>
