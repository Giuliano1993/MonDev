<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";



defineProps({
  name: String,
  label: String
})
defineEmits([])

const keyValue = ref("");

const getSecret = async () =>{
  invoke("get_secret",{key:"openAiSecret"}).then((r)=>{

    console.log(r)
  })
}

const saveSecret = () => {


}


onMounted(()=>{
 getSecret()
})

</script>

<template>
  <div>
    adsad
    <label>
      {{ label }}
    </label>
    <input v-model="keyValue">
  </div>
</template>

<style scoped>
</style>