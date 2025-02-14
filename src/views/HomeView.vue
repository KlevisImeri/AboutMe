<script setup lang="ts">
import { ref } from 'vue';
import MarkdownIt from 'markdown-it';
import ResumeMarkdown from '@/assets/Resume.md?raw';

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
});

const baseUrl = import.meta.env.BASE_URL;

const processedMarkdown = ResumeMarkdown.replace(
  /<img src="([^"]+)"/g,
  (match, imgPath) => {
    const fileName = imgPath.split("/").pop();
    return `<img src="${baseUrl}/${fileName}"`;
  }
);

const compiledMarkdown = ref(md.render(processedMarkdown));
</script>

<template>
  <!-- <div class="dark"> -->
    <div class="markdown" v-html="compiledMarkdown"></div>
  <!-- </div> -->
</template>


<!-- import { ref } from 'vue'; -->
<!-- import MarkdownIt from 'markdown-it'; -->
<!-- import ResumeMarkdown from '@/assets/Resume.md?raw'; // Note the ?raw suffix -->
<!---->
<!-- const md = new MarkdownIt({ -->
<!-- <script setup lang="ts"> -->
<!--   html: true, -->
<!--   linkify: true, -->
<!--   typographer: true, -->
<!---->
<!-- }); -->
<!-- const compiledMarkdown = ref(md.render(ResumeMarkdown)); -->
<!-- </script> -->
<!---->
<!-- <template> -->
<!--   <div class="markdown-body github-theme"> -->
<!--     <div v-html="compiledMarkdown"></div> -->
<!--   </div> -->
<!-- </template> -->
<!---->
<!-- <style> -->
<!-- @import 'github-markdown-css'; -->
<!-- </style> -->
