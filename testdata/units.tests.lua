
Program {
	Name = "test.failed_test_case.amiga.exe",
	Sources = "test.failed_test_case.amiga.s",
	Config = { "amiga-*-*-default" },
}

Program {
	Name = "test.successful_test_case.amiga.exe",
	Sources = "test.successful_test_case.amiga.s",
	Config = { "amiga-*-*-default" },
}

Program {
	Name = "test.illegal_instruction.amiga.exe",
	Sources = "test.illegal_instruction.amiga.s",
	Config = { "amiga-*-*-default" },
}

Program {
	Name = "test.test_cases.amiga.exe",
	Sources = "test.test_cases.amiga.s",
	Config = { "amiga-*-*-default" },
}

Default "test.failed_test_case.amiga.exe"
Default "test.successful_test_case.amiga.exe"
Default "test.illegal_instruction.amiga.exe"
Default "test.test_cases.amiga.exe"
