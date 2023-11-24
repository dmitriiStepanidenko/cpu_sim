<script>
</script>

<div class="main">
	<h2>Command memory</h2>
	<h3>Possible commands</h3>
	<div class="table">
		<div class="row">
			<div class="cell header name">Command name</div>
			<div class="cell header code">Command code (4 bits)</div>
			<div class="cell header protype">Prototype</div>
			<div class="cell header desc">Description</div>
		</div>
		<div class="row">
			<div class="cell name">Load</div>
			<div class="cell code">0</div>
			<div class="cell protype">Load(RegisterAddress, AddressingMode)</div>
			<div class="cell desc">LOAD reg, addr_mode - Load data from memory at add_mode into reg</div>
		</div>
		<div class="row">
			<div class="cell name">Store</div>
			<div class="cell code">1</div>
			<div class="cell protype">Store(AddressingMode, RegisterAddress)</div>
			<div class="cell desc">
				STORE addr_mode, reg - store data from reg into memory at addr_mode.
			</div>
		</div>
		<div class="row">
			<div class="cell name">Add</div>
			<div class="cell code">2</div>
			<div class="cell protype">Add(RegisterAddress, AddressingMode, AddressingMode)</div>
			<div class="cell desc">
				ADD Rdest, Rsrc1, Rsrc2 - add values from Rsrc1 and Rsrc2, store result in Rdest.
			</div>
		</div>
		<div class="row">
			<div class="cell name">Mul</div>
			<div class="cell code">3</div>
			<div class="cell protype">Mul(RegisterAddress, AddressingMode, AddressingMode)</div>
			<div class="cell desc">
				MUL Rdest, Rsrc1, Rsrc2 - multiply values from Rsrc1 and Rsrc2, store result in Rdest.
			</div>
		</div>
		<div class="row">
			<div class="cell name">Mov</div>
			<div class="cell code">4</div>
			<div class="cell protype">Mov(RegisterAddress, AddressingMode)</div>
			<div class="cell desc">MOV reg1, addr_mode - copy data from addr_mode to reg1.</div>
		</div>
		<div class="row">
			<div class="cell name">Inc</div>
			<div class="cell code">6</div>
			<div class="cell protype">Inc(AddressingMode)</div>
			<div class="cell desc">INC addr_mode - increment the value at addr_mode by one.</div>
		</div>
		<div class="row">
			<div class="cell name">Loop</div>
			<div class="cell code">7</div>
			<div class="cell protype">Loop(u8)</div>
			<div class="cell desc">
				LOOP addr - decrement loop counter (CX) by one, if not zero, jump to addr.
			</div>
		</div>
		<div class="row">
			<div class="cell name">Jmp</div>
			<div class="cell code">8</div>
			<div class="cell protype">Jmp(AddressingMode)</div>
			<div class="cell desc">JMP addr_mode - unconditional jump to addr_mode.</div>
		</div>
	</div>

	<h3>Register address format</h3>
	<div class="table">
		<div class="row">
			<div class="cell header register_name">Register</div>
			<div class="cell header register_value">Value (in memory addr)</div>
		</div>
		<div class="row">
			<div class="cell register_name">PC</div>
			<div class="cell register_value">0</div>
		</div>
		<div class="row">
			<div class="cell register_name">IR</div>
			<div class="cell register_value">from 1 to 4</div>
		</div>
		<div class="row">
			<div class="cell register_name">CX</div>
			<div class="cell register_value">5</div>
		</div>
		<div class="row">
			<div class="cell register_name">JMP</div>
			<div class="cell register_value">6</div>
		</div>
		<div class="row">
			<div class="cell register_name">GP(u8)</div>
			<div class="cell register_value">from 7 to (u8::MAX -1)</div>
		</div>
	</div>

	<h3>Addressing Mode</h3>
	<div class="table">
		<div class="row">
			<div class="cell header addressing_mode_type">Addressing mode type</div>
			<div class="cell header addressing_mode_code">Addressing mode code (2 bits)</div>
			<div class="cell header addressing_mode_prototype">Addressing mode protype</div>
			<div class="cell header addressing_mode_desc">Addressing mode description</div>
		</div>
		<div class="row">
			<div class="cell addressing_mode_type">Immediate</div>
			<div class="cell addressing_mode_code">0</div>
			<div class="cell addressing_mode_prototype">Immediate(u8)</div>
			<div class="cell addressing_mode_desc">
				Immediate addressing: MOV R0, 10 - place value 10 to R10
			</div>
		</div>
		<div class="row">
			<div class="cell addressing_mode_type">Direct</div>
			<div class="cell addressing_mode_code">1</div>
			<div class="cell addressing_mode_prototype">Direct(u8)</div>
			<div class="cell addressing_mode_desc">
				Direct addressing: MOV R0, [123] - place value from memory by address 123 to R0
			</div>
		</div>
		<div class="row">
			<div class="cell addressing_mode_type">Indirect</div>
			<div class="cell addressing_mode_code">2</div>
			<div class="cell addressing_mode_prototype">Indirect(RegisterAddress)</div>
			<div class="cell addressing_mode_desc">
				Indirect addressing: MOV R0, R1 - place value from R1 to R0
			</div>
		</div>
		<!--<div class="row">
			<div class="cell addressing_mode_type">Register</div>
			<div class="cell addressing_mode_prototype">Register(RegisterAddress)</div>
			<div class="cell addressing_mode_desc"></div>
    </div>-->
	</div>
</div>

<style>
	@import './globalHelp.css' scoped;

	.addressing_mode_type {
		flex: 2;
	}
	.addressing_mode_code {
		flex: 2;
	}
	.addressing_mode_prototype {
		flex: 4;
	}
	.addressing_mode_desc {
		flex: 10;
	}

	.register_name {
		flex: 2;
	}
	.register_value {
		flex: 3;
	}

	.name {
		flex: 2;
	}
	.code {
		flex: 2;
	}
	.protype {
		flex: 6;
	}
	.desc {
		flex: 10;
	}
</style>
