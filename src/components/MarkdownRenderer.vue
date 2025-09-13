<script setup lang="ts">
import { ref, watch } from 'vue'
import MarkdownIt from 'markdown-it'

const props = defineProps<{
  content: string
}>()

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
})

const compiledMarkdown = ref('')

watch(
  () => props.content,
  (newVal) => {
    compiledMarkdown.value = md.render(newVal)
  },
  { immediate: true }
)
</script>

<template>
  <div class="markdown" v-html="compiledMarkdown"></div>
</template>
