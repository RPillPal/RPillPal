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
  .user-menu {
    position: fixed;
    bottom: 20px;
    right: 30px;
    z-index: 99;
  }
  .menu-button {
    width: 75px;
    height: 75px;
    border-radius: 75px;
    background: none;
    background-color: orange;
    border: none;
    color: inherit;
    font: inherit;
    outline: inherit;
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

  .modal {
    display: none; /* Hidden by default */
    position: fixed; /* Stay in place */
    z-index: 1; /* Sit on top */
    left: 0;
    top: 0;
    width: 100%; /* Full width */
    height: 100%; /* Full height */
    overflow: auto; /* Enable scroll if needed */
    background-color: rgb(0,0,0); /* Fallback color */
    background-color: rgba(0,0,0,0.4); /* Black w/ opacity */
  }

  .modal-content {
    font-size: 24px;
    background-color: #fefefe;
    margin: 15% auto; /* 15% from the top and centered */
    padding: 20px;
    border: 1px solid #888;
    width: 60%; /* Could be more or less, depending on screen size */
  }

  .close {
    color: #aaa;
    float: right;
    font-size: 28px;
    font-weight: bold;
  }

  .close:hover,
  .close:focus {
    color: black;
    text-decoration: none;
    cursor: pointer;
  } 
  
  .modal-form {
    width: 90%;
    margin: 0 auto;
  }
    
  .user-field {
    margin-left: 30px;
    width: 70%;
    height: 50px;
    font-size: 24px;
  }
</style>

<script>
  import { onMount } from "svelte";
  import { apiData, familyMembers } from "./store.js";
  $: modalState = "display: none"
  function notifyToday(doseToday){
    return doseToday ? "Don't forget to take your medicine today!" : "You have already taken your medicine today!";
  }

  onMount(async () => {
  fetch("http://127.0.0.1:5000/fetch", {
    method: 'GET',
    headers: {
        'Content-Type': "application/json"
      }
  })
  .then(response => response.json())
  .then(data => {
		console.log(data);
    apiData.set(data);
  }).catch(error => {
    console.log(error);
    return [];
    });
  });
  
  function changeModalState(){
    console.log("changed modal state")
    if (modalState == "display: none"){
      modalState = "display: block";
    } 
    else{
      modalState = "display: none";
    }
  }
  </script>

<div class="parent-container">
  <h1 class="title">
    <img style="width: 100px; margin-right: 50px; transform: scale(-1, 1);" alt="Pill Icon" src="./pills-solid.svg">
    RPillPal
    <img style="width: 100px; margin-left: 50px;" alt="Pill Icon" src="./pills-solid.svg">
  </h1>
  <p class="title" style="font-size: 24px"> Manage your family's prescriptions safely.</p>

  <!-- <p>You have {dosesOnHand} doses remaining</p> -->

  <div class="summary-container">
    {#each $familyMembers as person}
      <div class="user-summary">
        <h3 class="summary-title">{person.name}</h3>
        <li class="summary-info">{notifyToday(person.pin)}</li>
        <li class="summary-info"><img alt="Pill Icon" src="./pills-solid.svg">{person.prescription} Dosages Left</li>
      </div>
    {/each}
  </div>
  <div class="user-menu">
    <button id="menu-button" class="menu-button" on:click={changeModalState}>
      <img alt="User Menu" src="./pencil-solid.svg" style="width: 50px">
    </button>
  </div>
</div>

<div class="modal" style={modalState}>

  <div class="modal-content">
    <span class="close" on:click{changeModalState}>&times;</span>
      <form class="modal-form">
        <label for="name">Name:</label>
        <select class="user-field" id="name" name="name">
        {#each $familyMembers as person}
          <option value={person.name}>{person.name}</option>
        {/each}
      </form>
  </div>

</div>
