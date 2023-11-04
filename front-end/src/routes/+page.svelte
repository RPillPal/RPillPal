<style>
  :global(body) {
    margin: 0;
    font-family: 'Quicksand', sans-serif;
  }
  .parent-container {
    width: 100vw;
    height: 100vh;
    background-color: white;
  }
  .title {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 64px;
    color: black;
  }
  .summary-container {
    display: flex;
    width: 100%;
    flex-wrap: wrap;
    justify-content: center;
  }
  .user-summary {
    height: 400px;
    width: 300px;
    margin: 20px;
    border-radius: 30px;
    background-color: orange;
  }
  .summary-title {
    width: 100%;
    text-align: center;
    font-size: 24px;
  }
</style>

<script>
  import { onMount } from "svelte";
  import { apiData, familyMembers } from "./store.js";
  //$: dosesOnHand = 10;


</script>

<div class="parent-container">
  <h1 class="title">RPillPal</h1>

  <!-- <p>You have {dosesOnHand} doses remaining</p> -->

  <div class="summary-container">
    {#each $familyMembers as name}
      <div class="user-summary">
        <h3 class="summary-title">{name}</h3>
      </div>
    {/each}
  </div>
</div>
