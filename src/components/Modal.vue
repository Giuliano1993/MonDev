<script setup>

defineProps([])
defineEmits(['close'])


</script>

<template>
  <div class="modal-overlay" >
    <div class="modal-container">
      <div class="modal-body">
        <div @click="$emit('close')" class="close-modal">x</div>
        <slot></slot>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .modal-overlay{
    width: 100vw;
    height: 100vh;
    position: absolute;
    top: 0;
    bottom: 0;
    background-color: rgba(0,0,0,0.5);
  }
  .modal-container{
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    & > div.modal-body{
      background-color: #e2e8f0;
      padding: 30px;
      color: black;
      border-radius: 10px;
      flex-direction: column;
      gap: 10px;
      & > .close-modal{
        text-align: right;
        cursor: pointer;
      }
    }
  }
</style>