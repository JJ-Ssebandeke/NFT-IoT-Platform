<script lang="ts">
import SideBar from "./components/Navigation/SideBar.svelte";
import Router from 'svelte-spa-router'
import Network from "./components/Network.svelte";
import Monitoring from "./components/Monitoring.svelte";
import Registration from "./components/Registration.svelte";

let routes = {
	'/': Network,
	'/Devices': Monitoring,
	'/Register-device': Registration
}
	
</script>

<SideBar />  
<main class="w-screen h-scree bg-gradient-to-r from-[#1b1b1b] to-zinc-800">
	<Router {routes} />
	
</main>

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
    @tailwind utilities;
</style>