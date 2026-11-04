<script lang="ts">
  import ReportView from "@src/views/Report.svelte";

  let { data } = $props();
</script>

<ReportView result={data.result} />
