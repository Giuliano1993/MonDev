<script setup lang="ts">
import Modal from "./Modal.vue";
import SecretSave from "./SecretSave.vue";


defineProps([])
defineEmits(['close'])


</script>

<template>
  <Modal @close="$emit('close')">
    <Suspense>
      <div>
        <SecretSave name="openAiSecret" label="openAi"></SecretSave>
      </div>
      <template #fallback>
          carico
      </template>
    </Suspense>
    <Suspense>
      <div>
        <SecretSave name="brevoApi" label="brevo"></SecretSave>
      </div>
      <template #fallback>
          carico
      </template>
    </Suspense>
  </Modal>
</template>

<style scoped>
div{
  display: flex;
  flex-direction: column;
  margin-top: 15px;
  margin-bottom: 15px;
}
</style>