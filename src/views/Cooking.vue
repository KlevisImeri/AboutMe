<script setup lang="ts">
import MarkdownRenderer from '@/components/MarkdownRenderer.vue'
import CookingMarkdown from '@/assets/Cooking.md?raw'
</script>

<template>
  <MarkdownRenderer :content="CookingMarkdown" />
</template>
