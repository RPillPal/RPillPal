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
    width: 400px;
    margin: 20px;
    padding: 20px;
    border-radius: 30px;
    background-color: orange;
  }
  .summary-title {
    width: 100%;
    text-align: center;
    font-size: 24px;
  }
  .summary-info {
    display: flex;
    text-align: left;
    align-items: center;
    margin-left: 10px;
    font-size: 24px;
    list-style-type: none;
  }
  .summary-info img {
    width: 50px;
    margin-right: 10px;
  }
</style>

<script>
  import { onMount } from "svelte";
  import { apiData, familyMembers } from "./store.js";
  //$: dosesOnHand = 10;
  function notifyToday(doseToday){
    return doseToday ? "Don't forget to take your medicine today!" : "You have already taken your medicine today!";
  }

  onMount(async () => {
  fetch("")
  .then(response => response.json())
  .then(data => {
		console.log(data);
    apiData.set(data);
  }).catch(error => {
    console.log(error);
    return [];
  });
});
</script>

<div class="parent-container">
  <h1 class="title">RPillPal</h1>

  <!-- <p>You have {dosesOnHand} doses remaining</p> -->

  <div class="summary-container">
    {#each $familyMembers as person}
      <div class="user-summary">
        <h3 class="summary-title">{person.name}</h3>
        <li class="summary-info">{notifyToday(person.doseToday)}</li>
        <li class="summary-info"><img src="./pills-solid.svg">{person.dosagesLeft} Dosages Left</li>
      </div>
    {/each}
  </div>
</div>
