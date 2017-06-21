Target = {}
Target.__index = Target

function Target:create(s)
	local t = {}
	setmetatable(t, Target)

	if (s == nil) then
		return t
	end

	t.arch = string.match(s, "^(%a+)-")
	t.gccprefix = s .. "-"
	t.string = s;

	return t
end

return Target
