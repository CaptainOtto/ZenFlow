<script>
    import TextAndIconLogo from '../../lib/logo/TextAndIconLogo.svelte';
    import TextButton from './../../lib/buttons/TextButton.svelte';
    import {SvelteComponent} from "svelte";

    // Handles getting started button click
    function handleClick() {
    }

</script>

<div class='h-full'>
    <div class='absolute top-1/3 left-1/2 transform -translate-x-1/2 -translate-y-1/3'>
        <TextAndIconLogo/>
        <TextButton name='getting-started' onClick={handleClick} text='Preferences' />
    </div>

    <div class='flex justify-center items-center h-full'>

        <div class='mt-auto flex flex-col justify-center items-center' style='margin-bottom: 100px'>
            <TextButton name='getting-started' onClick={handleClick} text='Getting Started' />
            <div class='flex flex-row justify-center items-center mt-10'>
                <p class='cursor-pointer text-base text-white font-normal mr-5'>Click to learn about</p>
                <p class='cursor-pointer text-base text-white font-bold '>Zen Flow</p>
            </div>
        </div>
    </div>
</div>

