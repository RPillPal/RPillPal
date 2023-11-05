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
    background-color: IndianRed;
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
    width: 500px;
    margin: 20px;
    padding: 20px;
    border-radius: 30px;
    background-color: IndianRed;
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
    margin-bottom: 30px;
    font-size: 18px;
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
    padding: 20px;
    padding-bottom: 60px;
    border-radius: 15px;
    font-size: 24px;
    background-color: #fefefe;
    margin: 15% auto; /* 15% from the top and centered */
    border: 1px solid #888;
    width: 60%; /* Could be more or less, depending on screen size */
  }

  .modal-title {
    width: 100%;
    margin: 0 auto;
    margin-bottom: 60px;
    text-align: center;
  }
    
  .close {
    text-align: right;
    color: #aaa;
    margin-right: 15px;
    font-size: 42px;
    font-weight: bold;
  }

  .close:hover,
  .close:focus {
    color: black;
    text-decoration: none;
    cursor: pointer;
  } 
  
  .modal-form {
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: repeat(3, 1fr);
    grid-column-gap: 0px;
    grid-row-gap: 20px; 
    width: 90%;
    margin: 0 auto;
  }
  .form-object {
    display: flex;
    justify-content: left;
    align-items: center;
    width: 100%;
  } 

  .form-object label{
    width: 30%;
  }
  
  .form-object input, select {
    width: 70%;
  }

  .user-field {
    margin-left: 30px;
    width: 70%;
    height: 40px;
    font-size: 20px;
  }
  .submit-user-field {
    margin: 0 auto;
    margin-top: 40px;
    width: 100%;
    height: 40px;
    font-size: 20px;
  }
</style>

<script>
  import { onMount } from "svelte";
  import { apiData, familyMembers } from "./store.js";
  $: modalState = "display: none"
  let selectedPerson;
  
  function findDoseToday(person){
    const lastTaken = new Date(person.prescription[0].lastTaken * 1000);
    let difference = new Date()-lastTaken;
    difference = difference / 1000 / 60 / 60;
    const frequency = person.prescription[0].frequency;
    if (frequency == 0 && difference > 12){
      console.log(person.name + "takes pill");
      return 1;
    }
    else if (frequency == 1 && difference > 24){
      console.log(person.name + "takes pill");
      return 1;
    }
    else if (frequency == 2 && difference > 72){
      console.log(person.name + "takes pill");
      return 1;
    }
    else if (frequency == 3 && difference > 168){
      console.log(person.name + "takes pill");
      return 1;
    }
    console.log(person.name + "can't take pill");
    return 0;
  }

  function notifyToday(doseToday){
    return doseToday ? "Don't forget to take your medicine today!" : "You have already taken your medicine today!";
  }
  
  function formatDate(timestamp){
    const date = new Date(timestamp * 1000); // This would be your date object

    const options = {
      weekday: 'long', // long-format day name
      month: 'long', // long-format month name
      day: '2-digit', // day with leading zero
      hour: 'numeric', // numeric hour
      minute: '2-digit', // minute with leading zero
      hour12: true // 12hr format
    };

    const formattedDate = date.toLocaleString('en-US', options);

    return formattedDate;
  }

  function checkExpiration(timestamp){
    const now = new Date();
    if (timestamp < now){
      return "Your medicine is past expiration, call your doctor."
    }
    else {
      let formattedDate = formatDate(timestamp)
      formattedDate = formattedDate.split(' at ')[0];
      //console.log(formattedDate);
      let response = "Your medicine will expire on " + formattedDate;
      return response;
    }
  }

  onMount(async () => {
  fetch("http://rpillpal.us:5000/fetch", {
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

  function updateInventory()

  async function doPost (person, numPills) {
		const res = await fetch('http://rpillpal.us/update', {
			method: 'POST',
			body: JSON.stringify({
			  "name": person.name,
				"numPills": person.prescription[0].numPills + numPills
			})
		})
		
		const json = await res.json()
		result = JSON.stringify(json)
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
        <li class="summary-info">{notifyToday(findDoseToday(person))}</li>
        <li class="summary-info">You have {person.prescription[0].numPills} Dosages Left</li>
        <li class="summary-info">Last dosage taken: <br/> {formatDate(person.prescription[0].lastTaken)}</li>
        <li class="summary-info">{checkExpiration(person.prescription[0].expiration * 1000)}</li>
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
      <div class="close" on:click={changeModalState}>&times;</div>
      <h3 class="modal-title">Add to Inventory</h3>
      <form class="modal-form" > <!-- on:submit={updateInventory} -->
        <div class="form-object">
          <label for="name">Name:</label>
          <select class="user-field" id="name" name="name">
            {#each $familyMembers as person}
              <option bind:value={selectedPerson}>{person.name}</option>
            {/each}
          </select>
        </div>
        <div class="form-object">
          <label for="prescriptionName">Medicine Name:</label>
          <select class="user-field" id="prescriptionName" name="prescriptionName">
            {#each $familyMembers as person}
              <option bind:value={person.prescription[0].name}>{person.prescription[0].name}</option>
            {/each}
          </select>
        </div>
        <div class="form-object">
          <label for="pillsAdded">Pills Added:</label>
          <input bind:value={} class="user-field" type="number" id="pillsAdded" name="pillsAdded" min="1" max="30">
        </div>
        <div class="form-object">
          <input class="submit-user-field" type="submit" value="Submit">
        </div>
      </form>
  </div>
</div>
<!-- <div class="close" on:click={changeModalState}>&times;</div> -->


