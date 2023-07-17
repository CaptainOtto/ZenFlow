<script lang='ts'>

    export let text: string = 'Button';
    export let onClick: (() => void) | undefined;
    export let name: string | undefined;

    function handleClick() {
      if (typeof onClick === 'function') {
        onClick();
      }

      console.debug(`Button ${name} clicked`);
    }

  </script>
  
  <button class='bg-gradient-to-t from-orange-700 to-orange-200 text-white font-bold py-2 px-10 rounded-full 
      hover:from-orange-500 hover:to-orange-200 
      active:from-orange-700 active:to-orange-400
    '
    on:click={handleClick}>
    {text}
  </button>
  
  
  