<script lang="ts">
  import TableView from "@src/views/Table.svelte";

  let { data } = $props();
</script>

<TableView rows={data.rows} />
