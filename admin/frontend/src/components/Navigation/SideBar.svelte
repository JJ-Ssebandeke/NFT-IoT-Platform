<script lang="ts">
    import Icon from '@iconify/svelte';

</script>

<div class="fixed flex flex-col top-0 left-0 h-screen w-14 bg-[#020202] bg-opacity-30 text-white shadow-lg gap-y-96 pt-5
 items-center backdrop-blur-lg backdrop-saturate-[180%] ">

    <div class="flex flex-col gap-y-14 border-t-[1px] border-[#ffffff9d] pt-10" >

        <div><a href="#/"><Icon icon="carbon:container-registry" width="25.5" height="25.5" color="#C5F7C2" /></a></div>
        <div><a href="#/Devices" class="visited:text-white"><Icon icon="carbon:iot-connect" width="23.5" height="23.5"  /></a></div>
        <div><a href="#/Register-device" class="visited:text-white"><Icon icon="clarity:file-settings-solid" width="23" height="23" /></a></div>
        <div><Icon icon="carbon:ibm-cloud-pak-watson-aiops" width="23" height="23" /></div>

    </div>
    <div class=" border-t-[1px] border-[#ffffff9d] pt-4"><Icon icon="icon-park-outline:user-positioning"width="25" height="25" /></div>
   

</div>