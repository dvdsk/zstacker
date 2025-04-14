# script needs file to start with empty line

BEGIN { RS="[[:space:]]+},"; FS="\n" }
{ 	
	name = $3; 
	gsub(/ +name: '/, "", name); 
	gsub(/',/, "", name);  
	name = toupper(substr(name, 1, 1)) substr(name, 2) 
}
{ id = $4; gsub(/ +ID: /, "", id); gsub(/,/, "", id) }
{ type = $5; gsub(/ +type: /, "", type); gsub(/,/, "", type); gsub(/\./, "::", type) }

# request fields
{ 
	started = 0
	delete req_field_names; delete req_field_types
	for(i=2; i<99; i++) {
		if (length($i) == 0) { break }
		if ($i ~ /request: \[$/) { started = 1; continue }
		if ($i ~ /request: \[{/) { started = 1 }

		if (started) {
			if ($i ~ / ],$/) { break }
			if ($i ~ /response: \[/) { break }
			req_field_names[i] = gensub(/.+{name: '(.+)', .+/, "\\1", "g", $i);
			req_field_types[i] = gensub(/.+ParameterType.(.+)}]?,/, "\\1", "g", $i);
		}
	}
	# for (i in req_field_names) print req_field_types[i]
}

# response fields
{
	started = 0
	rsp_fields = 0
	delete rsp_field_names; delete rsp_field_types
	for(i=2 ; i<99; i++) {
		if (length($i) == 0) { break }
		if ($i ~ /response: \[$/) { started = 1; continue }
		if ($i ~ /response: \[/) { started = 1 }

		if (started) {
			if ($i ~ / ],$/) { break }
			if ($i ~ / },$/) { break }
			rsp_field_names[rsp_fields] = gensub(/.+{name: '(.+)', .+/, "\\1", "g", $i);
			rsp_field_types[rsp_fields] = gensub(/.+ParameterType.(.+)}]?,/, "\\1", "g", $i);
			rsp_fields += 1
		}
	}
}

# fix types
{
	for (i in req_field_types) {
		switch (req_field_types[i]) {
			case /UINT[[:digit:]]+/:
				req_field_types[i] = gensub("UINT", "u", "g", req_field_types[i])
				break
			case "BUFFER":
				req_field_types[i] = "Vec<u8>"
				break
			default:
				print "error can not translate request type: " req_field_types[i]
				print "for field " req_field_names[i]
				exit 
		}
	}
	for (i in rsp_field_types) {
		switch (rsp_field_types[i]) {
		case /UINT[[:digit:]]+/:
			rsp_field_types[i] = gensub("UINT", "u", "g", rsp_field_types[i])
			break
		case "BUFFER":
			rsp_field_types[i] = "Vec<u8>"
			break
		default:
			print "error can not translate response type: " rsp_field_types[i]
			print "for field: " rsp_field_names[i]
			exit 
		}
	}
}

{ 
	if (length(name) > 0) {

        # print struct and cmd impl
		print "struct " name " {";
			for (i in req_field_names) {
				print "\t" req_field_names[i] ": " req_field_types[i]  ","
			}
		print "}"

        # print reply struct if any
		if (rsp_fields == 0) {
			reply = "()"
		} else if (rsp_fields == 1 \
			   && rsp_field_names[0] == "status" \
			   && rsp_field_types[0] == "u8") {
			reply = "Status"
		} else {
			reply_name = name "Reply"
			print "struct " reply_name " {"
			for (i in rsp_field_names) {
				print "\t" rsp_field_names[i] ": " rsp_field_types[i]  ","
			}
			print "}"
			reply = reply_name
		}

        # print command impl
		print "impl Command for " name " {"
		print "\tconst ID: u8 = " id ";"
		print "\tconst TYPE: CommandType = " type ";"
		print "\tconst SUBSYSTEM: Subsystem = Subsystem::App" ";"
		print "\ttype Reply = " reply ";"
		print "}"
	}
}
