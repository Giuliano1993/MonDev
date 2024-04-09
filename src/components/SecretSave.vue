<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";



const props = defineProps({
  name: String,
  label: String
})
defineEmits([])

const keyValue = ref("");

const getSecret = async () =>{
  invoke("get_secret",{key:props.name}).then((r)=>{
    console.log(r);
    keyValue.value = String(r);
  })
}

const saveSecret = () => {
  invoke("save_secret",{key:props.name, value: keyValue.value}).then((r)=>{
    console.log(r);
  })

}


onMounted(()=>{
 getSecret()
})

</script>

<template>
  <div>
    <label>
      {{ label }}
    </label>
    <input v-model="keyValue">
    <button @click="saveSecret">Save</button>
  </div>
</template>

<style scoped>
div{
  display: flex;
  flex-direction: column;
  gap: 10px;
  label{
    text-transform: capitalize;
    font-size: large;
    font-weight: bold;
  }
}
</style>