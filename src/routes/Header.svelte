<script>
	import Logo from '$lib/images/android-chrome-512x512.png';
</script>

<div class="main">
	<enchanched:img src={Logo} width="64" height="64" alt="NN generated cpu" class="logo" />
	<h1 class="main_title">Simulator of imaginary Harvard-architecture cpu</h1>
</div>

<style>
	.main {
    padding-top: 10px;
    padding-bottom: 10px;
    padding-left: 50px;
    padding-right: 50px;
		display: flex;
		justify-content: center;
		flex-direction: row;
    align-items: center;
	}
  .logo {
    padding-right: 0px;
    padding-left: 0px;
  }
	.main_title {
		font-size: 32px;
		padding-left: 50px;
	}
</style>
