<script lang="ts">
  import WordsView from "@src/views/Words.svelte";

  let { data } = $props();
</script>

<WordsView result={data.result} />
