require("./premake/target")

newoption({
	trigger = "target",
	description = "Target host triplet",
})

-- Parse target host
local target = Target:create(_OPTIONS["target"])
if (target.arch == nil) then
	print(
		"You need to provide target with --target option \n" ..
		"Example: --target=lx106-xtensa-unknown-elf"
	)
	os.exit(1)
end

-- Tests enumeration
local tests = {
	"01_simple",
	"02_return"
}

local xtensa = {
	buildoptions = {
		"-mlongcalls",
		"-mtext-section-literals",
		"-nostdlib",
		"-nostartfiles",
		"-Wall",
		"-Wextra",
		"-g3"
	},
	linkoptions = {
		"-Wl,-EL",
		"-Wl,--no-check-sections",
		"-Wl,-static",
		"-nostdlib",
		"-nostartfiles",
		"-T./platform/xtensa/sim.ld",
		"-Wl,--start-group",
			"-lgcc",
			"-lc",
		"-Wl,--end-group",
		"-g3"
	}
}

function targetoptions()
	if (target.arch == "arm") then
		buildoptions({
			"-mlittle-endian",
			"-mcpu=cortex-a9",
			"-nostdlib",
			"-nostartfiles"
		})
	elseif (target.arch == "xtensa") then
		buildoptions(xtensa.buildoptions)
		linkoptions(xtensa.linkoptions)
	else
		print("Target " .. target.arch .. " is not supported")
		os.exit(1)
	end
end

-- Call premake
workspace("xtensa2arm-test")
configurations({ "default" })
gccprefix(target.gccprefix)

function platformproject()
	-- Compile startup code
	project("platform")
	kind("StaticLib")

	targetoptions()

	objdir("./build/obj/platform")
	targetdir("./build/lib")
	targetextension(".a")
	targetname("platform")
	files({
		"./platform/" .. target.arch .. "/*.S",
		"./platform/" .. target.arch .. "/*.c"
	})
end

function testproject(name)
	print("Preparing test: " .. name .. "...")

	project(name)
	kind("ConsoleApp")
	links("platform")

	objdir("./build/obj/" .. name)
	includedirs("./platform/include")

	targetdir("./build/bin/" .. name)
	targetextension(".elf")
	targetname(name)

	files({ "./src/" .. name .. "/*.c" })

	targetoptions()
end

platformproject()

for k, test in pairs(tests) do
	testproject(test)
end
