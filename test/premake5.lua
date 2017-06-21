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

-- Call premake
workspace("xtensa2arm-test")
configurations({ "default" })
gccprefix(target.gccprefix)

function testproject(name)
	print("Preparing test: " .. name .. "...")

	project(name)
	kind("ConsoleApp")

	objdir("./build/obj/" .. name)
	includedirs("./platform/include")

	targetdir("./build/bin/" .. name)
	targetextension(".elf")
	targetname(name)

	files({ "./src/" .. name .. "/*.c" })

	if (target.arch == "arm") then
		buildoptions({
			"-mlittle-endian",
			"-mcpu=cortex-a9",
			"-nostdlib",
			"-nostartfiles"
		})
	elseif (target.arch == "xtensa") then
		buildoptions({
			"-mlongcalls",
			"-mtext-section-literals",
			"-Wall",
			"-Wl,-EL",
			"-nostdlib",
			"-nostartfiles"
		})
	else
		print("Target " .. target.arch .. " is not supported")
		os.exit(1)
	end
end

for k, test in pairs(tests) do
	testproject(test)
end
