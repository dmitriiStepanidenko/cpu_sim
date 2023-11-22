<script>
	import Logo from '$lib/images/android-chrome-512x512.png?enhanced';
</script>

<div class="main">
	<enhanced:img src={Logo} alt="NN generated cpu" class="logo" sizes="64px 64px"/>
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
    width: 64px;
    height: 64px;
  }
	.main_title {
		font-size: 32px;
		padding-left: 50px;
	}
</style>
