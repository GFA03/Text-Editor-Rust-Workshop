<template>
  <v-app>
    <v-main>
      <TextEditor/>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import TextEditor from './components/TextEditor.vue';
export default defineComponent({
  name: 'App',

  components: {
    TextEditor,
  },

  data () {
    return {
      //
    }
  },
})
</script>
