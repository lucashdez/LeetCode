local function scan()
	local i, t, popen = 0, {}, io.popen
	local pfile = popen("dir . /b /ad")
	for filename in pfile:lines() do
		t[i] = filename
		i = i + 1
	end
	pfile:close()
	return t
end

local function main()
	local table = scan()
	for k, name in pairs(table) do
		print(k .. ":" .. name)
		print(k .. " (ENTER):")
		os.execute("cd " .. name .. " & rmdir /s /q target")
		print(k .. " (DELETED): /target...")
		print(k .. "(LEAVE): ")
	end
end

main()
