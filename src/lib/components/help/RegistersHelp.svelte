<div class="main">
	<h2>Registers</h2>
	<div class="table">
		<div class="row">
			<div class="cell header name">Name</div>
			<div class="cell header addr">Addr</div>
			<div class="cell header desc">Description</div>
		</div>
		<div class="row">
			<div class="cell name"><h3>PC</h3></div>
			<div class="cell addr">0</div>
			<div class="cell desc">
				<p>Program counter. Internal purpose. Hold the current instruction address.</p>
			</div>
		</div>
		<div class="row">
			<div class="cell name"><h3>IR</h3></div>
			<div class="cell addr">1-4</div>
			<div class="cell desc">
				<p>Instruction Register. Internal purpose. Hold the current instructon fetched code.</p>
			</div>
		</div>
		<div class="row">
			<div class="cell name"><h3>CX</h3></div>
			<div class="cell addr">5</div>
			<div class="cell desc">
				<p>
					Loop counter. Write here number of lopps and then fall in loop in code and at each cycle
					it will be decremented.
				</p>
			</div>
		</div>
		<div class="row">
			<div class="cell name"><h3>JMP</h3></div>
			<div class="cell addr">6</div>
			<div class="cell desc">
				<p>
					JMP register. Internal purpose. If we make a jump in loop or in jump instruction, we tell
					to conveyer to NOT increment Program Counter by setting the register to 1.
				</p>
			</div>
		</div>
	</div>
</div>

<style >
	@import './globalHelp.css' scoped;

	.name {
		flex: 2;
	}
	.addr {
		flex: 2;
	}
	.desc {
		flex: 12;
	}
	h3 {
		font-weight: 800px;
    font-size: 30px;
	}
</style>
