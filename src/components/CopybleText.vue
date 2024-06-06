<script setup lang="ts">

import { writeText } from "@tauri-apps/api/clipboard";
import { ref } from "vue";
const props = defineProps({

  text: {
    type: String || Promise<string>,
    required: true
  },
  id: String,
  title: String
})
defineEmits([])

const expanded = ref(false);
const justCopied = ref(false);
const copyText = ()=>{
  justCopied.value = true;

  console.log(props.text)
  writeText(props.text)
  setTimeout(()=>{justCopied.value = false},2000)
}

</script>

<template>
  <div :id="id">
    <h3>{{title}}</h3> 
    <button @click="expanded = !expanded">{{ !expanded ? "Expand" : "Collapse"}}</button> 

    <div v-if="expanded">{{ text }}</div>
    <button v-if="expanded" @click="copyText">{{ !justCopied ? "Copy" : "Copied"}}</button> 
    <!-- <button @click="openFile">Get from file</button> -->
  </div>
</template>

<style scoped>
</style>
