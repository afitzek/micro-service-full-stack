<script lang="ts">
  export let api_endpoint;
  let text: string = ""
  let count: number = 0
  const increment = () => {
    count += 1
  }

  const get = () => {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
          // Typical action to be performed when the document is ready:
          console.log(xhttp.responseText);
          text = xhttp.responseText;
        }
    };
    xhttp.open("GET", `${api_endpoint}/api`, true);
    xhttp.send();
  }
  get();
</script>

<svelte:options tag="my-weather-widget"/>

<div>
<button on:click={increment}>
  Weahter widget!!!
</button>

<p class="cool" id="demo">{text}</p>
</div>

<style>
  button {
    font-family: inherit;
    font-size: inherit;
    padding: 1em 2em;
    color: #ff3e00;
    background-color: rgba(255, 62, 0, 0.1);
    border-radius: 2em;
    border: 2px solid rgba(255, 62, 0, 0);
    outline: none;
    width: 200px;
    font-variant-numeric: tabular-nums;
    cursor: pointer;
  }

  button:focus {
    border: 2px solid #ff3e00;
  }

  button:active {
    background-color: rgba(255, 62, 0, 0.2);
  }
</style>
