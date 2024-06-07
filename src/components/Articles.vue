<script setup lang="ts">
import { ref, Ref } from 'vue';
import { invoke } from '@tauri-apps/api';


defineProps([])
defineEmits([])
type TextError = {
  location: String,
  errorType: String,
  error: String,
  correction:String
}
const articleText = ref("");
const corrections: Ref<null|Array<TextError>> = ref(null);

const getCorrections = function(){
  invoke("get_correction",{text:articleText.value}).then((res: any)=>{

    console.log(res.choices[0].message.content)
    console.log(JSON.parse(res.choices[0].message.content))
    corrections.value = JSON.parse(res.choices[0].message.content).errors
  })
}
</script>

<template>
  <textarea v-model="articleText" rows="10"></textarea>
  <button @click="getCorrections">Get Corrections</button>
  <table>
    <thead>
      <tr>
        <th>location</th>
        <th>Error type</th>
        <th>Error</th>
        <th>Correction</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="c in corrections">
        <td>{{ c.location }}</td>
        <td>{{ c.errorType }}</td>
        <td>{{ c.error }}</td>
        <td>{{ c.correction }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
  textarea{
    width: 100%;
  }
</style>