<script lang="ts">
  import MessageBox from "./MessageBox.svelte";
  import { onMount } from "svelte";
  import {
    connect,
    incomingMessages,
    messageStore,
    selectedRoom,
    sendMessage,
    token_store,
    uname,
  } from "$lib/stores";
  import { toast } from "svelte-french-toast";
  import { host, ip, type JWT } from "$lib";
  onMount(async () => {
    window.onbeforeunload = () => {
      if ($incomingMessages.socket) {
        $incomingMessages.socket.close();
      }
    };
    if ($token_store) {
      connect(new URL("ws://" + ip + "/chat/connect"));
      console.log("Found ", $uname);
    }
  });
  const createRoom = async () => {
    const roomName = prompt("Enter the room name:");
    if (roomName) {
      let res = await fetch(host + `/chat/create/${roomName}/` + $uname, {
        method: "POST",
        headers: { authorization: $token_store },
      });
      if (res.status === 200) {
        messageStore.update((store) => {
          store[roomName] = [];
          return store;
        });
        $selectedRoom = roomName;
      }
    }
  };
  const joinRoom = (room: string) => {
    $selectedRoom = room;
  };
</script>

<div class="chat-container">
  <div class="sidebar">
    {#if $uname}
      <h2>Welcome {$uname}</h2>
    {:else}
      <h2>Loading...</h2>
    {/if}
    <button on:click={createRoom}>Create Room</button>
    <h3>Rooms</h3>
    <ul>
      {#each Object.keys($messageStore) as room}
        <li><button on:click={() => joinRoom(room)}>{room}</button></li>
      {/each}
    </ul>
  </div>
  <MessageBox />
</div>

<style lang="scss">
  .chat-container {
    display: flex;
    height: 90%;
  }
  .sidebar {
    width: 200px;
    background-color: #f0f0f0;
    padding: 20px;
    h2 {
      margin-bottom: 20px;
    }
    button {
      margin-bottom: 10px;
    }
    ul {
      list-style: none;
      padding: 0;
      li {
        margin-bottom: 5px;
        button {
          width: 100%;
          text-align: left;
        }
      }
    }
  }
  .bottombar {
    position: absolute;
    bottom: 0;
    button {
      width: 100%;
    }
  }
</style>
