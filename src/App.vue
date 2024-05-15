<script setup lang="ts">
import {computed, onMounted, ref, Ref} from "vue";

import ConfigModal from "./components/ConfigModal.vue";
import Newsletter from "./components/Newsletter.vue";
import Posts from "./components/Posts.vue";
import Articles from "./components/Articles.vue";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
enum Menu{
  NEWSLETTER = "newsletter",
  ARTICLES = "articles",
  POSTS = "posts"
}

const window: Ref<null|Menu>= ref(null);

const showConfigModal = ref(false);


</script>


<template>
  <div class="container">
    <nav>
      <ul>
        <li><h3>GhostyLab</h3></li>
        <li :class="window === Menu.NEWSLETTER ? 'active' : ''" @click="window = Menu.NEWSLETTER">Newsletter</li>
        <li :class="window === Menu.ARTICLES ? 'active' : ''" @click="window = Menu.ARTICLES">Articles</li>
        <li :class="window === Menu.POSTS ? 'active' : ''" @click="window = Menu.POSTS">Posts</li>
        <li>
          <button @click="showConfigModal = true">Show Configs</button>
        </li>
      </ul>
    </nav>
    <Newsletter v-if="window === Menu.NEWSLETTER"/>
    <Articles v-else-if="window === Menu.ARTICLES"/>
    <Posts v-else-if="window === Menu.POSTS"/>
    <div>
      
    </div>
  </div>

  
  <ConfigModal v-if="showConfigModal" @close="showConfigModal = false"></ConfigModal>  

</template>

<style>
.container{
  textarea{
    border: none;
    outline: none;
    background: transparent;
    color: rgb(224, 224, 224);
    font-size: 16px;
    width: 100%;
    padding: 0 10px;
  }
  nav{
    width: 100%;
    border: 1px solid #1a6936;
    ul{
      display: flex;
      width: 100%;
      gap: 10px;
      align-items: center;
      padding-left: 0;
      li{
        list-style: none;
        cursor: pointer;
        &.active{
          text-decoration: underline #1a6936;
          text-decoration-thickness: 3px;
        }
        &:first-child{
          padding-left: 20px;
        }
        &:last-child{
          flex-grow: 3;
          text-align: right;
          padding-right: 20px;
        }
        h3{
          padding: 0;
          margin: 0;
          margin-right: 30px
        }
      }
    }
  }
  h1{
    color: #1a6936;
  }
  & > div{
    display: flex;
    gap: 10px;
    margin-top: 10px;
    margin-bottom: 10px;
    flex-wrap: wrap;
    & > * {
      flex-basis: calc(50% - 10px);
    }
    & > button{
      flex-basis: 100%;
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

