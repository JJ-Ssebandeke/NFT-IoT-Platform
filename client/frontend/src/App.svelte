<script lang="ts">
import Header from "./components/Header.svelte";
import Loan from "./components/Loan.svelte";

	
</script>

<Header />
<main class="flex flex-col w-screen justify-center items-center h-1/2">
	<Loan />
	
</main>

<style global lang="postcss">

	 @tailwind base;
	 @tailwind components;
	 @tailwind utilities;
	
</style>